// SPDX-License-Identifier: Apache-2.0
//! The Vyges engines as two binaries, each running the engine it is invoked as.
//!
//! `vyges-loom` carries the analysis and sign-off engines, `vyges-physical` the physical
//! construction engines. The installer links every engine's name to its binary
//! (`vyges-sta-si -> vyges-loom`), and the binary dispatches on the name it was run as:
//!
//! - `vyges-<engine> …` runs that engine, with its arguments as given;
//! - `vyges-loom <engine> …` / `vyges-physical <engine> …` runs it too (re-executed under the
//!   engine's name, so the engine sees the argument list it always has);
//! - `vyges-loom` with anything else is the loom front end itself; `vyges-physical` with anything
//!   else lists its engines.
//!
//! An engine's code is unchanged: each crate builds its `main` into its library behind its
//! `suite` feature, and the table here only names it.

use std::process::{ExitCode, Termination};

/// One engine: its name (`vyges-<name>` is what it runs as) and its entry.
pub struct Engine {
    pub name: &'static str,
    pub main: fn() -> ExitCode,
}

/// An engine's `main`, whatever it returns, as an exit code.
pub fn report<T: Termination>(t: T) -> ExitCode {
    t.report()
}

/// The table entries: `engine!("sta-si", vyges_sta_si)`.
#[macro_export]
macro_rules! engine {
    ($name:literal, $krate:ident) => {
        $crate::Engine { name: $name, main: || $crate::report($krate::cli_main::main()) }
    };
}

/// The name a binary was run as: argv[0]'s file name, without an `.exe`.
fn invoked_as() -> String {
    let arg0 = std::env::args_os().next().unwrap_or_default();
    let name = std::path::Path::new(&arg0).file_name().map(|n| n.to_string_lossy().into_owned()).unwrap_or_default();
    name.strip_suffix(".exe").map(str::to_string).unwrap_or(name)
}

/// Run as `own` (`vyges-loom`, `vyges-physical`) over `engines`; `fallback` is what `own` does
/// with no engine named.
pub fn run(own: &str, version: &str, engines: &[Engine], fallback: fn() -> ExitCode) -> ExitCode {
    let name = invoked_as();
    // Run as its own name, the binary reads its own options first: `vyges-loom` is also the loom
    // engine's name, and that engine is the fallback.
    if name != own {
        if let Some(e) = name.strip_prefix("vyges-").and_then(|n| engines.iter().find(|e| e.name == n)) {
            return (e.main)();
        }
    }
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("--suite-engines") => {
            for e in engines {
                println!("{}", e.name);
            }
            ExitCode::SUCCESS
        }
        Some("--suite-version") => {
            println!("{own} {version}");
            ExitCode::SUCCESS
        }
        Some(sub) if engines.iter().any(|e| e.name == sub && format!("vyges-{sub}") != own) => reexec_as(sub, &args[1..]),
        _ => fallback(),
    }
}

/// `vyges-physical pdn …`: run this binary again as `vyges-pdn …`.
#[cfg(unix)]
fn reexec_as(engine: &str, rest: &[String]) -> ExitCode {
    use std::os::unix::process::CommandExt;
    let exe = match std::env::current_exe() {
        Ok(p) => p,
        Err(e) => {
            eprintln!("vyges: cannot find this binary to run {engine}: {e}");
            return ExitCode::from(126);
        }
    };
    let err = std::process::Command::new(exe).arg0(format!("vyges-{engine}")).args(rest).exec();
    eprintln!("vyges: cannot run {engine}: {err}");
    ExitCode::from(126)
}

#[cfg(not(unix))]
fn reexec_as(engine: &str, _rest: &[String]) -> ExitCode {
    eprintln!("vyges: run `vyges-{engine}` directly on this platform");
    ExitCode::from(2)
}

/// `vyges-physical` run as itself with no engine: its engines, one per line, and how to run one.
pub fn list_engines(own: &str, version: &str, engines: &[Engine]) -> ExitCode {
    println!("{own} {version}");
    println!();
    println!("USAGE:");
    println!("  {own} <engine> [args...]     (or run it as vyges-<engine>)");
    println!();
    println!("ENGINES:");
    for e in engines {
        println!("  {}", e.name);
    }
    ExitCode::SUCCESS
}

// SPDX-License-Identifier: Apache-2.0
//! `vyges-loom`: the analysis and sign-off engines, and the loom front end itself.

use std::process::ExitCode;

const ENGINES: &[vyges_suite::Engine] = &[
    vyges_suite::engine!("loom", vyges_loom),
    vyges_suite::engine!("opendb", vyges_opendb),
    vyges_suite::engine!("char", vyges_char),
    vyges_suite::engine!("sta-si", vyges_sta_si),
    vyges_suite::engine!("extract", vyges_extract),
    vyges_suite::engine!("em-ir", vyges_em_ir),
    vyges_suite::engine!("power", vyges_power),
    vyges_suite::engine!("thermal", vyges_thermal),
    vyges_suite::engine!("lvs", vyges_lvs),
    vyges_suite::engine!("resize", vyges_resize),
    vyges_suite::engine!("vt-swap", vyges_vt_swap),
    vyges_suite::engine!("buffer-insert", vyges_buffer_insert),
    vyges_suite::engine!("hold-fix", vyges_hold_fix),
    vyges_suite::engine!("drc", vyges_drc),
    vyges_suite::engine!("cdc", vyges_cdc),
    vyges_suite::engine!("glitch", vyges_glitch),
    vyges_suite::engine!("gds-view", vyges_gds_view),
    vyges_suite::engine!("remap", vyges_remap),
    vyges_suite::engine!("meas", vyges_meas),
    vyges_suite::engine!("ant", vyges_ant),
    vyges_suite::engine!("stt", vyges_stt),
    vyges_suite::engine!("est", vyges_est),
];

fn main() -> ExitCode {
    vyges_suite::run("vyges-loom", env!("CARGO_PKG_VERSION"), ENGINES, || vyges_suite::report(vyges_loom::cli_main::main()))
}

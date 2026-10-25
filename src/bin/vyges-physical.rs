// SPDX-License-Identifier: Apache-2.0
//! `vyges-physical`: the physical construction engines.

use std::process::ExitCode;

const ENGINES: &[vyges_suite::Engine] = &[
    vyges_suite::engine!("lec", vyges_lec),
    vyges_suite::engine!("mpl", vyges_mpl),
    vyges_suite::engine!("ifp", vyges_ifp),
    vyges_suite::engine!("tap", vyges_tap),
    vyges_suite::engine!("pdn", vyges_pdn),
    vyges_suite::engine!("ppl", vyges_ppl),
    vyges_suite::engine!("pad", vyges_pad),
    vyges_suite::engine!("fin", vyges_fin),
    vyges_suite::engine!("gpl", vyges_gpl),
    vyges_suite::engine!("rsz", vyges_rsz),
    vyges_suite::engine!("dpl", vyges_dpl),
    vyges_suite::engine!("cts", vyges_cts),
    vyges_suite::engine!("grt", vyges_grt),
    vyges_suite::engine!("drt", vyges_drt),
];

fn main() -> ExitCode {
    vyges_suite::run("vyges-physical", env!("CARGO_PKG_VERSION"), ENGINES, || vyges_suite::list_engines("vyges-physical", env!("CARGO_PKG_VERSION"), ENGINES))
}

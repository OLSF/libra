//! Main entry point for MinerApp

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use onboard::application::APPLICATION;

/// Boot MinerApp
fn main() {
    abscissa_core::boot(&APPLICATION);
}

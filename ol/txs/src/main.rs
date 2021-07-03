//! Main entry point for TxsApp

#![deny(warnings, missing_docs, trivial_casts, unused_qualifications)]
#![forbid(unsafe_code)]

use txs::application::APPLICATION;

/// Boot TxsApp
fn main() {
    abscissa_core::boot(&APPLICATION);
}

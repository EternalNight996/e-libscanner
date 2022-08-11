//! A fast network scan tools of library
//! # Quick start
//! ```
//! fn main() -> Result<(), String> {
//! #[cfg(feature = "sync")]
//! {
//!     use e_libscanner::sync_scan;
//!     use e_libscanner::Opts;
//!     use std::thread;
//!     // more command information use: -h
//!     let mut scanner = Opts::new(Some(&[
//!         "e-libscanner",
//!         "--ips",
//!         "192.168.1.0/24",
//!         "192.168.2-3.1-10",
//!         "baidu.com",
//!         "--model",
//!         "sync",
//!         "--scan",
//!         "Icmp",
//!         "--no-gui",
//!         "--",
//!         "-AS",
//!     ]))?
//!    .init()?
//!     .downcast::<sync_scan::Scanner>()
//!     .unwrap();
//!     let rx = scanner.get_progress_receiver();
//!     // Run scan
//!     let handle = thread::spawn(move || scanner.scan(None));
//!     // Print progress
//!     while let Ok(socket_addr) = rx.lock().unwrap().recv() {
//!         println!("Check: {}", socket_addr);
//!     }
//!     let result = handle.join().unwrap();
//!     // Print results
//!     println!("Status: {:?}", result.scan_status);
//!     println!("UP Hosts:");
//!     let len = result.ips.len();
//!     for host in result.ips {
//!         println!("{:?}", host);
//!     }
//!     println!("Scan Time: {:?} count[ {} ]", result.scan_time, len);
//! }
//! Ok(())
//! }
//! ```

#![doc(
    html_logo_url = "https://www.rust-lang.org/logos/rust-logo-128x128-blk.png",
    html_favicon_url = "https://www.rust-lang.org/favicon.ico",
    html_root_url = "https://github.com/EternalNight996",
)]
#![warn(
    missing_debug_implementations,
    missing_doc_code_examples,
    missing_docs,
    rust_2018_idioms,
    unreachable_pub,
    bad_style,
    const_err,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true
)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![cfg_attr(docsrs, feature(docsrs))]
// Rustc lints.
#![deny(missing_docs, unused_imports)]

/// Async scan api
#[cfg(feature = "async")]
#[cfg_attr(docsrs, doc(cfg(feature = "async")))]
pub mod async_scan;

/// Sync scan api
#[cfg(feature = "sync")]
#[cfg_attr(docsrs, doc(cfg(feature = "sync")))]
pub mod sync_scan;

/// Service scan api
#[cfg(feature = "service")]
#[cfg_attr(docsrs, doc(cfg(feature = "service")))]
pub mod service;

/// Os scan api
#[cfg(feature = "os")]
#[cfg_attr(docsrs, doc(cfg(feature = "os")))]
pub mod os;

/// Static data model
pub mod data;
/// Struct model
pub mod frame;
/// Network interface api
pub mod interface;
/// Network packet model
pub mod packet;
/// Network utils
mod utils;
/// Network utils
pub use utils::*;
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
    html_root_url = "https://github.com/EternalNight996"
)]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![doc(test(attr(allow(unused_variables), deny(warnings))))]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

/// Async scan api
#[cfg(feature = "async")]
pub mod async_scan;

/// Sync scan api
#[cfg(feature = "sync")]
pub mod sync_scan;

/// Service scan api
#[cfg(feature = "service")]
pub mod service;

/// Os scan api
#[cfg(feature = "os")]
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
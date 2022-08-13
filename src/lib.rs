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
    rustdoc::missing_doc_code_examples,
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
#![cfg_attr(doc_cfg, feature(doc_cfg))]
// Rustc lints.
#![deny(missing_docs, unused_imports)]


/// Async Host Scanner
/// # Example
/// ```
/// fn main() -> Result<(), String> {
/// #[cfg(feature = "async")]
/// {
///     use e_libscanner::{async_scan, Opts};
///     use std::thread;
///     // more command information use: -h
///     let mut scanner = Opts::new(Some(&[
///         "e-libscanner",
///         "--ips",
///         "192.168.20.0/23",
///         "192.168.28-31.1-10",
///         "baidu.com",
///         "--model",
///         "async",
///         "--scan",
///         "icmp",
///         "--no-gui",
///     ]))?
///     .init()?
///     .downcast::<async_scan::Scanner>()
///    .unwrap();
///     let rx = scanner.get_progress_receiver();
///     // Run scan
///     let handle = thread::spawn(move || async_io::block_on(async { scanner.scan(None).await }));
///     // Print progress
///     while let Ok(socket_addr) = rx.lock().unwrap().recv() {
///         println!("Check: {}", socket_addr);
///     }
///     let result = handle.join().unwrap();
///     // Print results
///     println!("Status: {:?}", result.scan_status);
///     println!("UP Hosts:");
///     let len = result.ips.len();
///     for host in result.ips {
///         println!("{:?}", host);
///     }
///     println!("Scan Time: {:?} count[ {} ]", result.scan_time, len);
/// }
/// Ok(())
/// }
/// ```
#[cfg(feature = "async")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "async")))]
pub mod async_scan;

/// Host Scanner
/// # Examples
/// ```
/// {
///     use e_libscanner::sync_scan;
///     use e_libscanner::Opts;
///     use std::thread;
///     // more command information use: -h
///     let mut scanner = Opts::new(Some(&[
///         "e-libscanner",
///         "--ips",
///         "192.168.1.0/24",
///         "192.168.2-3.1-10",
///         "baidu.com",
///         "--model",
///         "sync",
///         "--scan",
///         "Icmp",
///         "--no-gui",
///         "--",
///         "-AS",
///     ]))?
///     .init()?
///     .downcast::<sync_scan::Scanner>()
///     .unwrap();
///     let rx = scanner.get_progress_receiver();
///     // Run scan
///     let handle = thread::spawn(move || scanner.scan(None));
///     // Print progress
///     while let Ok(socket_addr) = rx.lock().unwrap().recv() {
///         println!("Check: {}", socket_addr);
///     }
///     let result = handle.join().unwrap();
///     // Print results
///     println!("Status: {:?}", result.scan_status);
///     println!("UP Hosts:");
///     let len = result.ips.len();
///     for host in result.ips {
///         println!("{:?}", host);
///     }
///     println!("Scan Time: {:?} count[ {} ]", result.scan_time, len);
/// }
/// Ok(())
/// }
/// ```
#[cfg(feature = "sync")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "sync")))]
pub mod sync_scan;

/// Struct for service detection
/// # Example
/// ```
/// fn main() -> Result<(), String> {
/// #[cfg(feature = "service")]
/// {
///     use e_libscanner::service::{self, PortDatabase, ServiceDetector};
///     use e_libscanner::Opts;
///     use std::thread;
///     // more command information use: -h
///     let mut scanner = Opts::new(Some(&[
///         "e-libscanner",
///         "--ips",
///         "192.168.80.10",
///         "--ports",
///         "8000",
///         "8080",
///         "20-100",
///         "--rate",
///         "1",
///         "--model",
///         "service",
///         "--scan",
///         "tcpsyn",
///         "--no-gui",
///     ]))?
///     .init()?
///     .downcast::<service::Scanner>()
///     .unwrap();
///     let rx = scanner.get_progress_receiver();
///     let time = std::time::Instant::now();
///     // Run scan
///     let handle = thread::spawn(move || scanner.scan(None));
///     // Print progress
///     while let Ok(socket_addr) = rx.lock().unwrap().recv() {
///         println!("Check: {}", socket_addr);
///     }
///     let result = handle.join().unwrap();
///     for (ip, _ports) in result.ip_with_port.clone() {
///         let mut service_detector = ServiceDetector::new();
///         service_detector.set_dst_ip(ip);
///         service_detector.set_open_ports(result.get_open_ports(ip));
///         println!("{}", service_detector.scan(Some(PortDatabase::default())));
///     }
///     println!("time -> {}/s", time.elapsed().as_secs_f64());
/// }
/// Ok(())
/// }
/// ```
#[cfg(feature = "service")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "service")))]
pub mod service;

/// Struct for fingerprint probe
/// # Example
/// ```
/// fn main() -> Result<(), String> {
/// #[cfg(feature = "os")]
/// {
///     use e_libscanner::os;
///     use e_libscanner::Opts;
///     // more command information use: -h
///     let mut scanner = Opts::new(Some(&[
///         "e-libscanner",
///         "--ips",
///         "192.168.80.8",
///         "192.168.80.1",
///         "--ports",
///         "80",
///         "135",
///         "554",
///         "8000",
///         "22",
///         "--model",
///         "os",
///         "--no-gui",
///         "--",
///         "-AS",
///     ]))?
///     .init()?
///     .downcast::<os::Scanner>()
///     .unwrap();
///     let results = scanner.scan(None);
///     for result in results {
///         println!("{}", result.ip_addr);
///         println!("{:?}", result.icmp_echo_result);
///         println!("{:?}", result.icmp_timestamp_result);
///         println!("{:?}", result.icmp_address_mask_result);
///         println!("{:?}", result.icmp_information_result);
///         println!("{:?}", result.icmp_unreachable_ip_result);
///         println!("{:?}", result.icmp_unreachable_data_result);
///         println!("{:?}", result.tcp_syn_ack_result);
///         println!("{:?}", result.tcp_rst_ack_result);
///         println!("{:?}", result.tcp_ecn_result);
///         println!("{:?}", result.tcp_header_result);
///         println!();
///     }
///     Ok(())
/// }
/// }
/// ```
#[cfg(feature = "os")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "os")))]
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
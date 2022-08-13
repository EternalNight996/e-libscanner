mod cmd_input;
/// DNS results
/// # Examples
/// ```
/// use e_libscanner::{dns::DnsResults, Opts};
/// fn main() -> Result<(), String> {
///     // more command information use: -h
///     let opts = Opts::new(Some(&[
///         "e-libscanner",
///         "--ips",
///         "baidu.com",
///         "127.0.0.1",
///         "localhost",
///         "--model",
///        "dns",
///     ]))?
///     .init()?
///     .downcast::<DnsResults>();
///     match opts {
///         Ok(opt) => {
///             let mut n = 0i32;
///             for r in *opt {
///                 n += 1;
///                 eprintln!("{}- src[ {} ] parse [{:?}]", n, r.src, r.result);
///             }
///         }
///         Err(e) => panic!("{:?}", e),
///     }
///     Ok(())
/// }
/// ```
pub mod dns;
/// Traceroute model
/// # Examples
/// ```
/// use e_libscanner::{traceroute::Tracert, Opts};
/// fn main() -> Result<(), String> {
///     let opts = Opts::new(Some(&[
///         "e-libscanner",
///         "--ips",
///         "114.114.114.114",
///         "--model",
///         "traceroute",
///     ]))?
///     .init()?
///     .downcast::<Tracert>();
///     match opts {
///         Ok(opt) => {
///             let prx = opt.get_progress_receiver();
///             let handle = std::thread::spawn(move || {
///                 while let Ok(msg) = prx.lock().unwrap().recv() {
///                     // TODO Something
///                     eprintln!("recv {:?}", msg);
///                 }
///             });
///             let results = opt.scan(None);
///             handle.join().unwrap();
///             println!("count result -> {}", results.len());
///         }
///         Err(e) => panic!("{:?}", e),
///     }
///     Ok(())
/// }
/// ```
pub mod traceroute;
pub use cmd_input::{parse_ip_range, Opts, ScriptsRequired, ScanModelType, ScanOrderType};

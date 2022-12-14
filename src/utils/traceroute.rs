use std::{
    fmt,
    net::IpAddr,
    sync::{
        mpsc::{self, Receiver, Sender},
        Arc, Mutex,
    },
    thread,
    time::Duration,
};
use e_utils::traceroute::Traceroute;

/// Traceroute reuslt model
#[derive(Clone, Debug)]
pub struct TracertQueryResult {
    #[doc(hidden)]
    pub id: u8,
    /// Round-Trip Time
    pub rtt: Duration,
    /// IP address of a remote node
    pub addr: Vec<Vec<String>>,
}
impl fmt::Display for TracertQueryResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[id[{}] rtt[{}] addr[{:?}]]",
            self.id,
            self.rtt.as_millis(),
            self.addr
        )
    }
}

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
#[derive(Debug)]
pub struct Tracert {
    iface_ip: Option<IpAddr>,
    target: Vec<String>,
    sender: Arc<Mutex<Sender<TracertQueryResult>>>,
    receiver: Arc<Mutex<Receiver<TracertQueryResult>>>,
}
impl Tracert {
    /// targets; iface_ip: network interface ip;  
    pub fn new(target: Vec<String>, iface_ip: Option<IpAddr>) -> Self {
        let (tx, rx) = mpsc::channel();
        Tracert {
            iface_ip,
            target,
            sender: Arc::new(Mutex::new(tx)),
            receiver: Arc::new(Mutex::new(rx)),
        }
    }
    /// Get target length
    pub fn len(&self) -> usize {
        self.target.len()
    }
    /// Get receiver to get trace route of result
    pub fn get_progress_receiver(&self) -> Arc<Mutex<Receiver<TracertQueryResult>>> {
        self.receiver.clone()
    }
    /// Running scan to trace route
    pub fn scan(&self, pstop: Option<Arc<Mutex<bool>>>) -> Vec<TracertQueryResult> {
        let mut handles = vec![];
        let mut results = vec![];
        let stop = if let Some(p) = pstop {
            p
        } else {
            Arc::new(Mutex::new(false))
        };
        for t in self.target.clone() {
            let sender_cp = Arc::clone(&self.sender);
            let iface_ip_cp = self.iface_ip.clone();
            let stop_cp = Arc::clone(&stop);
            handles.push(thread::spawn(move || {
                let tracert = Traceroute::new(t.clone(), iface_ip_cp).unwrap();
                let mut tracert_results = vec![];
                let mut state = false;
                for hop in tracert {
                    if state {
                        break;
                    } else {
                        let mut result = TracertQueryResult {
                            id: hop.ttl,
                            rtt: Duration::default(),
                            addr: vec![],
                        };
                        for query_result in hop.query_result {
                            if query_result.rtt.as_millis() > result.rtt.as_micros() {
                                result.rtt = query_result.rtt;
                            }
                            result.addr.push(query_result.addr);
                            if *stop_cp.lock().unwrap() {
                                state = true;
                                break;
                            }
                        }
                        sender_cp.lock().unwrap().send(result.clone()).unwrap();
                        tracert_results.push(result);
                    }
                }
                tracert_results
            }));
        }
        for handle in handles {
            results.append(&mut handle.join().unwrap());
        }
        results
    }
}

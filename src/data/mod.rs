/// Const key value
pub mod id;
mod portmap;
use std::collections::HashMap;

use once_cell::sync::Lazy;

/// The know port of name in hash
#[derive(Debug)] 
pub struct Data {
    /// The know port of name in hash
    pub portmap: HashMap<u16, &'static str>,
}
/// Static data api 
pub static DATA: Lazy<Data> = Lazy::new(|| Data {
    portmap: portmap::get_tcp_portmap(),
});

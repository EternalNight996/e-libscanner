use std::fmt;

#[doc(hidden)]
pub type DnsResults = Vec<DnsResult>;
/// Dns result model
#[derive(Debug, Clone)]
pub struct DnsResult {
    #[doc(hidden)]
    pub src: String,
    #[doc(hidden)]
    pub result: DnsResultType,
}
impl fmt::Display for DnsResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[src_ip[{}] {}]", self.src, self.result)
    }
}
/// Dns result type
#[derive(Debug, Clone)]
pub enum DnsResultType {
    /// Host like Dns name
    Host(String),
    /// Addr like ip
    Addr(Vec<std::net::IpAddr>),
    /// Return Error
    Error(String),
}
impl fmt::Display for DnsResultType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let result = match self {
            DnsResultType::Host(data) => format!("Host[{}]",data),
            DnsResultType::Addr(data) => format!("Addr[{:?}]", data),
            DnsResultType::Error(e) => format!("Err[{}]",e),
        };
        write!(f, "{}", result)
    }
}
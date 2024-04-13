/// Simple class representing a Proxy configuration.
///
/// An instance of this class can be used through `ConnectionOptions.setProxy()` to instruct
/// a `LightstreamerClient` to connect to the Lightstreamer Server passing through a proxy.
///
/// # Parameters
///
/// * `proxy_type`: the proxy type
/// * `host`: the proxy host
/// * `port`: the proxy port
/// * `user`: the user name to be used to validate against the proxy. Optional.
/// * `password`: the password to be used to validate against the proxy. Optional.
#[derive(Debug)]
pub struct Proxy {
    proxy_type: ProxyType,
    host: String,
    port: u16,
    user: Option<String>,
    password: Option<String>,
}

impl Proxy {
    /// Creates a new instance of `Proxy`.
    ///
    /// # Parameters
    ///
    /// * `proxy_type`: the proxy type
    /// * `host`: the proxy host
    /// * `port`: the proxy port
    /// * `user`: the user name to be used to validate against the proxy. Optional.
    /// * `password`: the password to be used to validate against the proxy. Optional.
    pub fn new(
        proxy_type: ProxyType,
        host: String,
        port: u16,
        user: Option<String>,
        password: Option<String>,
    ) -> Self {
        Proxy {
            proxy_type,
            host,
            port,
            user,
            password,
        }
    }

    /// Returns the proxy type.
    pub fn get_proxy_type(&self) -> &ProxyType {
        &self.proxy_type
    }

    /// Returns the proxy host.
    pub fn get_host(&self) -> &str {
        &self.host
    }

    /// Returns the proxy port.
    pub fn get_port(&self) -> u16 {
        self.port
    }

    /// Returns the proxy user name.
    pub fn get_user(&self) -> Option<&String> {
        self.user.as_ref()
    }

    /// Returns the proxy password.
    pub fn get_password(&self) -> Option<&String> {
        self.password.as_ref()
    }
}

/// Represents the type of proxy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProxyType {
    /// HTTP proxy.
    Http,
    /// SOCKS4 proxy.
    Socks4,
    /// SOCKS5 proxy.
    Socks5,
}

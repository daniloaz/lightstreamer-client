use crate::proxy::Proxy;

/// Used by LightstreamerClient to provide an extra connection properties data object.
/// Data struct that contains the policy settings used to connect to a Lightstreamer Server.
/// An instance of this struct is attached to every LightstreamerClient as connection_options.
pub struct ConnectionOptions {
    content_length: Option<usize>,
    first_retry_max_delay: Option<u64>,
    forced_transport: Option<String>,
    http_extra_headers: Option<Vec<(String, String)>>,
    http_extra_headers_on_session_creation_only: Option<bool>,
    idle_timeout: Option<u64>,
    keepalive_interval: Option<u64>,
    polling_interval: Option<u64>,
    proxy: Option<Proxy>,
    real_max_bandwidth: Option<u64>,
    reconnect_timeout: Option<u64>,
    requested_max_bandwidth: Option<u64>,
    retry_delay: Option<u64>,
    reverse_heartbeat_interval: Option<u64>,
    server_instance_address_ignored: Option<bool>,
    session_recovery_timeout: Option<u64>,
    slowing_enabled: Option<bool>,
    stalled_timeout: Option<u64>,
}

impl ConnectionOptions {
    /// Creates a new ConnectionOptions object with default values.
    pub fn new() -> ConnectionOptions {
        ConnectionOptions::default()
    }

    /// Sets the content length.
    pub fn set_content_length(&mut self, content_length: usize) {
        self.content_length = Some(content_length);
    }

    /// Sets the first retry max delay.
    pub fn set_first_retry_max_delay(&mut self, first_retry_max_delay: u64) {
        self.first_retry_max_delay = Some(first_retry_max_delay);
    }

    /// Sets the forced transport.
    pub fn set_forced_transport(&mut self, forced_transport: String) {
        self.forced_transport = Some(forced_transport);
    }

    /// Sets the HTTP extra headers.
    pub fn set_http_extra_headers(&mut self, http_extra_headers: Vec<(String, String)>) {
        self.http_extra_headers = Some(http_extra_headers);
    }

    /// Sets the HTTP extra headers on session creation only.
    pub fn set_http_extra_headers_on_session_creation_only(&mut self, http_extra_headers_on_session_creation_only: bool) {
        self.http_extra_headers_on_session_creation_only = Some(http_extra_headers_on_session_creation_only);
    }

    /// Sets the idle timeout.
    pub fn set_idle_timeout(&mut self, idle_timeout: u64) {
        self.idle_timeout = Some(idle_timeout);
    }

    /// Sets the keepalive interval.
    pub fn set_keepalive_interval(&mut self, keepalive_interval: u64) {
        self.keepalive_interval = Some(keepalive_interval);
    }

    /// Sets the polling interval.
    pub fn set_polling_interval(&mut self, polling_interval: u64) {
        self.polling_interval = Some(polling_interval);
    }

    /// Sets the proxy.
    pub fn set_proxy(&mut self, proxy: Proxy) {
        self.proxy = Some(proxy);
    }

    /// Sets the real max bandwidth.
    pub fn set_real_max_bandwidth(&mut self, real_max_bandwidth: u64) {
        self.real_max_bandwidth = Some(real_max_bandwidth);
    }

    /// Sets the reconnect timeout.
    pub fn set_reconnect_timeout(&mut self, reconnect_timeout: u64) {
        self.reconnect_timeout = Some(reconnect_timeout);
    }

    /// Sets the requested max bandwidth.
    pub fn set_requested_max_bandwidth(&mut self, requested: u64) {
        self.requested_max_bandwidth = Some(requested);
    }

    /// Sets the retry delay.
    pub fn set_retry_delay(&mut self, retry_delay: u64) {
        self.retry_delay = Some(retry_delay);
    }

    /// Sets the reverse heartbeat interval.
    pub fn set_reverse_heartbeat_interval(&mut self, reverse_heartbeat_interval: u64) {
        self.reverse_heartbeat_interval = Some(reverse_heartbeat_interval);
    }

    /// Sets the server instance address ignored.
    pub fn set_server_instance_address_ignored(&mut self, server_instance_address_ignored: bool) {
        self.server_instance_address_ignored = Some(server_instance_address_ignored);
    }

    /// Sets the session recovery timeout.
    pub fn set_session_recovery_timeout(&mut self, session_recovery_timeout: u64) {
        self.session_recovery_timeout = Some(session_recovery_timeout);
    }

    /// Sets the slowing enabled.
    pub fn set_slowing_enabled(&mut self, slowing_enabled: bool) {
        self.slowing_enabled = Some(slowing_enabled);
    }

    /// Sets the stalled timeout.
    pub fn set_stalled_timeout(&mut self, stalled_timeout: u64) {
        self.stalled_timeout = Some(stalled_timeout);
    }
}

impl Default for ConnectionOptions {
    fn default() -> Self {
        ConnectionOptions {
            content_length: None,
            first_retry_max_delay: Some(100),
            forced_transport: None,
            http_extra_headers: None,
            http_extra_headers_on_session_creation_only: Some(false),
            idle_timeout: Some(19000),
            keepalive_interval: Some(0),
            polling_interval: Some(0),
            proxy: None,
            real_max_bandwidth: None,
            reconnect_timeout: Some(3000),
            requested_max_bandwidth: None,
            retry_delay: Some(4000),
            reverse_heartbeat_interval: Some(0),
            server_instance_address_ignored: Some(false),
            session_recovery_timeout: Some(15000),
            slowing_enabled: Some(false),
            stalled_timeout: Some(2000),
        }
    }
}
/// Interface to be implemented to listen to `LightstreamerClient` events comprehending notifications
/// of connection activity and errors.
///
/// Events for these listeners are dispatched by a different thread than the one that generates them.
/// This means that, upon reception of an event, it is possible that the internal state of the client
/// has changed. On the other hand, all the notifications for a single `LightstreamerClient`,
/// including notifications to `ClientListener`, `SubscriptionListener` and `ClientMessageListener`
/// will be dispatched by the same thread.
pub trait ClientListener {
    /// Event handler that receives a notification when the `ClientListener` instance is removed
    /// from a `LightstreamerClient` through `LightstreamerClient.removeListener()`. This is the
    /// last event to be fired on the listener.
    fn on_listen_end(&self) {
        // Implementation for on_listen_end
    }

    /// Event handler that receives a notification when the `ClientListener` instance is added
    /// to a `LightstreamerClient` through `LightstreamerClient.addListener()`. This is the first
    /// event to be fired on the listener.
    fn on_listen_start(&self) {
        // Implementation for on_listen_start
    }

    /// Event handler that receives a notification each time the value of a property of
    /// `LightstreamerClient.connectionDetails` or `LightstreamerClient.connectionOptions` is changed.
    ///
    /// Properties of these objects can be modified by direct calls to them or by server sent events.
    ///
    /// # Parameters
    ///
    /// * `property`: the name of the changed property.
    ///
    /// Possible values are:
    ///
    /// - `adapterSet`
    /// - `serverAddress`
    /// - `user`
    /// - `password`
    /// - `serverInstanceAddress`
    /// - `serverSocketName`
    /// - `clientIp`
    /// - `sessionId`
    /// - `contentLength`
    /// - `idleTimeout`
    /// - `keepaliveInterval`
    /// - `requestedMaxBandwidth`
    /// - `realMaxBandwidth`
    /// - `pollingInterval`
    /// - `reconnectTimeout`
    /// - `stalledTimeout`
    /// - `retryDelay`
    /// - `firstRetryMaxDelay`
    /// - `slowingEnabled`
    /// - `forcedTransport`
    /// - `serverInstanceAddressIgnored`
    /// - `reverseHeartbeatInterval`
    /// - `earlyWSOpenEnabled`
    /// - `httpExtraHeaders`
    /// - `httpExtraHeadersOnSessionCreationOnly`
    ///
    /// See also `LightstreamerClient.connectionDetails`
    ///
    /// See also `LightstreamerClient.connectionOptions`
    fn on_property_change(&self, property: &str) {
        // Implementation for on_property_change
    }

    /// Event handler that is called when the Server notifies a refusal on the client attempt
    /// to open a new connection or the interruption of a streaming connection. In both cases,
    /// the `onStatusChange()` event handler has already been invoked with a "DISCONNECTED" status
    /// and no recovery attempt has been performed. By setting a custom handler, however, it is
    /// possible to override this and perform custom recovery actions.
    ///
    /// # Parameters
    ///
    /// * `code`: The error code. It can be one of the following:
    ///   - `1`: user/password check failed
    ///   - `2`: requested Adapter Set not available
    ///   - `7`: licensed maximum number of sessions reached (this can only happen with some licenses)
    ///   - `8`: configured maximum number of sessions reached
    ///   - `9`: configured maximum server load reached
    ///   - `10`: new sessions temporarily blocked
    ///   - `11`: streaming is not available because of Server license restrictions (this can only happen with special licenses).
    ///   - `21`: a request for this session has unexpectedly reached a wrong Server instance, which suggests that a routing issue may be in place.
    ///   - `30-41`: the current connection or the whole session has been closed by external agents; the possible cause may be:
    ///     - The session was closed on the Server side (via software or by the administrator) (32), or through a client "destroy" request (31);
    ///     - The Metadata Adapter imposes limits on the overall open sessions for the current user and has requested the closure of the current session upon opening of a new session for the same user on a different browser window (35);
    ///     - An unexpected error occurred on the Server while the session was in activity (33, 34);
    ///     - An unknown or unexpected cause; any code different from the ones identified in the above cases could be issued. A detailed description for the specific cause is currently not supplied (i.e. `errorMessage` is `None` in this case).
    ///   - `60`: this version of the client is not allowed by the current license terms.
    ///   - `61`: there was an error in the parsing of the server response thus the client cannot continue with the current session.
    ///   - `66`: an unexpected exception was thrown by the Metadata Adapter while authorizing the connection.
    ///   - `68`: the Server could not open or continue with the session because of an internal error.
    ///   - `70`: an unusable port was configured on the server address.
    ///   - `71`: this kind of client is not allowed by the current license terms.
    ///   - `<= 0`: the Metadata Adapter has refused the user connection; the code value is dependent on the specific Metadata Adapter implementation
    /// * `message`: The description of the error as sent by the Server.
    ///
    /// See also `onStatusChange()`
    ///
    /// See also `ConnectionDetails.setAdapterSet()`
    fn on_server_error(&self, code: i32, message: &str) {
        // Implementation for on_server_error
    }

    /// Event handler that receives a notification each time the `LightstreamerClient` status has changed.
    /// The status changes may be originated either by custom actions (e.g. by calling `LightstreamerClient.disconnect()`)
    /// or by internal actions.
    ///
    /// The normal cases are the following:
    ///
    /// After issuing `connect()` when the current status is `DISCONNECTED*`, the client will switch to `CONNECTING`
    /// first and to `CONNECTED:STREAM-SENSING` as soon as the pre-flight request receives its answer. As soon as
    /// the new session is established, it will switch to `CONNECTED:WS-STREAMING` if the environment permits WebSockets;
    /// otherwise it will switch to `CONNECTED:HTTP-STREAMING` if the environment permits streaming or to
    /// `CONNECTED:HTTP-POLLING` as a last resort.
    ///
    /// On the other hand, after issuing `connect` when the status is already `CONNECTED:*` a switch to `CONNECTING`
    /// is usually not needed and the current session is kept.
    ///
    /// After issuing `LightstreamerClient.disconnect()`, the status will switch to `DISCONNECTED`.
    ///
    /// In case of a server connection refusal, the status may switch from `CONNECTING` directly to `DISCONNECTED`.
    /// After that, the `onServerError()` event handler will be invoked.
    ///
    /// Possible special cases are the following:
    ///
    /// - In case of Server unavailability during streaming, the status may switch from `CONNECTED:*-STREAMING` to
    ///   `STALLED` (see `ConnectionOptions.setStalledTimeout()`). If the unavailability ceases, the status will
    ///   switch back to `CONNECTED:*-STREAMING`; otherwise, if the unavailability persists (see `ConnectionOptions.setReconnectTimeout()`),
    ///   the status will switch to `DISCONNECTED:TRYING-RECOVERY` and eventually to `CONNECTED:*-STREAMING`.
    /// - In case the connection or the whole session is forcibly closed by the Server, the status may switch from
    ///   `CONNECTED:*-STREAMING` or `CONNECTED:*-POLLING` directly to `DISCONNECTED`. After that, the `onServerError()`
    ///   event handler will be invoked.
    /// - Depending on the setting in `ConnectionOptions.setSlowingEnabled()`, in case of slow update processing,
    ///   the status may switch from `CONNECTED:WS-STREAMING` to `CONNECTED:WS-POLLING` or from `CONNECTED:HTTP-STREAMING`
    ///   to `CONNECTED:HTTP-POLLING`.
    /// - If the status is `CONNECTED:*-POLLING` and any problem during an intermediate poll occurs, the status may
    ///   switch to `CONNECTING` and eventually to `CONNECTED:*-POLLING`. The same may hold for the `CONNECTED:*-STREAMING`
    ///   case, when a rebind is needed.
    /// - In case a forced transport was set through `ConnectionOptions.setForcedTransport()`, only the related final
    ///   status or statuses are possible.
    /// - In case of connection problems, the status may switch from any value to `DISCONNECTED:WILL-RETRY`
    ///   (see `ConnectionOptions.setRetryDelay()`), then to `CONNECTING` and a new attempt will start. However,
    ///   in most cases, the client will try to recover the current session; hence, the `DISCONNECTED:TRYING-RECOVERY`
    ///   status will be entered and the recovery attempt will start.
    /// - In case of connection problems during a recovery attempt, the status may stay in `DISCONNECTED:TRYING-RECOVERY`
    ///   for long time, while further attempts are made. If the recovery is no longer possible, the current session
    ///   will be abandoned and the status will switch to `DISCONNECTED:WILL-RETRY` before the next attempts.
    ///
    /// By setting a custom handler it is possible to perform actions related to connection and disconnection
    /// occurrences. Note that `LightstreamerClient.connect()` and `LightstreamerClient.disconnect()`, as any other
    /// method, can be issued directly from within a handler.
    ///
    /// # Parameters
    ///
    /// * `status`: The new status. It can be one of the following values:
    ///   - `CONNECTING`: the client has started a connection attempt and is waiting for a Server answer.
    ///   - `CONNECTED:STREAM-SENSING`: the client received a first response from the server and is now evaluating
    ///     if a streaming connection is fully functional.
    ///   - `CONNECTED:WS-STREAMING`: a streaming connection over WebSocket has been established.
    ///   - `CONNECTED:HTTP-STREAMING`: a streaming connection over HTTP has been established.
    ///   - `CONNECTED:WS-POLLING`: a polling connection over WebSocket has been started. Note that, unlike polling
    ///     over HTTP, in this case only one connection is actually opened (see `ConnectionOptions.setSlowingEnabled()`).
    ///   - `CONNECTED:HTTP-POLLING`: a polling connection over HTTP has been started.
    ///   - `STALLED`: a streaming session has been silent for a while, the status will eventually return to its
    ///     previous `CONNECTED:*-STREAMING` status or will switch to `DISCONNECTED:WILL-RETRY` / `DISCONNECTED:TRYING-RECOVERY`.
    ///   - `DISCONNECTED:WILL-RETRY`: a connection or connection attempt has been closed; a new attempt will be
    ///     performed (possibly after a timeout).
    ///   - `DISCONNECTED:TRYING-RECOVERY`: a connection has been closed and the client has started a connection
    ///     attempt and is waiting for a Server answer; if successful, the underlying session will be kept.
    ///   - `DISCONNECTED`: a connection or connection attempt has been closed. The client will not connect anymore
    ///     until a new `LightstreamerClient.connect()` call is issued.
    ///
    /// See also `LightstreamerClient.connect()`
    ///
    /// See also `LightstreamerClient.disconnect()`
    ///
    /// See also `LightstreamerClient.getStatus()`
    fn on_status_change(&self, status: &str) {
        // Implementation for on_status_change
    }
}
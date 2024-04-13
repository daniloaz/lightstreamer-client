/// Interface to be implemented to listen to `LightstreamerClient.sendMessage()` events reporting
/// a message processing outcome. Events for these listeners are dispatched by a different
/// thread than the one that generates them. All the notifications for a single `LightstreamerClient`,
/// including notifications to `ClientListener`, `SubscriptionListener` and `ClientMessageListener`
/// will be dispatched by the same thread. Only one event per message is fired on this listener.
pub trait ClientMessageListener {
    /// Event handler that is called by Lightstreamer when any notifications of the processing
    /// outcome of the related message haven't been received yet and can no longer be received.
    /// Typically, this happens after the session has been closed. In this case, the client has
    /// no way of knowing the processing outcome and any outcome is possible.
    ///
    /// # Parameters
    ///
    /// * `msg`: the message to which this notification is related.
    /// * `sent_on_network`: `true` if the message was sent on the network, `false` otherwise.
    ///   Even if the flag is `true`, it is not possible to infer whether the message actually
    ///   reached the Lightstreamer Server or not.
    fn on_abort(&self, _msg: &str, _sent_on_network: bool) {
        // Implementation for on_abort
        unimplemented!("Implement on_abort method for ClientMessageListener.");
    }

    /// Event handler that is called by Lightstreamer when the related message has been processed
    /// by the Server but the expected processing outcome could not be achieved for any reason.
    ///
    /// # Parameters
    ///
    /// * `msg`: the message to which this notification is related.
    /// * `code`: the error code sent by the Server. It can be one of the following:
    ///   - `<= 0`: the Metadata Adapter has refused the message; the code value is dependent
    ///     on the specific Metadata Adapter implementation.
    /// * `error`: the description of the error sent by the Server.
    fn on_deny(&self, _msg: &str, _code: i32, _error: &str) {
        // Implementation for on_deny
        unimplemented!("Implement on_deny method for ClientMessageListener.");
    }

    /// Event handler that is called by Lightstreamer to notify that the related message has
    /// been discarded by the Server. This means that the message has not reached the Metadata
    /// Adapter and the message next in the sequence is considered enabled for processing.
    ///
    /// # Parameters
    ///
    /// * `msg`: the message to which this notification is related.
    fn on_discarded(&self, _msg: &str) {
        // Implementation for on_discarded
        unimplemented!("Implement on_discarded method for ClientMessageListener.");
    }

    /// Event handler that is called by Lightstreamer when the related message has been processed
    /// by the Server but the processing has failed for any reason. The level of completion of
    /// the processing by the Metadata Adapter cannot be determined.
    ///
    /// # Parameters
    ///
    /// * `msg`: the message to which this notification is related.
    fn on_error(&self, _msg: &str) {
        // Implementation for on_error
        unimplemented!("Implement on_error method for ClientMessageListener.");
    }

    /// Event handler that is called by Lightstreamer when the related message has been processed
    /// by the Server with success.
    ///
    /// # Parameters
    ///
    /// * `msg`: the message to which this notification is related.
    /// * `response`: the response from the Metadata Adapter. If not supplied (i.e. supplied as `None`),
    ///   an empty message is received here.
    fn on_processed(&self, _msg: &str, _response: Option<&str>) {
        // Implementation for on_processed
        unimplemented!("Implement on_processed method for ClientMessageListener.");
    }
}

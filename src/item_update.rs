use std::collections::HashMap;
use serde_json::Value;

/// Contains all the information related to an update of the field values for an item.
/// It reports all the new values of the fields.
///
/// If the involved Subscription is a COMMAND Subscription, then the values for the current update
/// are meant as relative to the same key.
///
/// Moreover, if the involved Subscription has a two-level behavior enabled, then each update may
/// be associated with either a first-level or a second-level item. In this case, the reported fields
/// are always the union of the first-level and second-level fields, and each single update can only
/// change either the first-level or the second-level fields (but for the "command" field, which is
/// first-level and is always set to "UPDATE" upon a second-level update). When the two-level behavior
/// is enabled, in all methods where a field name has to be supplied, the following convention should
/// be followed:
///
/// - The field name can always be used, both for the first-level and the second-level fields.
///   In case of name conflict, the first-level field is meant.
/// - The field position can always be used; however, the field positions for the second-level fields
///   start at the highest position of the first-level field list + 1. If a field schema had been
///   specified for either first-level or second-level Subscriptions, then client-side knowledge of
///   the first-level schema length would be required.
pub struct ItemUpdate {
    changed_fields: HashMap<String, Value>,
    fields: HashMap<String, Value>,
    item_name: Option<String>,
    item_pos: usize,
    is_snapshot: bool,
    prev_values: HashMap<String, Value>,
}

impl ItemUpdate {
    /// Returns a map containing the values for each field changed with the last server update.
    /// The related field name is used as key for the values in the map.
    ///
    /// Note that if the Subscription mode of the involved Subscription is COMMAND, then changed
    /// fields are meant as relative to the previous update for the same key. On such tables if a
    /// DELETE command is received, all the fields, excluding the key field, will be present as
    /// changed, with None value. All of this is also true on tables that have the two-level behavior
    /// enabled, but in case of DELETE commands second-level fields will not be iterated.
    ///
    /// # Errors
    ///
    /// Returns an `IllegalStateException` if the Subscription was initialized using a field schema.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::json;
    /// let item_update = ItemUpdate {
    ///     changed_fields: vec![("foo".to_string(), json!(42)), ("bar".to_string(), json!("baz"))]
    ///         .into_iter()
    ///         .collect(),
    ///     fields: vec![("foo".to_string(), json!(42)), ("bar".to_string(), json!("baz"))]
    ///         .into_iter()
    ///         .collect(),
    ///     item_name: Some("item1".to_string()),
    ///     item_pos: 1,
    ///     is_snapshot: false,
    ///     prev_values: HashMap::new(),
    /// };
    /// let changed_fields = item_update.get_changed_fields();
    /// assert_eq!(changed_fields.len(), 2);
    /// assert_eq!(changed_fields.get("foo"), Some(&json!(42)));
    /// assert_eq!(changed_fields.get("bar"), Some(&json!("baz")));
    /// ```
    pub fn get_changed_fields(&self) -> &HashMap<String, Value> {
        &self.changed_fields
    }

    /// Returns a map containing the values for each field changed with the last server update.
    /// The 1-based field position within the field schema or field list is used as key for the
    /// values in the map.
    ///
    /// Note that if the Subscription mode of the involved Subscription is COMMAND, then changed
    /// fields are meant as relative to the previous update for the same key. On such tables if a
    /// DELETE command is received, all the fields, excluding the key field, will be present as
    /// changed, with None value. All of this is also true on tables that have the two-level behavior
    /// enabled, but in case of DELETE commands second-level fields will not be iterated.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::json;
    /// use std::collections::HashMap;
    /// let mut changed_fields_by_pos = HashMap::new();
    /// changed_fields_by_pos.insert(1, json!(42));
    /// changed_fields_by_pos.insert(2, json!("baz"));
    /// let item_update = ItemUpdate {
    ///     changed_fields: changed_fields_by_pos,
    ///     fields: HashMap::new(),
    ///     item_name: None,
    ///     item_pos: 0,
    ///     is_snapshot: false,
    ///     prev_values: HashMap::new(),
    /// };
    /// let changed_fields = item_update.get_changed_fields_by_position();
    /// assert_eq!(changed_fields.len(), 2);
    /// assert_eq!(changed_fields.get(&1), Some(&json!(42)));
    /// assert_eq!(changed_fields.get(&2), Some(&json!("baz")));
    /// ```
    pub fn get_changed_fields_by_position(&self) -> HashMap<usize, Value> {
        // Convert the changed_fields HashMap to a HashMap with usize keys
        let changed_fields_by_pos: HashMap<usize, Value> = self
            .changed_fields
            .iter()
            .enumerate()
            .map(|(i, (_k, v))| (i + 1, v.clone()))
            .collect();
        changed_fields_by_pos
    }

    /// Returns a map containing the values for each field in the Subscription.
    /// The related field name is used as key for the values in the map.
    ///
    /// # Errors
    ///
    /// Returns an `IllegalStateException` if the Subscription was initialized using a field schema.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::json;
    /// let item_update = ItemUpdate {
    ///     changed_fields: HashMap::new(),
    ///     fields: vec![("foo".to_string(), json!(42)), ("bar".to_string(), json!("baz"))]
    ///         .into_iter()
    ///         .collect(),
    ///     item_name: Some("item1".to_string()),
    ///     item_pos: 1,
    ///     is_snapshot: false,
    ///     prev_values: HashMap::new(),
    /// };
    /// let fields = item_update.get_fields();
    /// assert_eq!(fields.len(), 2);
    /// assert_eq!(fields.get("foo"), Some(&json!(42)));
    /// assert_eq!(fields.get("bar"), Some(&json!("baz")));
    /// ```
    pub fn get_fields(&self) -> &HashMap<String, Value> {
        &self.fields
    }

    /// Returns a map containing the values for each field in the Subscription.
    /// The 1-based field position within the field schema or field list is used as key for the
    /// values in the map.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::json;
    /// use std::collections::HashMap;
    /// let mut fields_by_pos = HashMap::new();
    /// fields_by_pos.insert(1, json!(42));
    /// fields_by_pos.insert(2, json!("baz"));
    /// let item_update = ItemUpdate {
    ///     changed_fields: HashMap::new(),
    ///     fields: fields_by_pos,
    ///     item_name: None,
    ///     item_pos: 0,
    ///     is_snapshot: false,
    ///     prev_values: HashMap::new(),
    /// };
    /// let fields = item_update.get_fields_by_position();
    /// assert_eq!(fields.len(), 2);
    /// assert_eq!(fields.get(&1), Some(&json!(42)));
    /// assert_eq!(fields.get(&2), Some(&json!("baz")));
    /// ```
    pub fn get_fields_by_position(&self) -> &HashMap<String, Value> {
        &self.fields
    }

    /// Inquiry method that retrieves the name of the item to which this update pertains.
    ///
    /// The name will be `None` if the related Subscription was initialized using an "Item Group".
    ///
    /// # Examples
    ///
    /// ```
    /// let item_update = ItemUpdate {
    ///     changed_fields: HashMap::new(),
    ///     fields: HashMap::new(),
    ///     item_name: Some("item1".to_string()),
    ///     item_pos: 1,
    ///     is_snapshot: false,
    ///     prev_values: HashMap::new(),
    /// };
    /// assert_eq!(item_update.get_item_name(), Some("item1".to_string()));
    /// ```
    pub fn get_item_name(&self) -> Option<&String> {
        self.item_name.as_ref()
    }

    /// Inquiry method that retrieves the position in the "Item List" or "Item Group" of the item
    /// to which this update pertains.
    ///
    /// # Examples
    ///
    /// ```
    /// let item_update = ItemUpdate {
    ///     changed_fields: HashMap::new(),
    ///     fields: HashMap::new(),
    ///     item_name: None,
    ///     item_pos: 5,
    ///     is_snapshot: false,
    ///     prev_values: HashMap::new(),
    /// };
    /// assert_eq!(item_update.get_item_pos(), 5);
    /// ```
    pub fn get_item_pos(&self) -> usize {
        self.item_pos
    }

    /// Inquiry method that gets the value for a specified field, as received from the Server with
    /// the current or previous update.
    ///
    /// # Errors
    ///
    /// Returns an `IllegalArgumentException` if the specified field is not part of the Subscription.
    ///
    /// # Parameters
    ///
    /// - `field_name_or_pos`: The field name or the 1-based position of the field within the "Field
    ///   List" or "Field Schema".
    ///
    /// # Returns
    ///
    /// The value of the specified field; it can be `None` in the following cases:
    /// - A `None` value has been received from the Server, as `None` is a possible value for a field.
    /// - No value has been received for the field yet.
    /// - The item is subscribed to with the COMMAND mode and a DELETE command is received (only the
    ///   fields used to carry key and command information are valued).
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::json;
    /// let item_update = ItemUpdate {
    ///     changed_fields: vec![("foo".to_string(), json!(42)), ("bar".to_string(), json!("baz"))]
    ///         .into_iter()
    ///         .collect(),
    ///     fields: vec![("foo".to_string(), json!(42)), ("bar".to_string(), json!("baz"))]
    ///         .into_iter()
    ///         .collect(),
    ///     item_name: Some("item1".to_string()),
    ///     item_pos: 1,
    ///     is_snapshot: false,
    ///     prev_values: HashMap::new(),
    /// };
    /// assert_eq!(item_update.get_value("foo"), Some(json!(42)));
    /// assert_eq!(item_update.get_value("bar"), Some(json!("baz")));
    /// assert_eq!(item_update.get_value(1), Some(json!(42)));
    /// assert_eq!(item_update.get_value(2), Some(json!("baz")));
    /// ```
    pub fn get_value(&self, field_name_or_pos: &str) -> Option<&Value> {
        self.fields.get(field_name_or_pos)
    }

    /// Inquiry method that gets the difference between the new value and the previous one as a
    /// JSON Patch structure, provided that the Server has used the JSON Patch format to send this
    /// difference, as part of the "delta delivery" mechanism. This, in turn, requires that:
    ///
    /// - The Data Adapter has explicitly indicated JSON Patch as the privileged type of compression
    ///   for this field.
    /// - Both the previous and new value are suitable for the JSON Patch computation (i.e. they are
    ///   valid JSON representations).
    /// - The item was subscribed to in MERGE or DISTINCT mode (note that, in case of two-level
    ///   behavior, this holds for all fields related with second-level items, as these items are in
    ///   MERGE mode).
    /// - Sending the JSON Patch difference has been evaluated by the Server as more efficient than
    ///   sending the full new value.
    ///
    /// Note that the last condition can be enforced by leveraging the Server's `<jsonpatch_min_length>`
    /// configuration flag, so that the availability of the JSON Patch form would only depend on the
    /// Client and the Data Adapter.
    ///
    /// When the above conditions are not met, the method just returns `None`; in this case, the
    /// new value can only be determined through `ItemUpdate::get_value()`. For instance, this will
    /// always be needed to get the first value received.
    ///
    /// # Errors
    ///
    /// Returns an `IllegalArgumentException` if the specified field is not part of the Subscription.
    ///
    /// # Parameters
    ///
    /// - `field_name_or_pos`: The field name or the 1-based position of the field within the "Field
    ///   List" or "Field Schema".
    ///
    /// # Returns
    ///
    /// A JSON Patch structure representing the difference between the new value and the previous one,
    /// or `None` if the difference in JSON Patch format is not available for any reason.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::{json, Value};
    /// let mut item_update = ItemUpdate {
    ///     changed_fields: vec![("foo".to_string(), json!(42))]
    ///         .into_iter()
    ///         .collect(),
    ///     fields: vec![("foo".to_string(), json!(42))]
    ///         .into_iter()
    ///         .collect(),
    ///     item_name: Some("item1".to_string()),
    ///     item_pos: 1,
    ///     is_snapshot: false,
    ///     prev_values: vec![("foo".to_string(), json!(41))]
    ///         .into_iter()
    ///         .collect(),
    /// };
    ///
    /// // Assuming the Server sends a JSON Patch for the "foo" field
    /// let json_patch: Value = json!([
    ///     { "op": "replace", "path": "/foo", "value": 42 }
    /// ]);
    /// item_update.changed_fields.insert("foo".to_string(), json_patch.clone());
    ///
    /// assert_eq!(
    ///     item_update.get_value_as_json_patch_if_available("foo"),
    ///     Some(&json_patch)
    /// );
    /// ```
    pub fn get_value_as_json_patch_if_available(&self, field_name_or_pos: &str) -> Option<&Value> {
        self.changed_fields.get(field_name_or_pos)
    }

    /// Inquiry method that asks whether the current update belongs to the item snapshot (which carries
    /// the current item state at the time of Subscription). Snapshot events are sent only if snapshot
    /// information was requested for the items through `Subscription::set_requested_snapshot()` and
    /// precede the real time events. Snapshot information take different forms in different subscription
    /// modes and can be spanned across zero, one or several update events. In particular:
    ///
    /// - If the item is subscribed to with the RAW subscription mode, then no snapshot is sent by
    ///   the Server.
    /// - If the item is subscribed to with the MERGE subscription mode, then the snapshot consists
    ///   of exactly one event, carrying the current value for all fields.
    /// - If the item is subscribed to with the DISTINCT subscription mode, then the snapshot consists
    ///   of some of the most recent updates; these updates are as many as specified through
    ///   `Subscription::set_requested_snapshot()`, unless fewer are available.
    /// - If the item is subscribed to with the COMMAND subscription mode, then the snapshot consists
    ///   of an "ADD" event for each key that is currently present.
    ///
    /// Note that, in case of two-level behavior, snapshot-related updates for both the first-level
    /// item (which is in COMMAND mode) and any second-level items (which are in MERGE mode) are
    /// qualified with this flag.
    ///
    /// # Examples
    ///
    /// ```
    /// let item_update = ItemUpdate {
    ///     changed_fields: HashMap::new(),
    ///     fields: HashMap::new(),
    ///     item_name: None,
    ///     item_pos: 0,
    ///     is_snapshot: true,
    ///     prev_values: HashMap::new(),
    /// };
    /// assert!(item_update.is_snapshot());
    /// ```
    pub fn is_snapshot(&self) -> bool {
        self.is_snapshot
    }

    /// Inquiry method that asks whether the value for a field has changed after the reception of
    /// the last update from the Server for an item. If the Subscription mode is COMMAND then the
    /// change is meant as relative to the same key.
    ///
    /// # Parameters
    ///
    /// - `field_name_or_pos`: The field name or the 1-based position of the field within the field
    ///   list or field schema.
    ///
    /// # Returns
    ///
    /// Unless the Subscription mode is COMMAND, the return value is `true` in the following cases:
    ///
    /// - It is the first update for the item.
    /// - The new field value is different than the previous field value received for the item.
    ///
    /// If the Subscription mode is COMMAND, the return value is `true` in the following cases:
    ///
    /// - It is the first update for the involved key value (i.e. the event carries an "ADD" command).
    /// - The new field value is different than the previous field value received for the item,
    ///   relative to the same key value (the event must carry an "UPDATE" command).
    /// - The event carries a "DELETE" command (this applies to all fields other than the field used
    ///   to carry key information).
    ///
    /// In all other cases, the return value is `false`.
    ///
    /// # Errors
    ///
    /// Returns an `IllegalArgumentException` if the specified field is not part of the Subscription.
    ///
    /// # Examples
    ///
    /// ```
    /// use serde_json::json;
    /// let item_update = ItemUpdate {
    ///     changed_fields: vec![("foo".to_string(), json!(42))]
    ///         .into_iter()
    ///         .collect(),
    ///     fields: vec![("foo".to_string(), json!(42)), ("bar".to_string(), json!("baz"))]
    ///         .into_iter()
    ///         .collect(),
    ///     item_name: Some("item1".to_string()),
    ///     item_pos: 1,
    ///     is_snapshot: false,
    ///     prev_values: vec![("foo".to_string(), json!(41))]
    ///         .into_iter()
    ///         .collect(),
    /// };
    /// assert!(item_update.is_value_changed("foo"));
    /// assert!(!item_update.is_value_changed("bar"));
    /// ```
    pub fn is_value_changed(&self, field_name_or_pos: &str) -> bool {
        if let Some(new_value) = self.fields.get(field_name_or_pos) {
            if let Some(prev_value) = self.prev_values.get(field_name_or_pos) {
                return new_value != prev_value;
            } else {
                // This is the first update for the item
                return true;
            }
        }
        false
    }
}
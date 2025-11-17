<div align="center">

<img src="assets/lightstreamer-logo.svg" alt="Lightstreamer" width="400"/>

# Lightstreamer Rust Client SDK

**A Rust implementation of the Lightstreamer TLCP (Text-based Live Connections Protocol)**

<!-- Project Info -->
[![License: GPL-3.0](https://img.shields.io/badge/license-GPL--3.0-blue.svg)](https://github.com/daniloaz/lightstreamer-client/blob/main/LICENSE)
[![Crates.io](https://img.shields.io/crates/v/lightstreamer-client.svg)](https://crates.io/crates/lightstreamer-client)
[![Downloads](https://img.shields.io/crates/d/lightstreamer-client.svg)](https://crates.io/crates/lightstreamer-client)
[![Documentation](https://docs.rs/lightstreamer-client/badge.svg)](https://docs.rs/lightstreamer-client)
[![Rust](https://img.shields.io/badge/rust-1.70%2B-orange.svg?logo=rust)](https://www.rust-lang.org)

<!-- Quality & Metrics -->
[![Dependencies](https://deps.rs/repo/github/daniloaz/lightstreamer-client/status.svg)](https://deps.rs/repo/github/daniloaz/lightstreamer-client)
[![Code Size](https://img.shields.io/github/languages/code-size/daniloaz/lightstreamer-client.svg?color=orange)](https://github.com/daniloaz/lightstreamer-client)
[![Contributors](https://img.shields.io/github/contributors/daniloaz/lightstreamer-client.svg)](https://github.com/daniloaz/lightstreamer-client/graphs/contributors)
[![Last Commit](https://img.shields.io/github/last-commit/daniloaz/lightstreamer-client.svg)](https://github.com/daniloaz/lightstreamer-client/commits/main)

<!-- Community & Status -->
[![GitHub Stars](https://img.shields.io/github/stars/daniloaz/lightstreamer-client.svg?style=social)](https://github.com/daniloaz/lightstreamer-client/stargazers)
[![GitHub Issues](https://img.shields.io/github/issues/daniloaz/lightstreamer-client.svg)](https://github.com/daniloaz/lightstreamer-client/issues)
[![Made with Rust](https://img.shields.io/badge/Made%20with-Rust-orange.svg?logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)](https://github.com/daniloaz/lightstreamer-client/graphs/commit-activity)

[Documentation](https://docs.rs/lightstreamer-client) | [Crates.io](https://crates.io/crates/lightstreamer-client) | [Repository](https://github.com/daniloaz/lightstreamer-client)

</div>

---

This project is a partial implementation of the Lightstreamer TLCP (Text-based Live Connections Protocol) in Rust. It provides a client SDK to interact with Lightstreamer servers, focused on supporting the specific needs of the [ig_trading_api](https://github.com/daniloaz/ig_trading_api) project.

## Features

- Full-duplex WebSocket-based connection mode.
- Subscriptions to items and item groups.
- MERGE subscription mode.
- Listening to connection events and messages.
- Configuration of connection options and connection details.
- Subscription lifecycle management.
- Retrieval of real-time item updates.

Please note that this SDK currently does not support all the features and capabilities of the full Lightstreamer protocol. It has been developed to cover the requirements of the ig_trading_api project mentioned above. Features like other connection modes, subscription modes (DISTINCT, RAW, COMMAND), and some other advanced options are not implemented at this time.

## Installation

To use this SDK in your Rust project, add the following dependency to your `Cargo.toml`:

```toml
[dependencies]
lightstreamer-client = "0.1.9"
```

## Usage

Here's a minimal example of how to use the Lightstreamer Rust Client SDK:

```rust
use lightstreamer_client::ls_client::LightstreamerClient;
use lightstreamer_client::subscription::{Subscription, SubscriptionMode};

#[tokio::main]
async fn main() {
    // Create a Lightstreamer client
    let client = LightstreamerClient::new(
        Some("http://push.lightstreamer.com/lightstreamer"), // Lightstreamer server
        Some("DEMO"), // adapter set
        None, // username
        None, // password
    ).unwrap();

    // Create a subscription
    let mut subscription = Subscription::new(
        SubscriptionMode::Merge,
        Some(vec!["item1".to_string(), "item2".to_string()]),
        Some(vec!["field1".to_string(), "field2".to_string()]),
    ).unwrap();

    // Subscribe and connect
    client.subscribe(subscription);
    client.connect(None).await.unwrap();
}
```

For a more advanced example of how to use the SDK to subscribe to item updates, refer to the main.rs file in the project source code. It demonstrates creating a Lightstreamer client, setting up subscriptions, handling item updates, and managing the connection lifecycle with a configurable number of connection attempts.

For more details on using the SDK, please refer to the reference documentation.

## Documentation

The full SDK documentation is available at [docs.rs](https://docs.rs/lightstreamer-client).

## Project Structure

Although this SDK does not provide a complete implementation of the Lightstreamer protocol, it has been built with a solid structure and scaffolding, similar to the official Lightstreamer libraries. The code is documented, and the project is designed to facilitate contributions from the community to add support for missing features.

## License

This project is licensed under the GPL-3.0 License. See the [LICENSE](LICENSE) file for details.

## Contributing

Contributions are welcome. Please open an issue or submit a pull request to propose changes and help complete the SDK with additional Lightstreamer features.
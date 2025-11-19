# AI Agent Guidelines for Lightstreamer Rust Client SDK

## Project Overview

This is a **Rust client SDK** for Lightstreamer servers implementing the **TLCP (Text-based Live Connections Protocol)**. The project provides real-time streaming capabilities focused on WebSocket-based connections, subscriptions management, and item update handling.

### Project Characteristics
- **Language**: Rust (Edition 2021)
- **Primary Use Case**: Real-time data streaming from Lightstreamer servers
- **Architecture**: Async/await pattern using Tokio runtime
- **Protocol**: TLCP v2.4.0 over WebSocket
- **Current Focus**: MERGE subscription mode with WebSocket streaming transport
- **Target Application**: Built specifically to support the [ig_trading_api](https://github.com/daniloaz/ig_trading_api) project

### License
GPL-3.0-only - All code contributions must respect this license.

---

## Core Principles and Mandatory Rules

### 1. Communication Language
- **ALWAYS respond to users in English**, regardless of the user's input language
- **ALL code comments MUST be written in English**
- Keep existing comment format and style when modifying code
- Enhance comments for better quality and understanding when appropriate

### 2. Autonomous Development
- **Make programming decisions autonomously** without requesting confirmation at every step
- Choose the most effective resolution following best practices
- Ensure code efficiency and maintainability
- Only ask for clarification when requirements are genuinely ambiguous

### 3. Documentation Synchronization
- **ALWAYS update `.md` documentation files** to reflect code changes
- Documentation must represent the **current state**, not a changelog
- Keep documentation accurate, clear, and synchronized with implementation
- Document public APIs, modules, and key architectural decisions

### 4. Code Quality Standards
- Follow Rust best practices and idioms
- Respect existing code patterns in the project
- Write clear, self-documenting code
- Add meaningful comments for complex logic
- Ensure proper error handling throughout

---

## Technical Architecture

### Project Structure

```
lightstreamer-client/
├── src/
│   ├── lib.rs                      # Public API exports
│   ├── ls_client.rs                # Main client implementation (Facade)
│   ├── subscription.rs             # Subscription management
│   ├── item_update.rs              # Real-time update data structure
│   ├── connection_details.rs       # Server connection configuration
│   ├── connection_options.rs       # Connection behavior options
│   ├── client_listener.rs          # Client event callbacks (trait)
│   ├── subscription_listener.rs    # Subscription event callbacks (trait)
│   ├── client_message_listener.rs  # Message event callbacks (trait)
│   ├── error.rs                    # Custom error types
│   ├── proxy.rs                    # Proxy configuration (future)
│   ├── util.rs                     # Utility functions
│   └── main.rs                     # Example/demo usage
├── Cargo.toml                      # Dependencies and metadata
├── README.md                       # User-facing documentation
├── LICENSE                         # GPL-3.0 license
└── AGENTS.md                       # This file
```

### Key Dependencies

```toml
# Async Runtime
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }

# WebSocket Communication
tokio-tungstenite = { version = "0", features = ["native-tls"] }

# HTTP Client
reqwest = { version = "0", features = ["json", "stream"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"
serde_urlencoded = "0"

# Async Utilities
futures = "0"
futures-util = "0"

# Logging
tracing = "0.1.40"
tracing-subscriber = "0.3.18"

# Other
url = "2"
json-patch = "1"
cookie = { version = "0", features = ["percent-encode"] }
signal-hook = "0"
colored = "2"
```

---

## Architectural Patterns

### 1. Facade Pattern
The `LightstreamerClient` serves as a facade for all operations:
- Connection lifecycle management
- Subscription handling
- Message sending
- Event listener registration

### 2. Listener/Observer Pattern
Event-driven architecture using trait-based listeners:
- `ClientListener`: Connection and session events
- `SubscriptionListener`: Subscription lifecycle and updates
- `ClientMessageListener`: Message delivery notifications

### 3. Async/Await with Tokio
- All I/O operations are non-blocking
- Uses `tokio::select!` for concurrent event handling
- Channel-based message passing for subscription operations

### 4. Builder Pattern
Configuration objects (`ConnectionDetails`, `ConnectionOptions`) use builder-like setters.

---

## Implementation Guidelines

### Connection Handling

#### WebSocket Communication
- **Only WebSocket streaming transport is currently supported**
- HTTP polling/streaming modes are planned but not implemented
- Connection uses standard WebSocket upgrade with custom TLCP headers
- Messages are text-based, CRLF-delimited

#### Connection Flow
1. WebSocket handshake with TLCP protocol headers
2. Send `wsok` message to initiate
3. Receive `wsok` confirmation from server
4. Send `create_session` with credentials and adapter set
5. Receive `conok` with session ID
6. Subscribe to items via `control` messages
7. Process real-time updates via message loop

### Subscription Management

#### Subscription Lifecycle
1. Create `Subscription` with mode, items, and fields
2. Call `LightstreamerClient::subscribe()` (non-blocking)
3. Client sends subscription request when session is active
4. Receive `subok` confirmation
5. Process `u` (update) messages
6. Unsubscribe via `LightstreamerClient::unsubscribe()`
7. Receive `unsub` confirmation

#### Current Limitations
- Only **MERGE mode** is fully implemented
- DISTINCT, RAW, and COMMAND modes are not supported yet
- Snapshot handling is implemented for MERGE mode

### Message Protocol (TLCP)

#### Key Message Types
- `wsok`: WebSocket connection confirmation
- `create_session`: Session creation request
- `conok`: Session creation confirmation
- `control`: Subscription/unsubscription commands
- `subok`: Subscription confirmation
- `unsub`: Unsubscription confirmation
- `u`: Item update (data)
- `reqok`: Request acknowledged
- `conerr`, `reqerr`: Error messages
- `probe`: Keep-alive probe

#### Update Message Format
```
u,<subscription_id>,<item_index>,<field_values>
```

Field values are pipe-separated:
- Empty value `""`: Unchanged field
- `#` or `$`: Null/empty field
- `^<number>`: Skip N unchanged fields
- `^P<json_patch>`: Apply JSON Patch
- `^T<tlcp_diff>`: Apply TLCP diff (not implemented)
- Regular value: URL-encoded field value

---

## Error Handling

### Custom Exceptions
Define custom error types in `error.rs`:
```rust
#[derive(Debug)]
pub struct IllegalStateException {
    message: String,
}
```

### Error Propagation
- Use `Result<T, Box<dyn Error + Send + Sync>>` for async operations
- Use `Result<T, Box<dyn Error>>` for sync operations
- Provide descriptive error messages
- Log errors appropriately before propagation

### Error Scenarios
- Connection failures
- Invalid configuration
- Protocol violations
- Subscription errors
- Network timeouts

---

## Logging Strategy

### Dual Logging Support
The client supports two logging mechanisms:

1. **Standard Logs** (`LogType::StdLogs`): Direct to stdout
2. **Tracing Logs** (`LogType::TracingLogs`): Via `tracing` crate

### Log Levels
- `ERROR`: Critical failures, protocol errors
- `WARN`: Non-critical issues, unexpected states
- `INFO`: Important events (connection, subscription confirmations)
- `DEBUG`: Detailed protocol messages, state changes
- `TRACE`: Low-level details (rarely used)

### Logging Categories (Future)
Plan to align with official Lightstreamer categories:
- `lightstreamer.stream`: Socket operations
- `lightstreamer.protocol`: TLCP protocol messages
- `lightstreamer.session`: Session lifecycle
- `lightstreamer.subscriptions`: Subscription events
- `lightstreamer.actions`: API calls and settings

---

## Code Style and Conventions

### Naming Conventions
- **Types**: `PascalCase` (e.g., `LightstreamerClient`, `ItemUpdate`)
- **Functions/Methods**: `snake_case` (e.g., `connect()`, `get_status()`)
- **Constants**: `SCREAMING_SNAKE_CASE` (e.g., `LIB_VERSION`, `TLCP_VERSION`)
- **Private fields**: Prefix with `_` if unused (e.g., `_session_id`)

### Documentation Comments
```rust
/// Brief description of the function/struct.
///
/// More detailed explanation if needed. Describe behavior,
/// use cases, and important notes.
///
/// # Parameters
///
/// * `param_name`: Description of parameter.
///
/// # Returns
///
/// Description of return value.
///
/// # Raises/Errors
///
/// Description of error conditions.
///
/// # Example
///
/// ```rust
/// // Example usage
/// ```
///
/// See also `related_function()`
```

### Error Messages
- Be specific and actionable
- Include context (what operation failed, why)
- Use proper capitalization and punctuation

### Async Functions
- Mark with `async` keyword
- Use `.await` for async operations
- Prefer `tokio::spawn` for background tasks
- Use `tokio::select!` for concurrent operations

---

## Testing Guidelines

### Test Organization
```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_name() {
        // Arrange
        // Act
        // Assert
    }

    #[tokio::test]
    async fn test_async_feature() {
        // Async test
    }
}
```

### Testing Priorities
1. **Connection logic**: WebSocket handshake, session creation
2. **Message parsing**: TLCP protocol messages
3. **Update handling**: Field value decoding, snapshot detection
4. **Subscription lifecycle**: Subscribe, update, unsubscribe
5. **Error handling**: Invalid states, protocol errors

### Mocking Strategy
- Mock WebSocket connections for unit tests
- Use test servers for integration tests
- Mock listener callbacks to verify event notifications

---

## Common Development Patterns

### Adding a New Feature

1. **Update the API**: Modify relevant structs/traits in `lib.rs`
2. **Implement logic**: Add implementation in appropriate module
3. **Add error handling**: Define new error types if needed
4. **Update documentation**: Sync README.md and inline docs
5. **Add tests**: Unit tests for the new feature
6. **Update examples**: If it affects usage, update `main.rs`

### Adding a New Subscription Mode

1. Extend `SubscriptionMode` enum in `subscription.rs`
2. Update `get_subscription_params()` in `ls_client.rs`
3. Modify update handling logic in the main message loop
4. Add snapshot detection logic for the new mode
5. Update tests and documentation

### Adding a New Transport

1. Extend `Transport` enum in `ls_client.rs`
2. Implement connection logic in `connect()` method
3. Handle transport-specific message formats
4. Update error handling for transport failures
5. Test with mock servers

---

## Important Constraints

### Protocol Limitations
- **TLCP v2.4.0**: Stick to this protocol version
- **WebSocket Only**: HTTP transports are not yet supported
- **MERGE Mode**: Primary focus; other modes are partial

### State Management
- Subscriptions persist across reconnections
- No session recovery mechanism yet implemented
- Shutdown signal handling via `Arc<Notify>`

### Thread Safety
- `LightstreamerClient` is **NOT** `Send` or `Sync` by default
- Listener callbacks execute in the event loop thread
- Keep listener logic fast and non-blocking
- Delegate heavy work to separate tasks

### Performance Considerations
- Minimize allocations in the hot path (update processing)
- Use `HashMap` for efficient field lookups
- Batch subscription requests when possible
- Consider message throughput in buffer sizing

---

## Feature Roadmap

### Implemented
- ✅ WebSocket streaming transport
- ✅ MERGE subscription mode
- ✅ Snapshot handling
- ✅ Real-time item updates
- ✅ Event listeners (Client, Subscription)
- ✅ Connection lifecycle management
- ✅ JSON Patch support (field diffs)

### Planned/Partial
- ⚠️ HTTP streaming transport
- ⚠️ HTTP/WS polling transports
- ⚠️ DISTINCT subscription mode
- ⚠️ RAW subscription mode
- ⚠️ COMMAND subscription mode
- ⚠️ TLCP-diff support
- ⚠️ Message sending (partial implementation)
- ⚠️ Cookie management
- ⚠️ Proxy support
- ⚠️ Session recovery

### Not Implemented
- ❌ Full client message handling
- ❌ Second-level subscriptions
- ❌ Custom logger provider
- ❌ SSL/TLS configuration
- ❌ Listener removal mechanism

---

## Git and Version Control

### Commit Messages
- Use clear, descriptive commit messages
- Start with a verb (Add, Fix, Update, Refactor, etc.)
- Reference issues when applicable

### Branching
- `main`: Stable, production-ready code
- Feature branches: `feature/description`
- Bug fixes: `fix/description`

### Version Numbering
Follow semantic versioning (SEMVER):
- MAJOR: Breaking API changes
- MINOR: New features, backward compatible
- PATCH: Bug fixes, backward compatible

Current version: **0.1.11** (see `Cargo.toml`)

---

## Working with This Project

### Before Making Changes
1. Read the relevant module documentation
2. Understand the TLCP protocol section involved
3. Check for related TODOs or `unimplemented!()` markers
4. Review existing patterns in similar code

### When Adding Code
1. Follow existing code style and patterns
2. Add comprehensive error handling
3. Write inline documentation for public APIs
4. Update README.md if user-facing changes
5. Add or update tests
6. Ensure `cargo build` succeeds
7. Run `cargo test` to verify tests pass
8. Run `cargo clippy` for linting
9. Format code with `cargo fmt`

### When Modifying Code
1. Preserve backward compatibility when possible
2. Update all related documentation
3. Check for side effects in dependent code
4. Update error messages if logic changes
5. Verify tests still pass

### When Removing Code
1. Ensure no public API is broken
2. Remove related tests
3. Update documentation
4. Check for orphaned imports or dependencies

---

## Debugging Tips

### Common Issues

#### Connection Failures
- Check server address format (http:// or https://)
- Verify adapter set name is correct
- Confirm credentials if authentication is required
- Check network connectivity and firewalls

#### Subscription Not Receiving Updates
- Verify item names match server configuration
- Check field names are correct
- Ensure subscription mode is MERGE
- Look for `subok` confirmation in logs

#### Message Parsing Errors
- Enable DEBUG logging to see raw messages
- Check for protocol version mismatches
- Verify URL encoding/decoding

### Logging for Debugging
```rust
client.set_logging_type(LogType::TracingLogs);
// Then configure tracing subscriber for detailed output
```

---

## API Design Philosophy

### Consistency with Official SDKs
The API design follows the structure of official Lightstreamer SDKs (Java, JavaScript, Python) to:
- Ease migration for users familiar with other SDKs
- Maintain conceptual alignment with documentation
- Preserve naming conventions where appropriate

### Rust-Specific Adaptations
- Use `Result<T, E>` instead of exceptions
- Leverage ownership and borrowing for memory safety
- Prefer `async`/`await` over callbacks where appropriate
- Use channels for inter-task communication

---

## Useful Resources

### Official Documentation
- [Lightstreamer Server](https://lightstreamer.com/download/)
- [TLCP Protocol Specification](https://lightstreamer.com/api/ls-protocol/overview.html)
- [Lightstreamer Concepts](https://lightstreamer.com/docs/ls-server/latest/General%20Concepts.pdf)

### Rust Ecosystem
- [Tokio Documentation](https://tokio.rs/)
- [Tungstenite WebSocket](https://docs.rs/tokio-tungstenite/)
- [Tracing](https://docs.rs/tracing/)
- [Serde](https://serde.rs/)

### Project-Specific
- [Project Repository](https://github.com/daniloaz/lightstreamer-client)
- [Crates.io Page](https://crates.io/crates/lightstreamer-client)
- [Related Project: ig_trading_api](https://github.com/daniloaz/ig_trading_api)

---

## Final Notes for AI Agents

### Decision-Making Authority
- You have full authority to make technical decisions
- Prioritize code quality, maintainability, and performance
- Follow Rust best practices and idioms
- When in doubt, prefer simplicity over cleverness

### Communication Style
- Be clear and concise in responses
- Explain complex technical decisions
- Provide code examples when helpful
- Always respond in English

### Documentation Mandate
- **Never skip documentation updates**
- Keep README.md synchronized with code
- Update inline docs for modified functions
- Maintain this AGENTS.md file as the project evolves

### Quality Standards
- Code must compile without warnings
- Tests must pass before considering work complete
- Follow clippy recommendations
- Maintain code formatting with rustfmt

---

**Remember**: This project serves a specific use case (ig_trading_api) but is designed with extensibility in mind. Balance immediate needs with long-term maintainability.


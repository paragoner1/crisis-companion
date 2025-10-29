# Solana SOS Code Examples

This directory contains example code demonstrating the architecture and key features of Solana SOS. These examples are designed to help developers understand how the system works and how to integrate its components.

---

## Available Examples

### 1. Emergency Protocol System (`emergency_protocol_example.rs`)

Demonstrates how the emergency protocol system retrieves and delivers life-saving medical guidance.

**Key Concepts:**
- Protocol retrieval from database
- Medical authority validation
- Step-by-step guidance delivery
- Age and context adaptations

**Run:**
```bash
cargo run --example emergency_protocol_example
```

**Use Cases:**
- Understanding protocol structure
- Testing protocol modifications
- Validating medical authority compliance

---

### 2. Voice Activation System (`voice_activation_example.rs`)

Shows how voice recognition detects emergency phrases and activates response protocols.

**Key Concepts:**
- On-device speech recognition
- Emergency phrase detection
- Privacy-preserving audio processing
- Performance optimization

**Run:**
```bash
cargo run --example voice_activation_example
```

**Use Cases:**
- Voice interface integration
- Audio pipeline optimization
- Privacy compliance validation

---

### 3. Blockchain Rewards System (`blockchain_rewards_example.rs`)

Illustrates how BONK and SKR token rewards incentivize emergency preparedness training.

**Key Concepts:**
- Reward calculation algorithms
- Performance multipliers
- Solana blockchain integration
- Mobile Wallet Adapter usage

**Run:**
```bash
cargo run --example blockchain_rewards_example
```

**Use Cases:**
- Token economics understanding
- Reward system customization
- Blockchain transaction flow

---

## Running Examples

### Prerequisites

All examples require:
- Rust toolchain (stable)
- Cargo build system
- Dependencies from `Cargo.toml`

### Basic Execution

```bash
# Run a single example
cargo run --example <example_name>

# Example with specific features
cargo run --example emergency_protocol_example --features voice

# Run all examples (if configured)
cargo run --examples
```

### With Logging

Enable detailed logging to see internal operations:

```bash
RUST_LOG=debug cargo run --example emergency_protocol_example
```

---

## Example Code Structure

Each example follows this pattern:

```rust
// 1. Imports
use solana_sos::public::types::*;
use solana_sos::public::emergency_interface::*;

// 2. Main function with demonstrations
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Example scenarios
    demonstrate_feature_1()?;
    demonstrate_feature_2()?;
    Ok(())
}

// 3. Individual demonstration functions
fn demonstrate_feature_1() -> Result<()> {
    // Clear explanation of what's being demonstrated
    // Working code example
    // Expected results
    Ok(())
}

// 4. Test cases
#[cfg(test)]
mod tests {
    #[test]
    fn test_example_functionality() {
        // Validation tests
    }
}
```

---

## Integration with Tests

Examples can also serve as integration tests:

```bash
# Run example as test
cargo test --example emergency_protocol_example

# Run all example tests
cargo test --examples
```

---

## Extending Examples

### Adding a New Example

1. Create new file: `examples/my_new_example.rs`
2. Follow the standard structure above
3. Add documentation at the top explaining the example
4. Update this README with the new example

### Example Template

```rust
// My New Feature Example
//
// This example demonstrates [feature description]

use solana_sos::public::types::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("My New Feature Example");
    println!("======================\n");

    demonstrate_my_feature()?;

    Ok(())
}

fn demonstrate_my_feature() -> Result<(), Box<dyn std::error::Error>> {
    println!("Demonstrating: [feature name]");
    
    // Your demonstration code here
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_my_feature() {
        // Your test code here
    }
}
```

---

## Common Patterns

### Error Handling

All examples use idiomatic Rust error handling:

```rust
// Using Result types
let result = some_operation()?;

// Pattern matching for specific errors
match some_operation() {
    Ok(value) => {
        // Handle success
    }
    Err(e) => {
        println!("Error: {}", e);
        // Handle error
    }
}
```

### Async Operations

Examples requiring async operations use `tokio`:

```rust
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let result = async_operation().await?;
    Ok(())
}
```

### Resource Cleanup

Examples properly clean up resources:

```rust
fn demonstrate_feature() -> Result<()> {
    let resource = acquire_resource()?;
    
    // Use resource
    
    // Explicit cleanup (if needed)
    drop(resource);
    
    Ok(())
}
```

---

## Performance Considerations

Examples are designed for clarity, not performance. For production:

- Add proper error handling
- Implement resource pooling
- Use async/await for I/O operations
- Add appropriate caching
- Follow production configuration

---

## Security Notes

**Important:** These examples use simplified configurations for demonstration purposes. In production:

- Use secure key storage
- Implement proper authentication
- Validate all inputs
- Follow security best practices
- Review security documentation (`docs/SECURITY.md`)

---

## Contributing

When contributing examples:

1. **Clarity First:** Examples should be easy to understand
2. **Working Code:** All examples must compile and run
3. **Documentation:** Include clear explanations
4. **Testing:** Add test cases where appropriate
5. **Real-World:** Demonstrate actual use cases

---

## Troubleshooting

### Example Won't Compile

```bash
# Clean build
cargo clean

# Update dependencies
cargo update

# Rebuild
cargo build --examples
```

### Missing Dependencies

Ensure `Cargo.toml` includes required dependencies:

```toml
[dev-dependencies]
tokio = { version = "1.0", features = ["full"] }
```

### Runtime Errors

Check that:
- Required files exist (models, databases)
- Permissions are correct
- Configuration is valid
- Environment variables are set

---

## Related Documentation

- **API Documentation:** `/docs/API.md`
- **Architecture:** `/docs/ARCHITECTURE.md`
- **Testing Strategy:** `/docs/TESTING_STRATEGY.md`
- **Performance:** `/docs/PERFORMANCE.md`

---

## Questions?

For questions about examples:
- Check API documentation first
- Review related test files in `/tests`
- Open GitHub issue with "examples" label
- Contact: X @paragoner1

---

**Remember:** These examples demonstrate core concepts. Production implementations require additional error handling, security measures, and optimization.


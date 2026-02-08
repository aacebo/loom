# loom-config

Configuration management for the Loom ecosystem.

## Features

- `json` - JSON configuration support
- `yaml` - YAML configuration support
- `toml` - TOML configuration support

## Key Types

### Config

Builder pattern for constructing configuration from multiple sources.

### ConfigSection

Type-safe configuration access with hierarchical paths. Supports `bind()` for deserializing into typed structs.

### Providers

- `MemoryProvider` - In-memory configuration
- `FileProvider` - File-based configuration
- `EnvProvider` - Environment variable configuration

## Macros

- `get!(config, "path.to.value")` - Get string configuration value
- `get!(config, "path", int)` - Get typed value (int, float, bool, value)

## Usage

```toml
[dependencies]
loom-config = { version = "0.0.1", features = ["json"] }
```

```rust
use loom_config::{Config, MemoryProvider, get};

let config = Config::new()
    .with_provider(MemoryProvider::from_pairs([("database.host", "localhost")]))
    .build()
    .unwrap();

let host: Option<&str> = get!(config, "database.host");
let port: Option<i64> = get!(config, "database.port", int);
```

## Navigation

[← Back to Libraries](../README.md)

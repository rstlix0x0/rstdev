# rstdev-config

A `rstdev_config` is a library used to fetch a configuration data
from multiple source and multiple format types

The main purpose of this library to help engineers to maintain their configuration
data. To give flexibility to engineers, it will provide a public trait (interface)
[`SourceParser`] so they should be able to create their own source adapter based on their needs

This library also provide an implementation of [`Format`] that will be able to parse given configuration.
Current supported format types:
- YAML
- TOML
- JSON

## Installation

```toml
[dependencies]
rstdev-config = {version = "0.1.0"}
```

## Expected Outcomes

```rust
use serde::Deserialize;

use rstdev_config::{Builder, ConfigError};
use rstdev_config::source::{from_file, from_env};

#[derive(Deserialize)]
struct Config {

}

fn main() -> Result<(), ConfigError> {
    let cfg_file_path = "./test.toml";
    let cfg_file: Config = Builder::new(from_file(cfg_file_path)).fetch()?.as_toml()?;
    let cfg_env: Config = Builder::new(from_env("PREFIX")).fetch()?.as_env()?;
}
```
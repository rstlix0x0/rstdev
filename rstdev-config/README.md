# rstdev-config

A `rstdev_config` is a library used to fetch a configuration data
from multiple source and multiple format types

The main purpose of this library to help engineers to maintain their configuration
data. To give flexibility to engineers, it will provide a base abstractions 
so they should be able to create their own source adapter and parsers based on their needs

This library also provide an implementation of [`Source`] that will be able to parse given configuration.
Current supported format types:
- YAML
- TOML
- JSON
- Environment variables data types

## Installation

```toml
[dependencies]
rstdev-config = {version = "0.1.3"}
```

## Usages 

```rust
use serde::Deserialize;

use rstdev_config::format::{use_toml, use_env};
use rstdev_config::{Builder, ConfigError};
use rstdev_config::parser::{from_file, from_env};

#[derive(Deserialize)]
struct Config {

}

fn main() -> Result<(), ConfigError> {
    let cfg_file_path = "./test.toml";
    let cfg_file: Config = Builder::new(from_file(cfg_file_path)).fetch()?.parse(use_toml)?;
    let cfg_env: Config = Builder::new(from_env("PREFIX_")).fetch()?.parse(use_env)?;
}
```

## Base Abstractions

```rust
pub trait SourceFormatter<'a, TValue>: Clone {
    fn get_source_value(&'a self) -> TValue;
}

pub trait SourceParser<TFormatter, TValue>
where
    TValue: Clone,
    TFormatter: for<'a> SourceFormatter<'a, TValue>,
{
    fn fetch(&self) -> Result<Source<TFormatter, TValue>, ConfigError>;
}

```

Example implementation of [`types::SourceFormatter`]:

```rust
#[derive(Clone)]
pub struct Value {
    input: String,
}

impl Value {
    pub fn new(input: String) -> Self {
        Self { input }
    }
}

impl<'a> SourceFormatter<'a, String> for Value {
    fn get_source_value(&'a self) -> String {
        self.input.clone()
    }
}

```

Example implementation of [`types::SourceParser`]:

```rust
pub struct File {
    filepath: String,
}

impl File {
    pub fn new(filepath: String) -> Self {
        Self { filepath }
    }
}

impl SourceParser<StringValue, String> for File {
    fn fetch(&self) -> Result<Source<StringValue, String>, ConfigError> {
        let content = fs::read_to_string(self.filepath.clone())
            .map_err(|err| ConfigError::ParseError(err.to_string()))?;

        Ok(Source::new(StringValue::new(content)))
    }
}

```

Example of [`Builder`] using [`parser::from_file`] :

```rust
let cfg: MessageGroup = Builder::new(from_file(toml_file))
    .fetch()?
    .parse(from_toml)?;
```

The `parse` method is a part of [`Source`] public method:

```rust
pub fn parse<F, Out>(&self, cb: F) -> Result<Out, ConfigError>
where
    F: FnOnce(St) -> Result<Out, ConfigError>,
    Out: DeserializeOwned,
{
    cb(self.input.get_source_value())
}
```

the `use_toml`, actually is a callback function that implement `FnOnce(St) -> Result<Out, ConfigError>`:

```rust
pub fn use_toml<In, Out>(input: In) -> Result<Out, ConfigError>
where
    In: ToString,
    Out: DeserializeOwned,
{
    let from_toml = toml::from_str(&input.to_string())
        .map_err(|err| ConfigError::FormatError(err.to_string()))?;

    Ok(from_toml)
}
```
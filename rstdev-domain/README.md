# rstdev-domain

A `rstdev-domain` is a library that only provides base trait abstractions and also
base error types.

Current available traits only for three main blocks :

- `Entity`
- `Aggregate Domain Event`
- `Repository`

> **INFO**
>
> This module used just to provides base trait behaviors and will not giving you
> too many constraints, or even it's almost a zero constraints. By only providing
> trait behaviors, user of this library will still be able to manage their business
> domains 

## Installation

```toml
[dependencies]
rstdev-domain = {version = "0.1.0"}
```
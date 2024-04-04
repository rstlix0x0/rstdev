# rstdev-cache

`rstdev-cache` is a library that provides base cache behavior abstractions to working with common
cache operations.

Base abstractions:

- `StorageTrait`
- `LayerTrait`

Including for common cache policies:

- `InvalidatationPolicyTrait`
- `EvictionPolicyTrait`

Besides of base abstractions, this library will also giving a `RuntimeImplementation` using some common
storage like `Redis` or `Memcache`.

## Installation

```toml
[dependencies]
rstdev-cache = {version = "0.1.0"}
```

## Usages

Example usages

```rust
fn main() {
    let mut cache = cache::runtime::new();
    let mut cache_l1 = cache::runtime::layer::testl1::new();
    cache_l1.add_storage(cache::runtime::storage::redis::new());
    cache_l1.policy().invalidate().add(cache::runtime::policy::invalidate::ttl::new());
    cache_l1.policy().eviction().add(cache::runtime::policy::eviction::lru::new());  
    
    let mut cache_l2 = cache::layer::testl2::new();
    cache_l2.add_storage(cache::runtime::storage::redis::new());
    cache_l2.policy().invalidate().add(cache::runtime::policy::ttl::new());  
    cache_l2.policy().eviction().add(cache::runtime::policy::eviction::random::new());    
    
    // it doesn't have to provides two layers
    // it's okay if we only need a single layer
    //
    // The `layer` concept inspired from the L1/L2 cache 
    cache.add_layer(cache_l1);
    cache.add_layer(cache_l2);
}
```

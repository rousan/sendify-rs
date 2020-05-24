[![Github Actions Status](https://github.com/rousan/sendify-rs/workflows/Test/badge.svg)](https://github.com/rousan/sendify-rs/actions)
[![crates.io](https://img.shields.io/crates/v/sendify.svg)](https://crates.io/crates/sendify)
[![Documentation](https://docs.rs/sendify/badge.svg)](https://docs.rs/sendify)
[![MIT](https://img.shields.io/crates/l/sendify.svg)](./LICENSE)

# sendify-rs

An unsafe crate to wrap a reference to make it [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html). Make sure the reference is still valid when unwrapping it.

[Docs](https://docs.rs/sendify)

## Install

Add this to your `Cargo.toml` file:

```toml
[dependencies]
sendify = "1.0"
```

## Example
 
```rust
use sendify::Sendify;
use std::thread;

fn main() {
    let data = "my string".to_owned();

    let ref_val = &data;
    // Wrap the reference to make it Send + Sync.
    let sendify_val = Sendify::wrap(ref_val);

    thread::spawn(move || {
        // Unwrap the reference, here make sure that reference is still valid otherwise
        // the app might crash.
        let ref_val = unsafe { sendify_val.unwrap() };
        assert_eq!(ref_val, "my string")
    })
    .join()
    .unwrap();

    assert_eq!(data, "my string")
}
```

## Contributing

Your PRs and suggestions are always welcome.

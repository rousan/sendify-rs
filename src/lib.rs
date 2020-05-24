//! An unsafe crate to wrap a reference to make it [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html)
//! to be able to transfer it between threads.
//! Make sure the reference is still valid when unwrapping it.
//!
//! # Examples
//!
//! ```no_run
//! use std::thread;
//!
//! fn main() {
//!     let data = "my string".to_owned();
//!
//!     let ref_val = &data;
//!     // Wrap the reference to make it Send + Sync.
//!     let sendify_val = sendify::wrap(ref_val);
//!
//!     thread::spawn(move || {
//!         // Unwrap the reference, here make sure that reference is still valid otherwise
//!         // the app might crash.
//!         let ref_val = unsafe { sendify_val.unwrap() };
//!         assert_eq!(ref_val, "my string")
//!     })
//!     .join()
//!     .unwrap();
//!
//!     assert_eq!(data, "my string")
//! }
//! ```

pub use self::sendify::Sendify;
pub use self::sendify_mut::SendifyMut;

mod sendify;
mod sendify_mut;

/// Wraps an immutable reference to make it [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html).
pub fn wrap<T>(val: &T) -> Sendify<T> {
    Sendify::wrap(val)
}

/// Wraps a mutable reference to make it [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html).
pub fn wrap_mut<T>(val: &mut T) -> SendifyMut<T> {
    SendifyMut::wrap(val)
}

//! An unsafe crate to wrap a reference to make it [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html).
//! Make sure the reference is still valid when unwrapping it.
//!
//! # Examples
//!
//! ```no_run
//! use sendify::Sendify;
//! use std::thread;

//! fn main() {
//!     let data = "my string".to_owned();
//!
//!     let ref_val = &data;
//!     // Wrap the reference to make it Send + Sync.
//!     let sendify_val = Sendify::wrap(ref_val);
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

mod sendify;

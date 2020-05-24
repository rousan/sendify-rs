/// Wraps an immutable reference to make it [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html).
pub struct Sendify<T> {
    ptr: *const T,
}

impl<T> Sendify<T> {
    /// Wraps an immutable reference to make it [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html).
    pub fn wrap(val: &T) -> Sendify<T> {
        Sendify { ptr: val }
    }

    /// Unwraps the immutable reference. Make sure the reference is still valid while unwrapping it.
    pub unsafe fn unwrap<'a>(self) -> &'a T {
        self.ptr.as_ref().unwrap()
    }
}

unsafe impl<T: Send + 'static> Send for Sendify<T> {}
unsafe impl<T: Sync + 'static> Sync for Sendify<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_sendify() {
        let data = "my string".to_owned();

        let ref_val = &data;
        let sendify_val = Sendify::wrap(ref_val);

        thread::spawn(move || {
            let ref_val = unsafe { sendify_val.unwrap() };
            assert_eq!(ref_val, "my string")
        })
        .join()
        .unwrap();

        assert_eq!(data, "my string")
    }
}

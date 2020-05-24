/// Wraps a mutable reference to make it [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html).
pub struct SendifyMut<T> {
    ptr: *mut T,
}

impl<T> SendifyMut<T> {
    /// Wraps a mutable reference to make it [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html).
    pub fn wrap(val: &mut T) -> SendifyMut<T> {
        SendifyMut { ptr: val }
    }

    /// Unwraps the mutable reference. Make sure the reference is still valid while unwrapping it.
    pub unsafe fn unwrap<'a>(self) -> &'a mut T {
        self.ptr.as_mut().unwrap()
    }
}

unsafe impl<T: Send + 'static> Send for SendifyMut<T> {}
unsafe impl<T: Sync + 'static> Sync for SendifyMut<T> {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_sendify_mut() {
        let mut data = "my string".to_owned();

        let ref_val = &mut data;
        let sendify_val = SendifyMut::wrap(ref_val);

        thread::spawn(move || {
            let ref_val = unsafe { sendify_val.unwrap() };
            ref_val.push_str(" and more");
            assert_eq!(ref_val, "my string and more")
        })
        .join()
        .unwrap();

        assert_eq!(data, "my string and more")
    }
}

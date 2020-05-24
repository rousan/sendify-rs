/// Provides methods to wrap a reference to make it [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html).
pub struct Sendify<T> {
    ptr: Option<*const T>,
    ptr_mut: Option<*mut T>,
}

impl<T> Sendify<T> {
    /// Wraps a immutable reference to make it [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html).
    pub fn wrap(val: &T) -> Sendify<T> {
        Sendify {
            ptr: Some(val),
            ptr_mut: None,
        }
    }

    /// Wraps a mutable reference to make it [`Send`](https://doc.rust-lang.org/nightly/core/marker/trait.Send.html) + [`Sync`](https://doc.rust-lang.org/nightly/core/marker/trait.Sync.html).
    pub fn wrap_mut(val: &mut T) -> Sendify<T> {
        Sendify {
            ptr: None,
            ptr_mut: Some(val),
        }
    }

    /// Unwraps the immutable reference. Make sure the reference is still valid while unwrapping it.
    pub unsafe fn unwrap<'a>(self) -> &'a T {
        if let Some(ptr) = self.ptr {
            return ptr.as_ref().unwrap();
        } else if let Some(ptr) = self.ptr_mut {
            return ptr.as_ref().unwrap();
        }
        panic!("No reference found");
    }

    /// Unwraps the mutable reference. Make sure the reference is still valid while unwrapping it.
    pub unsafe fn unwrap_mut<'a>(self) -> &'a mut T {
        if let Some(ptr) = self.ptr_mut {
            return ptr.as_mut().unwrap();
        }
        panic!("No mutable reference found");
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

    #[test]
    fn test_sendify_mut() {
        let mut data = "my string".to_owned();

        let ref_val = &mut data;
        let sendify_val = Sendify::wrap_mut(ref_val);

        thread::spawn(move || {
            let ref_val = unsafe { sendify_val.unwrap_mut() };
            ref_val.push_str(" and more");
            assert_eq!(ref_val, "my string and more")
        })
        .join()
        .unwrap();

        assert_eq!(data, "my string and more")
    }

    #[test]
    fn test_sendify_mut_as_ref() {
        let mut data = "my string".to_owned();

        let ref_val = &mut data;
        let sendify_val = Sendify::wrap_mut(ref_val);

        thread::spawn(move || {
            let ref_val = unsafe { sendify_val.unwrap() };
            assert_eq!(ref_val, "my string")
        })
        .join()
        .unwrap();

        assert_eq!(data, "my string")
    }
}

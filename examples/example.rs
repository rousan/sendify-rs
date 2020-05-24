use std::thread;

fn main() {
    let data = "my string".to_owned();

    let ref_val = &data;
    // Wrap the reference to make it Send + Sync.
    let sendify_val = sendify::wrap(ref_val);

    thread::spawn(move || {
        // Unwrap the reference, here make sure that reference is still valid otherwise
        // the app might crash.
        let ref_val = unsafe { sendify_val.unwrap() };
        println!("{}", ref_val);
    })
    .join()
    .unwrap();

    println!("My data: {}", data);
}

use std::thread;

fn main() {
    let data = "my string".to_owned();

    let ref_val = &data;
    let sendify_val = sendify::wrap(ref_val);

    thread::spawn(move || {
        let ref_val = unsafe { sendify_val.unwrap() };
        println!("{}", ref_val);
    })
    .join()
    .unwrap();

    println!("My data: {}", data);
}

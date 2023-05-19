
#[no_mangle]
pub extern "C" fn hello_rs() -> i32 {
    println!("Hello from Rust!");
    0
}
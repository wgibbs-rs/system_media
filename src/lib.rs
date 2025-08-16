

#[unsafe(no_mangle)]
pub extern "C" fn rust_hello() {
    println!("Hello from Rust!");
}

unsafe extern "C" {
    fn hello_world();
}



use std::thread;
use std::time::Duration;

pub fn call_swift() {
    unsafe {
        hello_world();
    }

    thread::sleep(Duration::from_secs(10));
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {}
}

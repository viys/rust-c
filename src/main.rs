extern "C" {
    fn printf_hello(name: &str);
}

pub fn call_printf_hello(name: &str) {
    unsafe {
        printf_hello(name);
    }
}

fn main() {
    let msg = "hello";
    // println!("Hello, world!");
    call_printf_hello(msg);
}
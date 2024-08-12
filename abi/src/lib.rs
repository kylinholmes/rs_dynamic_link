

mod import;
use import::prelude::*;

#[no_mangle]
pub extern fn init() {
    unsafe  {
        let thread_id = std::thread::current().id();
        println!("Init, {}, {:?}", add(1, 9), thread_id);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        init();
    }
}

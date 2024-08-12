use std::borrow::Borrow;

use libloading::{Library, Symbol};
use once_cell::sync::Lazy;


static DLL_HANDLE:Lazy<Library> = Lazy::new(|| {
    unsafe {
        let dll = std::env::args().nth(1).expect("No DLL provided");
        Library::new(dll).expect("Failed to load DLL")
    }
});

#[derive(Debug)]
struct Interface<'a> {
    add: Symbol<'a, extern "C" fn(usize, usize) -> usize>,
    init: Symbol<'a, extern "C" fn() -> ()>,
}
impl <'a> Interface<'a> {
    fn new() -> Self {
        unsafe {
            let add: Symbol<extern "C" fn(usize, usize) -> usize> = DLL_HANDLE.borrow().get(b"add").expect("Failed to load add");
            let init: Symbol<extern "C" fn() -> ()> = DLL_HANDLE.borrow().get(b"init").expect("Failed to load init");
            Interface {
                add,
                init
            }
        }
    }
}



fn main() {
    let handle = Interface::new();
    (handle.init)();
    let result = (handle.add)(1, 9);
    println!("Result: {}", result);
    
}
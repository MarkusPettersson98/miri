// This should fail even without validation
// compile-flags: -Zmiri-disable-validation

#![allow(unreachable_code)]

fn main() {
    let y = &5;
    let x: ! = unsafe {
        *(y as *const _ as *const !)  //~ ERROR entering unreachable code
    };
    f(x)
}

fn f(x: !) -> ! { x }

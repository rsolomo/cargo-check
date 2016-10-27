extern crate multitarget;

#[cfg(wrong_bin)]
fn main() {
    println!("{}", &multitarget::helloworld());
}

#[cfg(not(wrong_bin))]
fn main() {
    println!("{}", &multitarget::hello_world());
}

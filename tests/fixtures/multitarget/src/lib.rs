#[cfg(wrong_lib)]
pub fn hello_world() -> String {
    String::rom("Hello, world!")
}

#[cfg(not(wrong_lib))]
pub fn hello_world() -> String {
    String::from("Hello, world!")
}

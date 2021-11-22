use std::env;

fn main() {
    println!("RUSTFLAGS={:?}", env::var_os("RUSTFLAGS"));
    println!("FOO={:?}", env::var_os("FOO"));
}
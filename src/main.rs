#![allow(dead_code)]

enum Instruction {
    ClearDisplay,
    Return,
    Jump(u16),
    Call(u16),
}

fn main() {
    let _memory: [u8; 4096] = [0; 4096];
    println!("Hello, world!");
}

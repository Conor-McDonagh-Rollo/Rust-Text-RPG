mod color;
use color::{self as c, *};
use std::io::{self, stdin, Read};

fn main() {
    cls();
    COMMON::TITLE.print();
    c::printc(String::from("\n\n1) Play\n\n2) Exit\n"), COLOR::WHITE.value());
    let answered: bool = false;
    while !answered {
        let num = c::get_number();
        match num {
            1 => menu(),
            2 => return,
            _ => continue,
        }
    }
    
}

mod Player {
    pub static mut name: &'static str = "John";
}

fn menu() {
    cls();
    c::printc(String::from("Oh hi there!"), COLOR::GREEN.value());
    c::get_number();
    c::printc(String::from("What's your name?"), COLOR::GREEN.value());
}
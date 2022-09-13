use colored::Colorize;
use std::io::{self, stdin, Read};
#[allow(dead_code)]
pub enum COLOR {
    RED,
    GREEN,
    BLUE,
    WHITE
}

impl COLOR {
    pub fn value(&self) -> (u8, u8, u8) {
        match *self {
            COLOR::RED => (255, 0, 0),
            COLOR::GREEN => (0,255,0),
            COLOR::BLUE => (0,0,255),
            COLOR::WHITE => (255,255,255),
        }
    }
}
   

pub fn printc(msg: String, c: (u8, u8, u8) ) {
    println!("{}", msg.truecolor(c.0, c.1, c.2));
}

pub fn cls() {
    print!("\x1B[2J\x1B[1;1H");
}

pub fn get_number() -> u32 {
    let answered: bool = false;
    let final_ans: u32 = 0;
    while !answered {
        let mut ans = String::new();
        stdin()
            .read_line(&mut ans)
            .expect("Unable to read line");
        let ans: u32 = match ans.trim().parse() {
            Ok(ans) => {
                return ans;
            },
            Err(_) => continue,
        };
    }
    final_ans
}

pub enum COMMON {
    TITLE
}

impl COMMON {
    pub fn print(&self) {
        match *self {
            COMMON::TITLE => {
                printc(String::from("_____ _   _ _______ ____    _______ _    _ ______   ______ _____ _____  ______ "), COLOR::RED.value());
                printc(String::from("|_   _| \\ | |__   __/ __ \\  |__   __| |  | |  ____| |  ____|_   _|  __ \\|  ____|"), COLOR::RED.value());
                printc(String::from("  | | |  \\| |  | | | |  | |    | |  | |__| | |__    | |__    | | | |__) | |__  "), COLOR::RED.value());
                printc(String::from("  | | | . ` |  | | | |  | |    | |  |  __  |  __|   |  __|   | | |  _  /|  __|  "), COLOR::RED.value());
                printc(String::from(" _| |_| |\\  |  | | | |__| |    | |  | |  | | |____  | |     _| |_| | \\ \\| |____ "), COLOR::RED.value());
                printc(String::from("|_____|_| \\_|  |_|  \\____/     |_|  |_|  |_|______| |_|    |_____|_|  \\_\\______|"), COLOR::RED.value());
            }
        }
    }
}



 




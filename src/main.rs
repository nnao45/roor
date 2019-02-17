use std::process::Command;

use std::io::{stdin, Read};

/*
fn main() {
    loop {
        let mut character = [0];
        while let Ok(_) = stdin().read(&mut character) {
            println!("CHAR {:?}", character[0] as char);
        }
    }
}*/


fn main() {
    let mut stty_os_flag = "";
    let output = if cfg!(target_os = "darwin") {
        stty_os_flag = "-F"
    } else {
        stty_os_flag = "-f"
    };
    Command::new("stty")
            .args(&[stty_os_flag, "/dev/tty", "cbreak", "min", "1"])
            .output()
            .expect("failed to execute process");
    Command::new("stty")
            .args(&[stty_os_flag, "/dev/tty", "erase", "\n"])
            .output()
            .expect("failed to execute process");
    Command::new("stty")
            .args(&[stty_os_flag, "/dev/tty", "-echo"])
            .output()
            .expect("failed to execute process");
    //std::io::stdin().read_line(&mut s).ok();
    //println!("{}", s);
    loop {
        let mut character = [0];
        while let Ok(_) = stdin().read(&mut character) {
            println!("{:?}", character[0] as char);
        }
    }
    /*Command::new("stty")
            .args(&[stty_os_flag, "/dev/tty", "echo"])
            .output()
            .expect("failed to execute process");*/
}
use std::process::{Command,exit};
use std::env;

fn main() {
    // Get argument(s)
    let args: Vec<String> = env::args().collect();

    // Sanity check argument here, should be integers only
    if args[1].parse::<i32>().is_err() {
        println!("\"{}\" is not an integer.", args[1]);
        exit(1);
    }   

    let pid = &args[1];

    // Run command
    let ps_out = Command::new("ps")
                        .args(&["-o", "etime=", pid])
                        .output()
                        .expect("Failed to execute ps");

    println!("{}", String::from_utf8_lossy(&ps_out.stdout));
    //println!("{}", args[1]);
}

use std::process::{Command,exit};
use std::env;

static HELP: &str = "helptext";

fn main() {
    // Get argument(s)
    let args: Vec<String> = env::args().collect();

    // We require a PID, else exit
    if args.len() < 2 {
        println!("No argument given.");
        exit(1)
    }

    let helparg = String::from("-h");

    // If arg is '-h', show help
    if args.contains(&helparg) {
        println!("{}", HELP);
        exit(0);
    }

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
}

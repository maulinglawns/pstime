use std::process::{Command,exit};
use std::env;

static HELPTEXT: &str = "Usage:\npstime <PID> [-h]";

fn main() {
    // Get argument(s)
    let args: Vec<String> = env::args().collect();

    // We require an argument, else exit
    if args.len() < 2 || args.len() > 2 {
        println!("Expected exactly one argument.\n{}", HELPTEXT);
        exit(1)
    }

    // If arg is '-h', show help
    if args.contains(&String::from("-h")) {
        println!("{}", HELPTEXT);
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

    // If we don't get any output, we probably have a faulty PID
    if String::from_utf8_lossy(&ps_out.stdout).is_empty() {
        println!("Got nothing from PID {}, nonexistent process?", pid);
        exit(1);
    }

    let pstime = String::from_utf8_lossy(&ps_out.stdout);

    // Convert str to vec
    let timevec: Vec<&str> = pstime
        .trim()
        .split(|char| char == '-' || char == ':').collect(); 

    match timevec.len() {
        1 => println!("{} seconds", timevec[0]),
        2 => println!("{} minutes, {} seconds", timevec[0], timevec[1]),
        3 => println!("{} hours, {} minutes, {} seconds", 
                      timevec[0], timevec[1], timevec[2]),
        _ => println!("{} days, {} hours, {} minutes, {} seconds", 
                      timevec[0], timevec[1], timevec[2], timevec[3]),
    }
}

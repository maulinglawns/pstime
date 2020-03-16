use std::process::{Command,exit};
use std::env;

static HELPTEXT: &str = "
Usage:
pstime <PID> [-h]";

fn main() {
    // Get argument(s)
    let args: Vec<String> = env::args().collect();

    // We require an argument, else exit
    if args.len() < 2 {
        println!("No argument given.");
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
        println!("Got nothing from PID {}, does the process exist?", pid);
        exit(0);
    }


    //let timevec: Vec<String> = String::from_utf8_lossy(&ps_out.stdout);
    //println!("{:?}", &ps_out.stdout);

    let pstime = String::from_utf8_lossy(&ps_out.stdout);
    //////////////let pstime = String::from("114-17:33:44");
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
    //println!("{}", timevec_len);
    println!("{}", pstime);
    println!("{:?}", timevec);
    //Parse etime:
    //114-17:33:44
    // See: https://gist.github.com/rust-play/2e6adc87b2cabc91cc4ea95c87cfdc64
}

use std::process::{Command,exit};
use std::env;

fn main() {
    let pid: i32;

    // Get argument(s)
    let args: Vec<String> = env::args().collect();

    // Save PID here
    if args[1].parse::<i32>().is_ok() {
        //pid = args[1].parse::<i32>().unwrap();
        pid = args[1];
        println!("{}", pid);
    } else {
        println!("\"{}\" is not an integer.", args[1]);
        exit(1);
    }

    // Run command
    let ps_out = Command::new("ps")
                        .arg("-o")
                        // Suppress header
                        //.arg("pid=,user=,etime=,cmd=")
                        .arg("etime=")
                        // Supply PID below
                        //.arg(pid)
                        .output()
                        .expect("Failed to execute ps");

    println!("{}", String::from_utf8_lossy(&ps_out.stdout));
    //println!("{}", args[1]);
}

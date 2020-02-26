use std::process::Command;
use std::env;

fn main() {
    // Get argument(s)
    let args: Vec<String> = env::args().collect();
    // Run command
    let ps_out = Command::new("ps")
                        .arg("-o")
                        // Suppress header
                        .arg("pid=,user=,etime=,cmd=")
                        // Supply PID below
                        .arg(&args[1])
                        .output()
                        .expect("Failed to execute ps");

    println!("{}", String::from_utf8_lossy(&ps_out.stdout));
    println!("{}", args[1]);
}

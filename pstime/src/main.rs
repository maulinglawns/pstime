use std::process::Command;

fn main() {
    let ps_out = Command::new("ps")
                        .arg("-o")
                        // Suppress header
                        .arg("pid=,user=,etime=,cmd=")
                        .output()
                        .expect("Failed to execute ps");

    println!("{}", String::from_utf8_lossy(&ps_out.stdout));
}

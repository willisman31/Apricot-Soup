extern crate json;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;
use std::str;

fn main() {
    list_active_cgroups();
}


// query systemd for active cgroups; Podman relies on systemd as the standard
// API to access cgroups which it will use for each container by default.
// The benefit of this approach over running `podman ps -a` is that all user
// namespaces can be queried at once; Pods may appear as a single container,
// but I'll cross that brige when I come to it.
fn list_active_cgroups() {
    let output = Command::new("systemd-cgtop")
                        .arg("-1")
                        .output()
                        .expect("systemd-cgtop command failed to start");
    let io_buffer = match str::from_utf8(&output.stdout) {
        Ok(v) => v,
        Err(e) => panic!("Invalid result from command"),
    };
    println!("{}", io_buffer);
}

#[cfg(test)]
mod tests {
    // TODO
    // this guy should really test his code
    //
}


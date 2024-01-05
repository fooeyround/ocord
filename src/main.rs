use std::process::{Stdio, Command};

fn main() {
    println!("[ocord]: Starting Ocord! (Native Wayland with Ozone)");

    //Open Discord, this requires the `discord` executable
    Command::new("discord")
        .arg("-ozone-platform-hint=auto")
        .arg("--enable-features=UseOzonePlatform")
        .arg("--ozone-platform=wayland")
        .arg("--enable-gpu")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute process");

}




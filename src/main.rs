use std::process::{Stdio, Command};

fn main() {
    println!("[ocord]: Starting Ocord! (Native Wayland)");


    //Open Discord, this requires the `discord` executable
    

    open_discord()
 

}


fn open_discord() {

    Command::new("discord")
        .arg("-ozone-platform-hint=auto")
        .arg("--enable-features=UseOzonePlatform")
        .arg("--ozone-platform=wayland")
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute process");
}

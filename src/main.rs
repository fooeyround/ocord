use std::process::{Stdio, Command};

fn main() {
    println!("[ocord]: Starting Ocord! (Native Wayland with Ozone)");

    //Open Discord, this requires the `discord` executable
    Command::new("discord")
        .arg("-ozone-platform-hint=auto")
        .arg("--enable-features=UseOzonePlatform")
        .arg("--ozone-platform=wayland")
        //Regression: Crashes when opening share screen window.
        //.arg("--enable-features=WebRTCPipeWireCapturer") // Enable Pipewire audio support
        .arg("--enable-webrtc-pipewire-capturer")

        //Hardware acceleration related flags
        .arg("--enable-gpu")
        .arg("--ignore-gpu-blocklist")
        .arg("--enable-features=VaapiVideoDecoder")
        .arg("--use-gl=desktop")
        .arg("--enable-gpu-rasterization")
        .arg("--enable-zero-copy")



        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .output()
        .expect("failed to execute process");

}




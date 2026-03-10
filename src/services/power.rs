use std::process::Command;

pub fn sleep() {
    platform_sleep();
}

#[cfg(target_os = "windows")]
fn platform_sleep() {
    let _ = Command::new("rundll32")
        .args(["powrprof.dll,SetSuspendState", "0,1,0"])
        .status();
}

#[cfg(target_os = "linux")]
fn platform_sleep() {
    let _ = Command::new("systemctl").arg("suspend").status();
}

#[cfg(target_os = "macos")]
fn platform_sleep() {
    let _ = Command::new("pmset").arg("sleepnow").status();
}

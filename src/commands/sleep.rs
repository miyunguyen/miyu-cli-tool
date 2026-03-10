use std::process::Command;

pub fn run() {
    let os = std::env::consts::OS;

    let result: Result<std::process::ExitStatus, std::io::Error> = match os {
        "windows" => Command::new("rundll32")
            .args(["powrprof.dll,SetSuspendState", "0,1,0"])
            .status(),

        "macos" => Command::new("pmset").arg("sleepnow").status(),

        "linux" => Command::new("systemctl").arg("suspend").status(),

        _ => {
            println!("Unsupported OS");
            return;
        }
    };

    match result {
        Ok(_) => println!("Sleep command executed"),
        Err(e) => eprintln!("Failed: {}", e),
    }
}

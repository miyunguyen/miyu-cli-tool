use crate::services::power;

pub fn run() {
    println!("Putting computer to sleep...");
    power::sleep();
}

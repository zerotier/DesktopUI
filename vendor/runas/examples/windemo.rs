use runas;

fn main() {
    println!("Running cmd as root:");
    println!(
        "Status: {}",
        runas::Command::new("cmd")
            .status()
            .expect("failed to execute")
    );
}

use runas;

fn main() {
    println!("Running id as root:");
    println!(
        "Status: {}",
        runas::Command::new("touch")
            .arg("/tmp/test.foo")
            .force_prompt(false)
            .status()
            .expect("failed to execute")
    );
}

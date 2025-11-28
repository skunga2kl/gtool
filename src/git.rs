use std::process::Command;

pub fn run_git(args: &[&str]) {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("failed to run git");

    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
}

pub fn run_status() {
    let output = Command::new("git")
        .args(&["status", "--short", "--branch"])
        .output()
        .expect("failed to run git");

    let text = String::from_utf8_lossy(&output.stdout);
    println!("{}", text);
}

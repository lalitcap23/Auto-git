use std::process::{Command, exit};
use names::Generator;

fn update_commit() {
    let add_command = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("Failed to execute the git add command");

    if !add_command.status.success() {
        println!("Error: Failed to add the files to the remote repo.");
        exit(1);
    }

    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(name_generator())
        .output()
        .expect("Failed to execute the commit command");

    if !commit_command.status.success() {
        println!("Error: Failed to commit the file.");
        exit(1);
    }

    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("master")
        .output()
        .expect("Failed to push the changes");

    if !push_command.status.success() {
        println!("Error: Failed to push the file.");
        exit(1);
    }

    println!("Successfully pushed all changes to the remote repository.");
}

fn name_generator() -> String {
    let mut generator = Generator::default();
    generator.next().unwrap_or_else(|| "DefaultCommit".to_string())
}

fn main() {
    update_commit();
}

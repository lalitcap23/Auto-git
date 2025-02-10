use std::process::{Command, exit};
use std::env;
use reqwest::blocking::Client;
use serde_json::json;

fn update_commit() {
    // Add all files
    if !run_git_command(&["add", "."]) {
        println!("❌ Error: Failed to add files.");
        exit(1);
    }

    // Generate meaningful commit message
    let commit_message = generate_commit_message();

    // Commit changes
    if !run_git_command(&["commit", "-m", &commit_message]) {
        println!("❌ Error: Failed to commit changes.");
        exit(1);
    }

    // Get current branch dynamically
    let branch = get_current_branch().unwrap_or_else(|| "main".to_string());

    // Push changes
    if !run_git_command(&["push", "origin", &branch]) {
        println!("❌ Error: Failed to push changes.");
        exit(1);
    }

    println!("✅ Successfully pushed changes to remote repository!");
}

// Helper function to execute Git commands
fn run_git_command(args: &[&str]) -> bool {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("❌ Failed to execute Git command");

    if !output.status.success() {
        eprintln!("❌ Error: {:?}", String::from_utf8_lossy(&output.stderr));
    }

    output.status.success()
}

// Generate commit message based on Git diff
fn generate_commit_message() -> String {
    let api_key = match env::var("Gemini_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            println!("❌ Error: Set GEMINI_API_KEY environment variable.");
            return "Updated files".to_string();
        }
    };

    let output = Command::new("git")
        .arg("diff")
        .arg("--staged")
        .output()
        .expect("❌ Failed to get git diff");

    let diff_output = String::from_utf8_lossy(&output.stdout);

    if diff_output.is_empty() {
        return "Minor updates".to_string();
    }

    let prompt = format!("Generate a clear, concise Git commit message for the following changes:\n\n{}", diff_output);

    let client = Client::new();
    let response = client.post(&format!("https://generativelanguage.googleapis.com/v1beta/models/gemini-pro:generateText?key={}",api_key))
        .json(&json!({
            "contents": [{
                "parts": [{
                    "text": prompt
                }]
            }]
        }))
        .send();

    match response {
        Ok(resp) => {
            let json_resp: serde_json::Value = resp.json().unwrap_or_else(|_| json!({}));
            json_resp["candidates"]
                .get(0)
                .and_then(|c| c["content"]["parts"][0]["text"].as_str())
                .unwrap_or("Updated files")
                .to_string()
        }
        Err(_) => {
            println!("❌ Error: Failed to get commit message from Gemini API.");
            "Updated files".to_string()
        }
    }
}

// Get the current Git branch
fn get_current_branch() -> Option<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .ok()?;

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

fn main() {
    update_commit();
    println!("Remove .env from version control");
")
}

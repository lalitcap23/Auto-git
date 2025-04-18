use std::process::{Command, exit};
use std::env;
use reqwest::blocking::Client;
use serde_json::json;
use dotenv::dotenv;

fn main() {
    dotenv().ok();

    if env::var("GEMINI_API_KEY").is_err() {
        println!("❌ Error: Set GEMINI_API_KEY environment variable.");
        exit(1);
    }

    update_commit();
}

fn update_commit() {
    // Stage all changes
    if !run_git_command(&["add", "."]) {
        println!("❌ Error: Failed to add files.");
        exit(1);
    }

    // Check if anything is staged
    if !has_staged_changes() {
        println!("❌ No changes staged. Nothing to commit.");
        exit(1);
    }

    // Generate commit message
    let commit_message = generate_commit_message();
    println!("📦 Commit message: {}", commit_message);

    // Commit
    if !run_git_command(&["commit", "-m", &commit_message]) {
        println!("❌ Error: Failed to commit changes.");
        exit(1);
    }

    // Get current branch
    let branch = get_current_branch().unwrap_or_else(|| "main".to_string());

    // Push changes
    if !run_git_command(&["push", "origin", &branch]) {
        println!("❌ Error: Failed to push changes.");
        exit(1);
    }

    println!("✅ Successfully pushed changes to remote repository!");
}

fn has_staged_changes() -> bool {
    Command::new("git")
        .args(["diff", "--cached", "--quiet"])
        .status()
        .map(|status| !status.success()) // true = has changes
        .unwrap_or(false)
}

fn run_git_command(args: &[&str]) -> bool {
    let output = Command::new("git")
        .args(args)
        .output()
        .expect("❌ Failed to execute Git command");

    if !output.status.success() {
        eprintln!("❌ Error: {:?}", String::from_utf8_lossy(&output.stderr));
        return false;
    }
    true
}

fn generate_commit_message() -> String {
    let api_key = match env::var("GEMINI_API_KEY") {
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
        .expect("❌ Failed to get Git diff");

    let diff_output = String::from_utf8_lossy(&output.stdout);

    if diff_output.trim().is_empty() {
        return "Minor updates".to_string();
    }

    let prompt = format!(
        "Generate a clear, concise Git commit message for the following changes:\n\n{}",
        diff_output
    );

    let client = Client::new();
    let response = client
        .post(&format!(
            "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key={}",
            api_key
        ))
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
            if !resp.status().is_success() {
                println!("❌ Error: Gemini API returned status code {}", resp.status());
                return "Updated files".to_string();
            }

            let json_resp: serde_json::Value = resp.json().unwrap_or_else(|_| json!({}));
            json_resp["candidates"]
                .get(0)
                .and_then(|c| c["content"]["parts"][0]["text"].as_str())
                .unwrap_or("Updated files")
                .to_string()
        }
        Err(err) => {
            println!("❌ Error: Failed to get commit message from Gemini API: {}", err);
            "Updated files".to_string()
        }
    }
}

fn get_current_branch() -> Option<String> {
    let output = Command::new("git")
        .arg("rev-parse")
        .arg("--abbrev-ref")
        .arg("HEAD")
        .output()
        .ok()?;

    Some(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

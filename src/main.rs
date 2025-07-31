use dotenv::dotenv;
use reqwest::blocking::Client;
use serde_json::json;
use std::env;
use std::process::{exit, Command};

fn main() {
    dotenv().ok();

    if env::var("GEMINI_API_KEY").is_err() {
        println!("❌ Error: Set GEMINI_API_KEY environment variable.");
        exit(1);
    }

    update_commit();
}

fn update_commit() {
    // Add only tracked files and respect .gitignore
    if !smart_add_files() {
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

fn smart_add_files() -> bool {
    // First, add all currently tracked files that have been modified
    if !run_git_command(&["add", "-u"]) {
        return false;
    }

    // Then add any new files that aren't ignored by .gitignore
    // This is safer than "git add ." as it respects .gitignore rules
    let output = Command::new("git")
        .args(["ls-files", "--others", "--exclude-standard"])
        .output();

    match output {
        Ok(result) => {
            if result.status.success() {
                let files = String::from_utf8_lossy(&result.stdout);
                for file in files.lines() {
                    let file = file.trim();
                    if !file.is_empty() {
                        // Add each untracked file individually
                        run_git_command(&["add", file]);
                    }
                }
            }
        }
        Err(_) => {
            // Fallback to adding all files if the command fails
            return run_git_command(&["add", "."]);
        }
    }

    true
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

    if !output.status.success() {
        println!(
            "❌ Error: Failed to get git diff: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        return "Updated files".to_string();
    }

    let diff_output = String::from_utf8_lossy(&output.stdout);

    if diff_output.trim().is_empty() {
        return "Minor updates".to_string();
    }

    let prompt = format!(
        "Generate a concise Git commit message (max 50 characters) for these changes. Follow conventional commit format if applicable (feat:, fix:, docs:, etc.). Only return the commit message, nothing else:\n\n{}",
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
                println!(
                    "❌ Error: Gemini API returned status code {}",
                    resp.status()
                );
                return "Updated files".to_string();
            }

            match resp.json::<serde_json::Value>() {
                Ok(json_resp) => {
                    if let Some(text) = json_resp["candidates"]
                        .get(0)
                        .and_then(|c| c["content"]["parts"][0]["text"].as_str())
                    {
                        text.trim().to_string()
                    } else {
                        println!("❌ Warning: Unexpected API response format");
                        "Updated files".to_string()
                    }
                }
                Err(err) => {
                    println!("❌ Error: Failed to parse API response: {}", err);
                    "Updated files".to_string()
                }
            }
        }
        Err(err) => {
            println!(
                "❌ Error: Failed to get commit message from Gemini API: {}",
                err
            );
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

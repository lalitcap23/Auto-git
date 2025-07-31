#!/usr/bin/env node

const { execSync, spawn } = require('child_process');
const axios = require('axios');
const fs = require('fs');
const path = require('path');
require('dotenv').config();

const API_KEY = process.env.GEMINI_API_KEY;

// Colors for console output
const colors = {
  red: '\x1b[31m',
  green: '\x1b[32m',
  yellow: '\x1b[33m',
  blue: '\x1b[34m',
  reset: '\x1b[0m'
};

function log(message, color = 'reset') {
  console.log(`${colors[color]}${message}${colors.reset}`);
}

function runGitCommand(args) {
  try {
    const result = execSync(`git ${args.map(arg => `"${arg}"`).join(' ')}`, { 
      encoding: 'utf8',
      stdio: ['pipe', 'pipe', 'pipe']
    });
    return { success: true, output: result };
  } catch (error) {
    return { 
      success: false, 
      error: error.message,
      stderr: error.stderr || error.message
    };
  }
}

function hasGitRepository() {
  const result = runGitCommand(['rev-parse', '--git-dir']);
  return result.success;
}

function hasStagedChanges() {
  const result = runGitCommand(['diff', '--cached', '--quiet']);
  return !result.success; // git diff --quiet returns 1 if there are differences
}

function getCurrentBranch() {
  const result = runGitCommand(['rev-parse', '--abbrev-ref', 'HEAD']);
  return result.success ? result.output.trim() : 'main';
}

async function generateCommitMessage() {
  if (!API_KEY) {
    log('❌ Error: GEMINI_API_KEY environment variable not set', 'red');
    return 'Updated files';
  }

  const diffResult = runGitCommand(['diff', '--staged']);
  if (!diffResult.success) {
    log('❌ Error: Failed to get git diff', 'red');
    return 'Updated files';
  }

  const diffOutput = diffResult.output.trim();
  if (!diffOutput) {
    return 'Minor updates';
  }

  const prompt = `Generate a concise Git commit message (max 50 characters) for these changes. Follow conventional commit format if applicable (feat:, fix:, docs:, etc.). Only return the commit message, nothing else:\n\n${diffOutput}`;

  try {
    log('🤖 Generating commit message with AI...', 'blue');
    
    const response = await axios.post(
      `https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key=${API_KEY}`,
      {
        contents: [{
          parts: [{
            text: prompt
          }]
        }]
      },
      {
        headers: {
          'Content-Type': 'application/json'
        },
        timeout: 30000 // 30 second timeout
      }
    );

    if (response.data && response.data.candidates && response.data.candidates[0]) {
      const text = response.data.candidates[0].content.parts[0].text;
      return text.trim();
    } else {
      log('❌ Warning: Unexpected API response format', 'yellow');
      return 'Updated files';
    }
  } catch (error) {
    log(`❌ Error: Failed to get commit message from Gemini API: ${error.message}`, 'red');
    return 'Updated files';
  }
}

async function main() {
  log('🚀 Auto-git: Automated Git workflow with AI', 'blue');

  // Check if we're in a git repository
  if (!hasGitRepository()) {
    log('❌ Error: Not in a Git repository', 'red');
    process.exit(1);
  }

  // Stage changes intelligently (respecting .gitignore)
  log('📁 Staging changes...', 'yellow');
  if (!smartAddFiles()) {
    log('❌ Error: Failed to stage files', 'red');
    process.exit(1);
  }

  // Check if there are staged changes
  if (!hasStagedChanges()) {
    log('❌ No changes staged. Nothing to commit.', 'yellow');
    process.exit(1);
  }

  // Generate commit message
  const commitMessage = await generateCommitMessage();
  log(`📦 Commit message: ${commitMessage}`, 'green');

  // Commit changes
  log('💾 Committing changes...', 'yellow');
  const commitResult = runGitCommand(['commit', '-m', commitMessage]);
  if (!commitResult.success) {
    log('❌ Error: Failed to commit changes', 'red');
    log(commitResult.stderr, 'red');
    process.exit(1);
  }

  // Get current branch
  const branch = getCurrentBranch();
  
  // Push changes
  log(`🚀 Pushing to origin/${branch}...`, 'yellow');
  const pushResult = runGitCommand(['push', 'origin', branch]);
  if (!pushResult.success) {
    log('❌ Error: Failed to push changes', 'red');
    log(pushResult.stderr, 'red');
    
    // Check if it's because no remote is configured
    if (pushResult.stderr && pushResult.stderr.includes('No such remote')) {
      log('💡 Tip: Make sure you have a remote repository configured:', 'blue');
      log('   git remote add origin <your-repo-url>', 'blue');
    }
    process.exit(1);
  }

  log('✅ Successfully pushed changes to remote repository!', 'green');
}

// Handle Ctrl+C gracefully
process.on('SIGINT', () => {
  log('\n❌ Operation cancelled by user', 'yellow');
  process.exit(0);
});

// Run the main function
main().catch(error => {
  log(`❌ Unexpected error: ${error.message}`, 'red');
  process.exit(1);
});

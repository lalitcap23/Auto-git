# 🚀 auto-git-ai

[![npm version](https://img.shields.io/npm/v/auto-git-ai.svg)](https://www.npmjs.com/package/auto-git-ai)
[![npm downloads](https://img.shields.io/npm/dm/auto-git-ai.svg)](https://www.npmjs.com/package/auto-git-ai)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Automated Git workflow with AI-generated commit messages using Google Gemini API**

🔗 **NPM Package**: https://www.npmjs.com/package/auto-git-ai

This CLI tool automates your Git workflow by:

- Smart staging of changes (respects `.gitignore`)
- Generating meaningful commit messages using AI
- Committing with the AI-generated message
- Pushing to your remote repository

## 🎯 Features

- **🤖 AI-Powered Commit Messages**: Uses Google's Gemini AI to analyze your changes and generate descriptive commit messages
- **⚡ One Command Workflow**: Single command to stage, commit, and push
- **🎨 Beautiful Output**: Colorful console output with clear status messages
- **🛡️ Error Handling**: Comprehensive error handling with helpful tips
- **📝 Conventional Commits**: Follows conventional commit format when applicable

## � Quick Start

1. **Install globally:**

   ```bash
   npm install -g auto-git-ai
   ```

2. **Get Gemini API key from [Google AI Studio](https://aistudio.google.com/app/apikey)**

3. **Set environment variable:**

   ```bash
   export GEMINI_API_KEY="your-api-key-here"
   ```

4. **Use in any Git repository:**
   ```bash
   auto-git
   ```

## 🚀 Usage

### Command Line Interface

Run in any Git repository with a remote configured:

```bash
auto-git
```

### Example Output

```
🚀 Auto-git: Automated Git workflow with AI
📁 Staging changes...
🤖 Generating commit message with AI...
📦 Commit message: feat: add user authentication and login validation
💾 Committing changes...
🚀 Pushing to origin/main...
✅ Successfully pushed changes to remote repository!
```

### What It Does Automatically

1. **Smart Staging**: Stages modified tracked files and new files (respects `.gitignore`)
2. **AI Analysis**: Analyzes your changes using Google's Gemini AI
3. **Commit Generation**: Creates meaningful commit messages following conventional format
4. **Auto Push**: Commits and pushes to your current branch

## 🔧 Configuration

### Environment Variables

**Required:**

- `GEMINI_API_KEY` - Your Google Gemini API key

**Setup Options:**

**Option 1: Global Environment Variable**

```bash
export GEMINI_API_KEY="your-api-key-here"
```

**Option 2: Project .env file**
Create a `.env` file in your project root:

```
GEMINI_API_KEY=your-api-key-here
```

## 📋 Requirements

- **Node.js**: Version 14.0.0 or higher
- **Git**: Properly configured with a remote repository
- **Gemini API Key**: Valid Google Gemini API key from [Google AI Studio](https://aistudio.google.com/app/apikey)
- **Internet Connection**: Required for AI API calls

## 🎨 AI Commit Messages

The AI generates commit messages with these characteristics:

- **Smart Analysis**: Analyzes your actual code changes
- **Conventional Format**: Uses `feat:`, `fix:`, `docs:`, etc. when applicable
- **Concise**: Maximum 50 characters for readability
- **Meaningful**: Describes what the changes actually do
- **Fallback**: Uses "Updated files" if AI is unavailable

## ⚠️ Error Handling & Troubleshooting

### Common Issues

**"GEMINI_API_KEY environment variable not set"**

```bash
export GEMINI_API_KEY="your-api-key-here"
```

**"Not in a Git repository"**

- Navigate to a directory containing a Git repository

**"No changes staged. Nothing to commit."**

- Make some changes to your files first

**"No such remote 'origin'"**

```bash
git remote add origin <your-repo-url>
```

**"Failed to push changes"**

- Ensure you have push permissions to the remote repository
- Check if your branch exists on the remote

## 🔐 Security & Privacy

- ✅ API key is read from environment variables only
- ✅ No sensitive data is logged or stored locally
- ✅ Uses your existing Git credentials and configuration
- ✅ Only sends git diff data to Gemini API for commit message generation

## 📄 License

**MIT License**

```
MIT License

Copyright (c) 2025 Lalit

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.
```

## 🤝 Contributing

Contributions are welcome! Please feel free to:

- 🐛 Report bugs
- 💡 Suggest new features
- 🔧 Submit pull requests
- 📖 Improve documentation

## 🔗 Links

- 📦 **NPM Package**: https://www.npmjs.com/package/auto-git-ai
- 🔑 **Get API Key**: https://aistudio.google.com/app/apikey
- 📝 **Conventional Commits**: https://www.conventionalcommits.org/
- 🤖 **Google Gemini AI**: https://ai.google.dev/

---

**Made with ❤️ by Lalit** | **Powered by Google Gemini AI**

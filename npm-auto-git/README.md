# auto-git-ai

🚀 **Automated Git workflow with AI-generated commit messages using Google Gemini API**

This CLI tool automates your Git workflow by:

- Staging all changes (`git add .`)
- Generating meaningful commit messages using AI
- Committing with the AI-generated message
- Pushing to your remote repository

## 🎯 Features

- **🤖 AI-Powered Commit Messages**: Uses Google's Gemini AI to analyze your changes and generate descriptive commit messages
- **⚡ One Command Workflow**: Single command to stage, commit, and push
- **🎨 Beautiful Output**: Colorful console output with clear status messages
- **🛡️ Error Handling**: Comprehensive error handling with helpful tips
- **📝 Conventional Commits**: Follows conventional commit format when applicable

## 📦 Installation

### Global Installation (Recommended)

```bash
npm install -g auto-git-ai
```

### Local Installation

```bash
npm install auto-git-ai
```

## 🔧 Setup

1. **Get a Gemini API Key**:

   - Visit [Google AI Studio](https://aistudio.google.com/app/apikey)
   - Create a new API key

2. **Set Environment Variable**:

   **Option 1: Global Environment Variable**

   ```bash
   export GEMINI_API_KEY="your-api-key-here"
   ```

   **Option 2: Project .env file**
   Create a `.env` file in your project root:

   ```
   GEMINI_API_KEY=your-api-key-here
   ```

## 🚀 Usage

Run in any Git repository with a remote configured:

```bash
auto-git
```

### Example Output

```
🚀 Auto-git: Automated Git workflow with AI
📁 Staging all changes...
🤖 Generating commit message with AI...
📦 Commit message: feat: add user authentication and login validation
💾 Committing changes...
🚀 Pushing to origin/main...
✅ Successfully pushed changes to remote repository!
```

## 📋 Requirements

- **Node.js**: Version 14.0.0 or higher
- **Git**: Properly configured with a remote repository
- **Gemini API Key**: Valid Google Gemini API key
- **Internet Connection**: Required for AI API calls

## 🔍 What It Does

1. **Checks Git Repository**: Verifies you're in a valid Git repository
2. **Stages Changes**: Runs `git add .` to stage all modifications
3. **Analyzes Changes**: Gets the diff of staged changes
4. **Generates Commit Message**: Sends diff to Gemini AI for analysis
5. **Commits**: Creates commit with AI-generated message
6. **Pushes**: Pushes to the current branch on origin

## ⚠️ Error Handling

The tool handles common scenarios gracefully:

- **No Git Repository**: Clear error message if not in a Git repo
- **No Changes**: Exits gracefully if nothing to commit
- **No Remote**: Helpful tip to configure remote repository
- **API Failures**: Falls back to generic commit message
- **Network Issues**: Timeout handling and error messages

## 🎨 Commit Message Format

The AI generates commit messages following these guidelines:

- **Length**: Maximum 50 characters
- **Format**: Conventional commit format when applicable
- **Types**: `feat:`, `fix:`, `docs:`, `style:`, `refactor:`, `test:`, `chore:`

## 🔐 Security

- API key is read from environment variables only
- No sensitive data is logged or stored
- Git credentials use your existing Git configuration

## 🐛 Troubleshooting

### "GEMINI_API_KEY environment variable not set"

Make sure you've set the API key as described in the setup section.

### "Not in a Git repository"

Navigate to a directory that contains a Git repository.

### "No such remote 'origin'"

Configure a remote repository:

```bash
git remote add origin <your-repo-url>
```

### "Failed to push changes"

Ensure you have push permissions to the remote repository.

## 📄 License

MIT License - see LICENSE file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## 🔗 Links

- [Google AI Studio](https://aistudio.google.com/app/apikey) - Get your Gemini API key
- [Conventional Commits](https://www.conventionalcommits.org/) - Commit message format
# Test npm version

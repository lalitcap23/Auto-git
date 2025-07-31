# Auto-git

An automated Git workflow CLI tool that uses Google's Gemini AI to generate meaningful commit messages.

## Features

- **Automatic staging**: Runs `git add .` to stage all changes
- **AI-powered commit messages**: Uses Gemini AI to analyze your changes and generate descriptive commit messages
- **Automatic push**: Pushes changes to the current branch on origin
- **Error handling**: Provides clear error messages for common Git issues

## Setup

1. **Get a Gemini API key**:

   - Visit [Google AI Studio](https://aistudio.google.com/app/apikey)
   - Create a new API key

2. **Set environment variable**:

   ```bash
   export GEMINI_API_KEY="your-api-key-here"
   ```

   Or create a `.env` file in your project root:

   ```
   GEMINI_API_KEY=your-api-key-here
   ```

3. **Build the project**:

   ```bash
   cargo build --release
   ```

4. **Install globally** (optional):
   ```bash
   cargo install --path .
   ```

## Usage

Simply run the tool in any Git repository:

```bash
./target/release/Auto-git
```

Or if installed globally:

```bash
Auto-git
```

## What it does

1. Stages all changes (`git add .`)
2. Checks if there are staged changes
3. Gets the diff of staged changes
4. Sends the diff to Gemini AI to generate a commit message
5. Commits with the AI-generated message
6. Pushes to the current branch on origin

## Example Output

```
📦 Commit message: feat: add error handling and improve API response parsing
✅ Successfully pushed changes to remote repository!
```

## Requirements

- Rust 1.70+
- Git repository with remote origin configured
- Valid Gemini API key
- Internet connection for API calls

## Error Handling

The tool handles common scenarios:

- No staged changes
- Git command failures
- API errors or timeouts
- Network connectivity issues

In case of API failures, it falls back to a generic "Updated files" commit message.
# Test line for auto-git functionality

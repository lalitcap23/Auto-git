# Publishing auto-git-ai to NPM

## Steps to Publish Your Package

### 1. Create NPM Account

```bash
npm adduser
# Follow the prompts to create an account
```

### 2. Login to NPM

```bash
npm login
```

### 3. Update Package Name (if needed)

The package name `auto-git-ai` might be taken. Check availability:

```bash
npm view auto-git-ai
```

If taken, update the name in `package.json`:

```json
{
  "name": "your-unique-package-name",
  ...
}
```

### 4. Test Package Locally

```bash
# Install globally for testing
npm install -g .

# Test the command
auto-git --help
```

### 5. Publish to NPM

```bash
npm publish
```

### 6. Update and Republish

When making changes:

```bash
# Update version
npm version patch  # or minor, major

# Republish
npm publish
```

## Package Structure

```
npm-auto-git/
├── bin/
│   └── auto-git.js          # Main CLI script
├── package.json             # Package configuration
├── README.md               # Documentation
├── .env.example            # Environment example
├── .gitignore              # Git ignore rules
└── setup.sh                # Setup script
```

## Usage After Publishing

Users can install your package with:

```bash
# Global installation
npm install -g auto-git-ai

# Usage
auto-git
```

## Environment Setup for Users

Users need to set their Gemini API key:

```bash
export GEMINI_API_KEY="their-api-key"
```

Or create a `.env` file in their project:

```
GEMINI_API_KEY=their-api-key
```

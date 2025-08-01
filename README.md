# 🚀 auto-git-ai

<div align="center">

[![npm version](https://img.shields.io/npm/v/auto-git-ai.svg?style=for-the-badge)](https://www.npmjs.com/package/auto-git-ai)
[![npm downloads](https://img.shields.io/npm/dm/auto-git-ai.svg?style=for-the-badge)](https://www.npmjs.com/package/auto-git-ai)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg?style=for-the-badge)](https://opensource.org/licenses/MIT)
[![Node Version](https://img.shields.io/node/v/auto-git-ai.svg?style=for-the-badge)](https://www.npmjs.com/package/auto-git-ai)

**🤖 Automated Git workflow with AI-generated commit messages**

_Transform your Git workflow with one command - powered by Google Gemini AI_

### � [**Install from NPM**](https://www.npmjs.com/package/auto-git-ai) • 🔑 [**Get API Key**](https://aistudio.google.com/app/apikey) • 📖 [**Documentation**](#-how-to-use-in-any-project)

```bash
npm install -g auto-git-ai
```

</div>

---

## ⚡ What This Tool Does

Replace this:

```bash
git add . && git commit -m "updated stuff" && git push
```

With this:

```bash
auto-git  # ✨ AI analyzes changes and creates meaningful commits
```

**🎯 Features:** AI commit messages • One-command workflow • Smart staging • Conventional commits

## 🚀 Quick Start

```bash
# 1. Install
npm install -g auto-git-ai

# 2. Set API key (get from: https://aistudio.google.com/app/apikey)
export GEMINI_API_KEY="your-api-key-here"

# 3. Use anywhere
cd your-project
auto-git
```

## 🔍 How It Works

1. **Analyzes** your local changes vs remote repository
2. **Understands** context: file types, functions, patterns
3. **Generates** meaningful commit messages using AI
4. **Commits & pushes** with conventional format (`feat:`, `fix:`, etc.)

```bash
# Example: You add a new function
+ function calculateTotal(items) { ... }

# AI generates:
# feat: implement calculateTotal function for price computation
```

## 📋 Real-World Example

```bash
# 1. Install once globally (from NPM: https://www.npmjs.com/package/auto-git-ai)
npm install -g auto-git-ai

# 2. Work on your project
cd my-awesome-project
git remote add origin https://github.com/username/my-awesome-project.git

# 3. Make some changes
echo "const api = 'https://api.example.com';" > config.js
mkdir components && echo "<Button>Click me</Button>" > components/Button.js

# 4. Run the magic command
auto-git

# 🎭 Watch the AI work:
# 🚀 Auto-git: Automated Git workflow with AI
# 📁 Staging changes...
# 🤖 Generating commit message with AI...
# 📦 Commit message: feat: add API config and Button component
# 💾 Committing changes...
# 🚀 Pushing to origin/main...
# ✅ Successfully pushed changes to remote repository!
```

## 🔍 How It Works - Smart Diff Analysis

The AI doesn't just randomly generate commit messages. Here's the intelligent process:

### 🧠 **Intelligent Analysis Process:**

1. **📊 Compares Local vs Remote** - Analyzes what's different between your local changes and the remote repository
2. **🔍 Reads Your Changes** - Examines the actual code diff to understand what you modified
3. **🎯 Context Understanding** - Identifies file types, function names, and change patterns
4. **📝 Generates Message** - Creates a meaningful commit message based on the actual changes
5. **✅ Follows Standards** - Uses conventional commit format when applicable

### � **Example Analysis:**

```bash
# Your changes:
+ function calculateTotal(items) { return items.reduce((sum, item) => sum + item.price, 0); }
+ export { calculateTotal };
- // TODO: implement calculation logic

# AI Analysis Result:
# � Commit message: feat: implement calculateTotal function for price computation
```

**🚀 Works with any programming language, any project size, anywhere you have Git!**

## � Requirements & Setup

- **Node.js** 14.0.0+
- **Git** repository with remote
- **Gemini API Key** from [Google AI Studio](https://aistudio.google.com/app/apikey)

**Environment Setup:**

```bash
export GEMINI_API_KEY="your-api-key-here"
# OR create .env file: GEMINI_API_KEY=your-api-key-here
```

## 🎨 AI Commit Examples

| **Your Changes**    | **AI Generated Message**                 |
| ------------------- | ---------------------------------------- |
| Added new component | `feat: add UserProfile component`        |
| Fixed null error    | `fix: handle null values in parseUser`   |
| Updated docs        | `docs: update installation instructions` |
| Refactored code     | `refactor: optimize user data queries`   |

## ⚠️ Troubleshooting

**"GEMINI_API_KEY not set"** → `export GEMINI_API_KEY="your-key"`  
**"Not in Git repository"** → Navigate to a Git directory  
**"No remote 'origin'"** → `git remote add origin <url>`

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

## 🔗 Links & Resources

<div align="center">



**[Conventional Commits](https://www.conventionalcommits.org/)** • **[Google Gemini AI](https://ai.google.dev/)**

### 🤝 **Community**

**[Report Issues](https://www.npmjs.com/package/auto-git-ai)** • **[Feature Requests](https://www.npmjs.com/package/auto-git-ai)**

</div>

---

<div align="center">



---
 **Available on NPM**

⭐ _Star this package on NPM if it helps your workflow!_

</div>

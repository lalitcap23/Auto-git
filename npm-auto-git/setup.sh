#!/bin/bash

echo "🚀 Setting up auto-git-ai NPM package..."

# Check if Node.js is installed
if ! command -v node &> /dev/null; then
    echo "❌ Node.js is not installed. Please install Node.js first."
    exit 1
fi

# Check if npm is installed
if ! command -v npm &> /dev/null; then
    echo "❌ npm is not installed. Please install npm first."
    exit 1
fi

echo "✅ Node.js and npm are installed"

# Install dependencies
echo "📦 Installing dependencies..."
npm install

# Make the binary executable
chmod +x bin/auto-git.js

echo "✅ Setup complete!"
echo ""
echo "🔧 Next steps:"
echo "1. Set your Gemini API key:"
echo "   export GEMINI_API_KEY='your-api-key-here'"
echo ""
echo "2. Or create a .env file with:"
echo "   GEMINI_API_KEY=your-api-key-here"
echo ""
echo "3. Test the tool:"
echo "   node bin/auto-git.js"
echo ""
echo "4. Install globally (optional):"
echo "   npm install -g ."
echo ""
echo "🎉 auto-git-ai is ready to use!"

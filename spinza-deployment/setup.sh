#!/bin/bash

# 🚀 Spinza.io Smart Contract Deployment Setup Script
# This script helps you set up GitHub deployment in one command

echo "🎮 Spinza.io Smart Contract Deployment Setup"
echo "============================================="

# Check if git is available
if ! command -v git &> /dev/null; then
    echo "❌ Git is not installed. Please install Git first."
    exit 1
fi

# Check if we're in a git repository
if [ ! -d ".git" ]; then
    echo "📁 Initializing Git repository..."
    git init
fi

echo "📦 Adding all files to git..."
git add .

echo "📝 Creating initial commit..."
git commit -m "Initial Spinza.io smart contract deployment setup" || echo "Commit already exists"

echo ""
echo "🎯 Next Steps:"
echo "============="
echo "1. Create a new repository on GitHub"
echo "2. Add the remote origin:"
echo "   git remote add origin https://github.com/yourusername/your-repo-name.git"
echo "3. Push to GitHub:"
echo "   git push -u origin main"
echo "4. Generate Solana keypair:"
echo "   solana-keygen new --no-bip39-passphrase --outfile temp_key.json"
echo "   cat temp_key.json"
echo "5. Add the keypair to GitHub Secrets (SOLANA_PRIVATE_KEY)"
echo "6. Run the GitHub Actions workflow!"
echo ""
echo "🔗 Full instructions: https://github.com/your-repo/README.md"
echo "✅ Setup complete! Ready for GitHub deployment 🚀"
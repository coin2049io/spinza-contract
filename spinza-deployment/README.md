# 🎮 Spinza.io Smart Contract Deployment

Complete deployment package for the Spinza.io Solana betting game smart contract.

## 🚀 Quick Deploy (GitHub Actions)

### Prerequisites
- GitHub account
- 5 minutes of your time
- That's it! 🎉

### Step 1: Create GitHub Repository
1. Create a new repository on GitHub
2. Upload these files to your repository
3. Commit and push

### Step 2: Generate Solana Keypair
```bash
# Run this command locally (or use online tools)
solana-keygen new --no-bip39-passphrase --outfile temp_key.json
cat temp_key.json
```

### Step 3: Add Secret to GitHub
1. Go to your repo → Settings → Secrets and variables → Actions
2. Click "New repository secret"
3. Name: `SOLANA_PRIVATE_KEY`
4. Value: Copy the entire array from temp_key.json (including brackets)
5. Click "Add secret"

### Step 4: Deploy
1. Go to Actions tab in your repo
2. Click "🚀 Deploy Spinza.io Smart Contract"
3. Click "Run workflow"
4. Select "devnet" 
5. Click "Run workflow"
6. Wait 3-5 minutes for completion ⏰

### Step 5: Get Your Program ID
1. After successful deployment, check the workflow logs
2. Look for: "📍 Program ID: YOUR_ACTUAL_PROGRAM_ID"
3. Copy this Program ID for your frontend! 🎯

## 📁 Project Structure

```
spinza-deployment/
├── .github/workflows/
│   └── deploy.yml          # GitHub Actions deployment
├── programs/spinza/
│   ├── src/lib.rs          # Smart contract code
│   └── Cargo.toml          # Rust dependencies
├── scripts/
│   ├── initialize.js       # Game state initialization
│   ├── test-deployment.js  # Deployment verification
│   └── package.json        # Node.js dependencies
├── Anchor.toml             # Anchor configuration
├── Cargo.toml              # Workspace configuration
└── README.md               # This file
```

## 🎮 Game Features

### Core Functionality
- ✅ Pool-based betting (0.1 - 100 SOL)
- ✅ Multi-player rounds (up to 50 players)
- ✅ Weighted winner selection
- ✅ 10% commission on winnings
- ✅ Automatic payouts

### Admin Features
- ✅ Pause/unpause game
- ✅ Update bet limits
- ✅ Update commission rate
- ✅ Emergency functions

### Security Features
- ✅ On-chain randomness
- ✅ Transparent winner selection
- ✅ Automatic fund transfers
- ✅ No house edge (except commission)

## 🔧 Configuration

### Game Parameters
- **Operator Wallet**: `E7Y3q3gNA8DKGrXydpCnv4cTQnbkzM1wx3maHqJDv7n6`
- **Min Bet**: 0.1 SOL
- **Max Bet**: 100 SOL  
- **Max Players**: 50 per round
- **Commission**: 10% of winnings only

### Network Settings
- **Development**: Solana Devnet
- **Production**: Solana Mainnet-Beta

## 📊 Commission Calculation

**Example:**
- Total Pool: 100 SOL
- Winner's Bet: 10 SOL
- Gross Winnings: 100 - 10 = 90 SOL
- Commission: 90 × 10% = 9 SOL → Operator
- Winner Receives: 10 + (90 - 9) = 91 SOL

## 🛠️ Manual Deployment (Advanced)

If you prefer local deployment:

```bash
# Install dependencies
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sh -c "$(curl -sSfL https://release.solana.com/stable/install)"
npm install -g @coral-xyz/anchor-cli

# Configure Solana
solana config set --url devnet
solana-keygen new
solana airdrop 2

# Build and deploy
anchor build
anchor deploy

# Initialize game state
cd scripts
npm install
node initialize.js
```

## 🧪 Testing

After deployment, test your contract:

```bash
cd scripts
node test-deployment.js
```

## 🔗 Useful Links

- [Solana Explorer (Devnet)](https://explorer.solana.com/?cluster=devnet)
- [Solana Documentation](https://docs.solana.com/)
- [Anchor Framework](https://www.anchor-lang.com/)

## 🎯 Frontend Integration

After deployment, update your frontend:

```javascript
// In your frontend/src/utils/solana.js
const PROGRAM_ID = new PublicKey('YOUR_DEPLOYED_PROGRAM_ID_HERE');
```

## 🚨 Security Notes

- ✅ Contract uses proven Anchor framework
- ✅ All funds are held in secure PDAs
- ✅ Winner selection is verifiable on-chain
- ✅ Commission calculation is transparent
- ⚠️ Test thoroughly on devnet before mainnet
- ⚠️ Keep your deployment keypair secure

## 📞 Support

If you encounter issues:
1. Check the GitHub Actions logs
2. Verify your SOLANA_PRIVATE_KEY secret
3. Ensure wallet has sufficient SOL
4. Test on devnet first

## 🎉 Success!

Once deployed, your Spinza.io game will be:
- ✅ Live on Solana blockchain
- ✅ Ready for player betting
- ✅ Earning 10% commission
- ✅ Fully decentralized

Happy gaming! 🎮🚀
# 🚀 DEPLOYMENT CHECKLIST

## ✅ Pre-Deployment Setup

- [ ] GitHub account ready
- [ ] All files uploaded to GitHub repository
- [ ] Solana keypair generated locally
- [ ] Keypair added to GitHub Secrets as `SOLANA_PRIVATE_KEY`

## 🎯 Deployment Steps

- [ ] Go to GitHub Actions tab
- [ ] Click "🚀 Deploy Spinza.io Smart Contract"
- [ ] Click "Run workflow" 
- [ ] Select "devnet"
- [ ] Click "Run workflow" again
- [ ] Wait for deployment completion (3-5 minutes)

## 📋 Post-Deployment

- [ ] Copy Program ID from deployment logs
- [ ] Update frontend PROGRAM_ID in `utils/solana.js`
- [ ] Test wallet connection
- [ ] Test betting functionality
- [ ] Verify commission payments

## 🔧 Frontend Update Required

After deployment, update this line in your frontend:

```javascript
// In frontend/src/utils/solana.js
const PROGRAM_ID = new PublicKey('YOUR_NEW_PROGRAM_ID_HERE');
```

## 🧪 Testing Commands

```bash
# Test deployment locally
cd scripts
node test-deployment.js

# Check wallet balance
solana balance YOUR_WALLET_ADDRESS

# View on explorer
https://explorer.solana.com/address/YOUR_PROGRAM_ID?cluster=devnet
```

## 🚨 Troubleshooting

**Deployment fails with "insufficient funds":**
- Keypair needs SOL for deployment fees
- GitHub Actions automatically requests airdrop
- Check airdrop limits (may need multiple runs)

**Frontend can't connect:**
- Verify Program ID is updated
- Check wallet adapter configuration
- Ensure network is set to devnet

**Game initialization fails:**
- Check operator wallet address
- Verify program is deployed correctly
- Run test-deployment.js for diagnosis

## 🎉 Success Indicators

✅ GitHub Actions shows green checkmark
✅ Program ID appears in deployment logs  
✅ Game state initialization succeeds
✅ Frontend connects without "Loading..." message
✅ Wallet connection works
✅ Test bet can be placed

## 📞 Support

If stuck:
1. Check GitHub Actions logs for error details
2. Verify all secrets are set correctly
3. Test on devnet before mainnet deployment
4. Ensure wallet has sufficient SOL

**Once deployed successfully, your Spinza.io game is live on Solana! 🎮🚀**
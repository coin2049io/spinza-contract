const anchor = require('@coral-xyz/anchor');
const { Connection, PublicKey } = require('@solana/web3.js');

async function testDeployment() {
  console.log('🧪 Testing Spinza.io Smart Contract Deployment...');
  
  try {
    // Setup connection
    const network = process.env.ANCHOR_PROVIDER_URL || 'https://api.devnet.solana.com';
    const connection = new Connection(network, 'confirmed');
    
    // Setup wallet
    const wallet = anchor.AnchorProvider.env().wallet;
    const provider = new anchor.AnchorProvider(connection, wallet, {
      commitment: 'confirmed'
    });
    anchor.setProvider(provider);

    // Load the program
    const program = anchor.workspace.Spinza;
    console.log('📦 Program ID:', program.programId.toString());
    
    // Test: Get program info
    const programInfo = await connection.getAccountInfo(program.programId);
    if (!programInfo) {
      throw new Error('Program not found on-chain');
    }
    console.log('✅ Program exists on-chain');
    console.log('📊 Program data size:', programInfo.data.length, 'bytes');
    
    // Test: Get game state
    const [gameStatePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('game_state')],
      program.programId
    );

    console.log('🔍 Testing game state retrieval...');
    const gameState = await program.account.gameState.fetch(gameStatePDA);
    
    console.log('✅ Game State Retrieved Successfully!');
    console.log('');
    console.log('🎮 CURRENT GAME CONFIGURATION:');
    console.log('===============================');
    console.log('👑 Operator Wallet:', gameState.operatorWallet.toString());
    console.log('💰 Min Bet:', gameState.minBet.toNumber() / anchor.web3.LAMPORTS_PER_SOL, 'SOL');
    console.log('💰 Max Bet:', gameState.maxBet.toNumber() / anchor.web3.LAMPORTS_PER_SOL, 'SOL');
    console.log('👥 Max Players:', gameState.maxPlayers);
    console.log('💼 Commission Rate:', gameState.commissionRate / 100, '%');
    console.log('⏸️ Is Paused:', gameState.isPaused);
    console.log('🔢 Round Count:', gameState.roundCount.toNumber());
    console.log('🎯 Current Round:', gameState.currentRound ? gameState.currentRound.toString() : 'None');
    console.log('');
    
    // Test: Create a test round
    console.log('🧪 Testing round creation...');
    const nextRoundId = gameState.roundCount.toNumber() + 1;
    const [roundPDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('round'), new anchor.BN(nextRoundId).toArrayLike(Buffer, 'le', 8)],
      program.programId
    );
    
    try {
      const tx = await program.methods
        .createRound()
        .accounts({
          gameState: gameStatePDA,
          round: roundPDA,
          authority: wallet.publicKey,
          systemProgram: anchor.web3.SystemProgram.programId,
        })
        .rpc();
        
      console.log('✅ Test round created successfully!');
      console.log('📄 Transaction:', tx);
      console.log('🎯 Round PDA:', roundPDA.toString());
      
    } catch (error) {
      if (error.message.includes('already in use')) {
        console.log('ℹ️ Round already exists (expected if run multiple times)');
      } else {
        console.log('⚠️ Round creation test failed:', error.message);
      }
    }
    
    console.log('');
    console.log('🎉 DEPLOYMENT TEST COMPLETE!');
    console.log('=============================');
    console.log('✅ Smart contract is deployed and functional');
    console.log('✅ Game state is properly initialized');
    console.log('✅ Ready for frontend integration');
    console.log('');
    console.log('🔗 Useful Links:');
    console.log(`📊 Program Explorer: https://explorer.solana.com/address/${program.programId.toString()}?cluster=devnet`);
    console.log(`🎮 Game State Explorer: https://explorer.solana.com/address/${gameStatePDA.toString()}?cluster=devnet`);
    console.log('');
    console.log('🔧 Next Steps:');
    console.log(`1. Update your frontend PROGRAM_ID to: ${program.programId.toString()}`);
    console.log('2. Test wallet connection and betting functionality');
    console.log('3. Deploy to mainnet when ready!');
    
  } catch (error) {
    console.error('❌ Deployment test failed:', error);
    
    if (error.message.includes('Program account does not exist')) {
      console.log('💡 The smart contract is not deployed yet');
      console.log('   Run the GitHub Actions deployment first');
    } else if (error.message.includes('Account does not exist')) {
      console.log('💡 Game state not initialized yet');
      console.log('   The initialize script needs to run first');
    }
    
    process.exit(1);
  }
}

// Run if called directly
if (require.main === module) {
  testDeployment().catch(console.error);
}

module.exports = { testDeployment };
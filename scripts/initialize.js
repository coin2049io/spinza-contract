const anchor = require('@coral-xyz/anchor');
const { Connection, PublicKey } = require('@solana/web3.js');

async function initialize() {
  console.log('🎯 Initializing Spinza.io Game State...');
  
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
    
    // Configuration
    const operatorWallet = new PublicKey('E7Y3q3gNA8DKGrXydpCnv4cTQnbkzM1wx3maHqJDv7n6');
    const minBet = 0.1 * anchor.web3.LAMPORTS_PER_SOL; // 0.1 SOL
    const maxBet = 100 * anchor.web3.LAMPORTS_PER_SOL; // 100 SOL
    const maxPlayers = 50;
    const commissionRate = 1000; // 10% (in basis points)

    // Find game state PDA
    const [gameStatePDA] = PublicKey.findProgramAddressSync(
      [Buffer.from('game_state')],
      program.programId
    );

    console.log('🔍 Game State PDA:', gameStatePDA.toString());
    console.log('👑 Operator Wallet:', operatorWallet.toString());
    
    // Check if already initialized
    try {
      const existingState = await program.account.gameState.fetch(gameStatePDA);
      console.log('⚠️ Game state already initialized!');
      console.log('📊 Current configuration:', {
        operatorWallet: existingState.operatorWallet.toString(),
        minBet: existingState.minBet.toNumber() / anchor.web3.LAMPORTS_PER_SOL + ' SOL',
        maxBet: existingState.maxBet.toNumber() / anchor.web3.LAMPORTS_PER_SOL + ' SOL',
        maxPlayers: existingState.maxPlayers,
        commissionRate: existingState.commissionRate / 100 + '%',
        isPaused: existingState.isPaused,
        roundCount: existingState.roundCount.toNumber(),
      });
      return;
    } catch (error) {
      // Game state doesn't exist, continue with initialization
      console.log('🆕 Game state not found, initializing...');
    }

    // Initialize the game
    console.log('🚀 Sending initialization transaction...');
    const tx = await program.methods
      .initialize(
        operatorWallet,
        new anchor.BN(minBet),
        new anchor.BN(maxBet),
        maxPlayers,
        commissionRate
      )
      .accounts({
        gameState: gameStatePDA,
        authority: wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log('✅ Game initialized successfully!');
    console.log('📄 Transaction signature:', tx);
    console.log('');
    console.log('🎮 GAME CONFIGURATION:');
    console.log('======================');
    console.log('🎯 Game State PDA:', gameStatePDA.toString());
    console.log('👑 Operator Wallet:', operatorWallet.toString());
    console.log('💰 Min Bet:', minBet / anchor.web3.LAMPORTS_PER_SOL, 'SOL');
    console.log('💰 Max Bet:', maxBet / anchor.web3.LAMPORTS_PER_SOL, 'SOL');
    console.log('👥 Max Players:', maxPlayers);
    console.log('💼 Commission Rate:', commissionRate / 100, '%');
    console.log('');
    console.log('🔗 View on Explorer:');
    console.log(`https://explorer.solana.com/address/${gameStatePDA.toString()}?cluster=devnet`);
    
  } catch (error) {
    console.error('❌ Initialization failed:', error);
    if (error.message.includes('insufficient funds')) {
      console.log('💡 Solution: Fund your wallet with more SOL');
      console.log('   solana airdrop 2');
    } else if (error.message.includes('already in use')) {
      console.log('💡 Game state already exists - this is normal');
    }
    process.exit(1);
  }
}

// Run if called directly
if (require.main === module) {
  initialize().catch(console.error);
}

module.exports = { initialize };
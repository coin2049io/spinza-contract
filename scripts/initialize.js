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
    
    // Skip workspace, load program directly
    const programId = new PublicKey('5gBR963NUrgHLLV6qL7RbMGdpZ4GcLXY3hvjyxrGthEY');
    const idl = require('../target/idl/spinza.json');
    const program = new anchor.Program(idl, programId, provider);
    console.log('📦 Program ID:', program.programId.toString());
    
    // Configuration
    const operatorWallet = new PublicKey('E7Y3q3gNA8DKGrXydpCnv4cTQnbkzM1wx3maHqJDv7n6');
    const minBet = 0.1 * anchor.web3.LAMPORTS_PER_SOL; // 0.1 SOL
    const maxBet = 100 * anchor.web3.LAMPORTS_PER_SOL; // 100 SOL
    const maxPlayers = 50;
    const commissionRate = 1000; // 10% in basis points
    
    console.log('🎯 Initializing with parameters:');
    console.log('- Operator Wallet:', operatorWallet.toString());
    console.log('- Min Bet:', minBet / anchor.web3.LAMPORTS_PER_SOL, 'SOL');
    console.log('- Max Bet:', maxBet / anchor.web3.LAMPORTS_PER_SOL, 'SOL');
    console.log('- Max Players:', maxPlayers);
    console.log('- Commission Rate:', commissionRate / 100, '%');
    
    // Derive game state PDA
    const [gameStatePDA] = await PublicKey.findProgramAddress(
      [Buffer.from('game_state')],
      program.programId
    );
    
    console.log('🎲 Game State PDA:', gameStatePDA.toString());
    
    // Initialize the game
    const tx = await program.methods
      .initialize(operatorWallet, minBet, maxBet, maxPlayers, commissionRate)
      .accounts({
        gameState: gameStatePDA,
        authority: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();
    
    console.log('✅ Game initialized successfully!');
    console.log('📋 Transaction:', tx);
    console.log('🎲 Game State PDA:', gameStatePDA.toString());
    console.log('🎯 Spinza.io is ready for players!');
    
  } catch (error) {
    console.error('❌ Initialization failed:', error);
    throw error;
  }
}

// Run the initialization
initialize().catch(console.error);

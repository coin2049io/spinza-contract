const anchor = require('@coral-xyz/anchor');
const { Connection, PublicKey } = require('@solana/web3.js');
const { execSync } = require('child_process');

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
    
    // Get program ID from the IDL file (most reliable)
    let programId;
    try {
      console.log('🔍 Reading program ID from IDL file...');
      const idl = require('../target/idl/spinza.json');
      const programIdString = idl.programId;
      console.log('🔍 Program ID from IDL:', programIdString);
      
      programId = new PublicKey(programIdString);
      console.log('✅ Valid Program ID created:', programId.toString());
    } catch (error) {
      console.error('❌ Failed to read IDL or create PublicKey:', error.message);
      throw new Error(`Cannot get program ID from IDL: ${error.message}`);
    }
    
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
      programId
    );
    
    console.log('🎲 Game State PDA:', gameStatePDA.toString());
    
    // Create initialize instruction manually
    const borsh = require('borsh');
    
    // Serialize instruction data
    const instructionData = Buffer.concat([
      Buffer.from([175, 175, 109, 31, 13, 152, 155, 237]), // initialize discriminator
      operatorWallet.toBuffer(),
      Buffer.from(minBet.toString(16).padStart(16, '0'), 'hex').reverse(),
      Buffer.from(maxBet.toString(16).padStart(16, '0'), 'hex').reverse(),
      Buffer.from([maxPlayers]),
      Buffer.from(commissionRate.toString(16).padStart(4, '0'), 'hex').reverse(),
    ]);
    
    // Create instruction
    const instruction = new anchor.web3.TransactionInstruction({
      keys: [
        { pubkey: gameStatePDA, isSigner: false, isWritable: true },
        { pubkey: provider.wallet.publicKey, isSigner: true, isWritable: true },
        { pubkey: anchor.web3.SystemProgram.programId, isSigner: false, isWritable: false },
      ],
      programId,
      data: instructionData,
    });
    
    // Send transaction
    const tx = new anchor.web3.Transaction().add(instruction);
    const signature = await provider.sendAndConfirm(tx);
    
    console.log('✅ Game initialized successfully!');
    console.log('📋 Transaction:', signature);
    console.log('🎲 Game State PDA:', gameStatePDA.toString());
    console.log('🎯 Spinza.io is ready for players!');
    
  } catch (error) {
    console.error('❌ Initialization failed:', error);
    throw error;
  }
}

// Run the initialization
initialize().catch(console.error);

use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod spinza {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        operator_wallet: Pubkey,
        min_bet: u64,
        max_bet: u64,
        max_players: u8,
        commission_rate: u16,
    ) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        game_state.operator_wallet = operator_wallet;
        game_state.min_bet = min_bet;
        game_state.max_bet = max_bet;
        game_state.max_players = max_players;
        game_state.commission_rate = commission_rate; // basis points (1000 = 10%)
        game_state.is_paused = false;
        game_state.round_count = 0;
        game_state.current_round = None;
        
        emit!(GameStateInitialized {
            operator_wallet,
            min_bet,
            max_bet,
            max_players,
            commission_rate,
        });

        Ok(())
    }

    pub fn create_round(ctx: Context<CreateRound>) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        let round = &mut ctx.accounts.round;
        
        require!(!game_state.is_paused, GameError::GamePaused);
        require!(game_state.current_round.is_none(), GameError::RoundInProgress);

        game_state.round_count += 1;
        game_state.current_round = Some(round.key());
        
        round.round_id = game_state.round_count;
        round.status = RoundStatus::WaitingForPlayers;
        round.total_pool = 0;
        round.player_count = 0;
        round.created_at = Clock::get()?.unix_timestamp;
        round.winner = None;

        emit!(RoundCreated {
            round_id: round.round_id,
            round_pubkey: round.key(),
        });

        Ok(())
    }

    pub fn place_bet(ctx: Context<PlaceBet>, amount: u64) -> Result<()> {
        let game_state = &ctx.accounts.game_state;
        let round = &mut ctx.accounts.round;
        let player = ctx.accounts.player.key();
        
        require!(!game_state.is_paused, GameError::GamePaused);
        require!(round.status == RoundStatus::WaitingForPlayers || round.status == RoundStatus::Active, GameError::RoundNotActive);
        require!(amount >= game_state.min_bet, GameError::BetTooLow);
        require!(amount <= game_state.max_bet, GameError::BetTooHigh);
        require!(round.player_count < game_state.max_players, GameError::RoundFull);

        // Transfer SOL from player to round vault
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            anchor_lang::system_program::Transfer {
                from: ctx.accounts.player.to_account_info(),
                to: ctx.accounts.round_vault.to_account_info(),
            },
        );
        anchor_lang::system_program::transfer(cpi_context, amount)?;

        // Check if player already has a bet in this round
        let mut player_exists = false;
        let mut total_player_amount = amount;
        
        for i in 0..round.player_count {
            if round.players[i as usize].player == player {
                // Add to existing bet
                round.players[i as usize].bet_amount += amount;
                total_player_amount = round.players[i as usize].bet_amount;
                player_exists = true;
                break;
            }
        }

        if !player_exists {
            // Add new player
            require!(round.player_count < MAX_PLAYERS_PER_ROUND, GameError::RoundFull);
            round.players[round.player_count as usize] = PlayerBet {
                player,
                bet_amount: amount,
            };
            round.player_count += 1;
        }

        round.total_pool += amount;

        // Update round status if we have 2+ unique players
        if round.status == RoundStatus::WaitingForPlayers && round.player_count >= 2 {
            round.status = RoundStatus::Active;
            emit!(RoundActivated {
                round_id: round.round_id,
                player_count: round.player_count,
                total_pool: round.total_pool,
            });
        }

        emit!(BetPlaced {
            round_id: round.round_id,
            player,
            amount,
            total_amount: total_player_amount,
            total_pool: round.total_pool,
            player_count: round.player_count,
        });

        Ok(())
    }

    pub fn resolve_round(ctx: Context<ResolveRound>, random_seed: u64) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        let round = &mut ctx.accounts.round;
        
        require!(round.status == RoundStatus::Active, GameError::RoundNotActive);
        require!(round.player_count >= 2, GameError::NotEnoughPlayers);
        require!(round.total_pool > 0, GameError::EmptyPool);

        // Use random seed to select winner based on weighted probability
        let winner_index = select_weighted_winner(round, random_seed)?;
        let winner = round.players[winner_index].player;
        let winner_bet_amount = round.players[winner_index].bet_amount;
        
        round.winner = Some(winner);
        round.status = RoundStatus::Resolved;
        round.resolved_at = Some(Clock::get()?.unix_timestamp);

        // Calculate commission correctly: 10% of winnings, not total pool
        let gross_winnings = round.total_pool - winner_bet_amount; // What winner gains
        let commission = (gross_winnings * game_state.commission_rate as u64) / 10000;
        let net_winnings = gross_winnings - commission;
        let total_to_winner = winner_bet_amount + net_winnings; // Bet return + net profit

        // Transfer commission to operator
        **ctx.accounts.round_vault.to_account_info().try_borrow_mut_lamports()? -= commission;
        **ctx.accounts.operator_wallet.to_account_info().try_borrow_mut_lamports()? += commission;

        // Transfer total amount to winner (original bet + net winnings)
        **ctx.accounts.round_vault.to_account_info().try_borrow_mut_lamports()? -= total_to_winner;
        **ctx.accounts.winner_wallet.to_account_info().try_borrow_mut_lamports()? += total_to_winner;

        // Clear current round
        game_state.current_round = None;

        emit!(RoundResolved {
            round_id: round.round_id,
            winner,
            winner_bet: winner_bet_amount,
            total_pool: round.total_pool,
            gross_winnings,
            net_winnings,
            commission,
            total_to_winner,
        });

        Ok(())
    }

    // Admin functions
    pub fn pause_game(ctx: Context<AdminAction>) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        require!(ctx.accounts.authority.key() == game_state.operator_wallet, GameError::Unauthorized);
        game_state.is_paused = true;
        emit!(GamePaused {});
        Ok(())
    }

    pub fn unpause_game(ctx: Context<AdminAction>) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        require!(ctx.accounts.authority.key() == game_state.operator_wallet, GameError::Unauthorized);
        game_state.is_paused = false;
        emit!(GameUnpaused {});
        Ok(())
    }

    pub fn update_bet_limits(ctx: Context<AdminAction>, min_bet: u64, max_bet: u64) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        require!(ctx.accounts.authority.key() == game_state.operator_wallet, GameError::Unauthorized);
        require!(min_bet < max_bet, GameError::InvalidBetLimits);
        
        game_state.min_bet = min_bet;
        game_state.max_bet = max_bet;
        
        emit!(BetLimitsUpdated { min_bet, max_bet });
        Ok(())
    }

    pub fn update_commission_rate(ctx: Context<AdminAction>, commission_rate: u16) -> Result<()> {
        let game_state = &mut ctx.accounts.game_state;
        require!(ctx.accounts.authority.key() == game_state.operator_wallet, GameError::Unauthorized);
        require!(commission_rate <= 2000, GameError::CommissionTooHigh); // Max 20%
        
        game_state.commission_rate = commission_rate;
        
        emit!(CommissionRateUpdated { commission_rate });
        Ok(())
    }

    pub fn emergency_withdraw(ctx: Context<EmergencyWithdraw>) -> Result<()> {
        let game_state = &ctx.accounts.game_state;
        require!(ctx.accounts.authority.key() == game_state.operator_wallet, GameError::Unauthorized);
        
        let vault_balance = ctx.accounts.round_vault.to_account_info().lamports();
        
        **ctx.accounts.round_vault.to_account_info().try_borrow_mut_lamports()? = 0;
        **ctx.accounts.operator_wallet.to_account_info().try_borrow_mut_lamports()? += vault_balance;
        
        emit!(EmergencyWithdrawal { amount: vault_balance });
        Ok(())
    }
}

// Helper function to select winner based on weighted probability
fn select_weighted_winner(round: &Round, random_seed: u64) -> Result<usize> {
    if round.player_count == 0 || round.total_pool == 0 {
        return err!(GameError::EmptyPool);
    }

    // Create cumulative probability ranges
    let mut cumulative_weights = Vec::new();
    let mut cumulative_sum = 0u64;
    
    for i in 0..round.player_count {
        cumulative_sum += round.players[i as usize].bet_amount;
        cumulative_weights.push(cumulative_sum);
    }

    // Generate random number in range [0, total_pool)
    let random_value = random_seed % round.total_pool;
    
    // Find winner based on weighted probability
    for (index, &cumulative_weight) in cumulative_weights.iter().enumerate() {
        if random_value < cumulative_weight {
            return Ok(index);
        }
    }

    // Fallback to last player (should not happen with correct logic)
    Ok((round.player_count - 1) as usize)
}

// Account structures
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + GameState::INIT_SPACE,
        seeds = [b"game_state"],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreateRound<'info> {
    #[account(
        mut,
        seeds = [b"game_state"],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    #[account(
        init,
        payer = authority,
        space = 8 + Round::INIT_SPACE,
        seeds = [b"round", game_state.round_count.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, Round>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct PlaceBet<'info> {
    #[account(
        seeds = [b"game_state"],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    #[account(
        mut,
        seeds = [b"round", round.round_id.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, Round>,
    #[account(
        mut,
        seeds = [b"round_vault", round.key().as_ref()],
        bump
    )]
    /// CHECK: Round vault for holding bets
    pub round_vault: AccountInfo<'info>,
    #[account(mut)]
    pub player: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct ResolveRound<'info> {
    #[account(
        mut,
        seeds = [b"game_state"],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    #[account(
        mut,
        seeds = [b"round", round.round_id.to_le_bytes().as_ref()],
        bump
    )]
    pub round: Account<'info, Round>,
    #[account(
        mut,
        seeds = [b"round_vault", round.key().as_ref()],
        bump
    )]
    /// CHECK: Round vault for holding bets
    pub round_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Operator wallet for commission
    pub operator_wallet: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Winner wallet for prize
    pub winner_wallet: AccountInfo<'info>,
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct AdminAction<'info> {
    #[account(
        mut,
        seeds = [b"game_state"],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct EmergencyWithdraw<'info> {
    #[account(
        seeds = [b"game_state"],
        bump
    )]
    pub game_state: Account<'info, GameState>,
    #[account(
        mut,
        seeds = [b"round_vault", round.key().as_ref()],
        bump
    )]
    /// CHECK: Round vault for emergency withdrawal
    pub round_vault: AccountInfo<'info>,
    #[account(mut)]
    /// CHECK: Operator wallet for emergency funds
    pub operator_wallet: AccountInfo<'info>,
    #[account(mut)]
    pub round: Account<'info, Round>,
    pub authority: Signer<'info>,
}

// Data structures
#[account]
#[derive(InitSpace)]
pub struct GameState {
    pub operator_wallet: Pubkey,
    pub min_bet: u64,
    pub max_bet: u64,
    pub max_players: u8,
    pub commission_rate: u16, // basis points (1000 = 10%)
    pub is_paused: bool,
    pub round_count: u64,
    pub current_round: Option<Pubkey>,
}

#[account]
#[derive(InitSpace)]
pub struct Round {
    pub round_id: u64,
    pub status: RoundStatus,
    pub total_pool: u64,
    pub player_count: u8,
    #[max_len(50)]
    pub players: Vec<PlayerBet>,
    pub created_at: i64,
    pub resolved_at: Option<i64>,
    pub winner: Option<Pubkey>,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, PartialEq, Eq, InitSpace)]
pub enum RoundStatus {
    WaitingForPlayers,
    Active,
    Resolved,
    Cancelled,
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, Copy, InitSpace)]
pub struct PlayerBet {
    pub player: Pubkey,
    pub bet_amount: u64,
}

const MAX_PLAYERS_PER_ROUND: u8 = 50;

// Events
#[event]
pub struct GameStateInitialized {
    pub operator_wallet: Pubkey,
    pub min_bet: u64,
    pub max_bet: u64,
    pub max_players: u8,
    pub commission_rate: u16,
}

#[event]
pub struct RoundCreated {
    pub round_id: u64,
    pub round_pubkey: Pubkey,
}

#[event]
pub struct BetPlaced {
    pub round_id: u64,
    pub player: Pubkey,
    pub amount: u64,
    pub total_amount: u64,
    pub total_pool: u64,
    pub player_count: u8,
}

#[event]
pub struct RoundActivated {
    pub round_id: u64,
    pub player_count: u8,
    pub total_pool: u64,
}

#[event]
pub struct RoundResolved {
    pub round_id: u64,
    pub winner: Pubkey,
    pub winner_bet: u64,
    pub total_pool: u64,
    pub gross_winnings: u64,
    pub net_winnings: u64,
    pub commission: u64,
    pub total_to_winner: u64,
}

#[event]
pub struct GamePaused {}

#[event]
pub struct GameUnpaused {}

#[event]
pub struct BetLimitsUpdated {
    pub min_bet: u64,
    pub max_bet: u64,
}

#[event]
pub struct CommissionRateUpdated {
    pub commission_rate: u16,
}

#[event]
pub struct EmergencyWithdrawal {
    pub amount: u64,
}

// Error codes
#[error_code]
pub enum GameError {
    #[msg("Game is currently paused")]
    GamePaused,
    #[msg("Round is not active")]
    RoundNotActive,
    #[msg("Round is already in progress")]
    RoundInProgress,
    #[msg("Bet amount is too low")]
    BetTooLow,
    #[msg("Bet amount is too high")]
    BetTooHigh,
    #[msg("Round is full")]
    RoundFull,
    #[msg("Not enough players to resolve round")]
    NotEnoughPlayers,
    #[msg("Pool is empty")]
    EmptyPool,
    #[msg("Unauthorized operation")]
    Unauthorized,
    #[msg("Invalid bet limits")]
    InvalidBetLimits,
    #[msg("Commission rate too high")]
    CommissionTooHigh,
}

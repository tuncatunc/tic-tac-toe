use anchor_lang::prelude::*;
use num_derive::*;
use num_traits::*;

declare_id!("RLjm256P3u3pnS8JNzd1Emp77eBctBbYw2MF9Aohzis");

#[program]
pub mod tic_tac_toe {
    use super::*;

    pub fn setup_game(ctx: Context<SetupGame>, player_two: Pubkey) -> Result<()> {
        let game = &mut ctx.accounts.game;
        game.players = [ctx.accounts.player_one.key(), player_two];
        game.turn = 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct SetupGame<'info> {
    #[account(init, payer = player_one)]
    pub game: Account<'info, Game>,
    #[account(mut)]
    pub player_one: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
#[derive(Default)]
pub struct Game {
    players: [Pubkey; 2],
    turn: u8,
    board: [[Option<Sign>; 3]; 3],
    state: GameState
}

#[derive(AnchorSerialize, AnchorDeserialize, Clone, PartialEq, Eq)]
pub enum GameState {
    Active,
    Tie,
    Won {winner: Pubkey}
}

impl Default for GameState {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(
    AnchorSerialize, 
    AnchorDeserialize, 
    FromPrimitive,
    ToPrimitive,
    Copy,
    Clone, 
    PartialEq, 
    Eq)]
pub enum Sign {
    X,
    O
}
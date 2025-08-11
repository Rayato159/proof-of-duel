use anchor_lang::prelude::*;

pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("GsetEEa4YtiaFcQP4NnqM2vBtJrtbFjKBgfdszMK8ePC");

#[program]
pub mod proof_of_duel_program {
    use super::*;

    pub fn initialize_player(ctx: Context<InitializePlayer>) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.initialize();
        Ok(())
    }

    pub fn win_increment(ctx: Context<WinIncrement>) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.win_increment();
        Ok(())
    }

    pub fn loss_increment(ctx: Context<LossIncrement>) -> Result<()> {
        let player = &mut ctx.accounts.player;
        player.loss_increment();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

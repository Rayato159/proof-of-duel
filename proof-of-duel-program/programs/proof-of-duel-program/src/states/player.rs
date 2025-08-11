use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Player {
    pub win: u64,
    pub loss: u64,
}

impl Player {
    pub fn initialize(&mut self) {
        self.win = 0;
        self.loss = 0;
    }

    pub fn win_increment(&mut self) {
        self.win += 1;
    }

    pub fn loss_increment(&mut self) {
        self.loss += 1;
    }
}

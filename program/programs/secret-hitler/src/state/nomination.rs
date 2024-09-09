use anchor_lang::prelude::*;

#[account]
pub struct Nomination {
    pub voters_index: Vec<u8>,
    pub nominee: Pubkey,
    pub nein: u8,
    pub ja: u8,
    pub bump: u8,
}

impl Space for Nomination {
    const INIT_SPACE: usize = 8 + 32 + (4 + 10) + (1 * 3);
}

impl Nomination {
    pub fn init(&mut self, nominee: Pubkey, president_index: u8, bump: u8) {
        self.voters_index = vec![president_index];
        self.nominee = nominee;
        self.nein = 0;
        self.ja = 1; // it is assumed the president votes ja by nominating
        self.bump = bump;
    }
}

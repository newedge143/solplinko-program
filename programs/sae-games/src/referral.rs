use super::*;

#[account]
#[derive(Default)]
pub struct Referral {
  pub bump: u8,
  pub authority: Pubkey,
  pub owner: Pubkey,
  pub verified: bool,
  pub percentage: u8,
}

impl Referral {
  pub const LEN: usize = 8  // DISCRIMINATOR
    + 1                     // BUMP
    + 32                    // AUTHORITY PUBKEY
    + 32                    // OWNER PUBKEY
    + 1                     // VERIFIED
    + 1;                    // PERCENTAGE
}

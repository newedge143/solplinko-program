use super::*;

#[account]
#[derive(Default)]
pub struct Dao {
  pub bump: u8,
  pub authority: Pubkey,
  pub owner: Pubkey,
  pub timestamp: i64,
  pub verified: bool,
}

impl Dao {
  pub const LEN: usize = 8  // DISCRIMINATOR
    + 1                     // BUMP
    + 32                    // AUTHORITY PUBKEY
    + 32                    // OWNER PUBKEY
    + 8                     // TIMESTAMP
    + 1;                    // VERIFIED
}

#[derive(Accounts)]
#[instruction(
  bump: u8,
  referral_bump: u8,
)]
pub struct CreateDao<'info> {
  #[account(
    init,
    seeds = [
      b"dao".as_ref(),
      owner.key().as_ref(),
      authority.key().as_ref(),
    ],
    payer = owner,
    space = Dao::LEN,
    bump,
  )]
  pub dao: Account<'info, Dao>,

  #[account(
    init,
    seeds = [
      b"referral".as_ref(),
      dao.key().as_ref(),
      authority.key().as_ref(),
    ],
    payer = owner,
    space = Referral::LEN,
    bump,
  )]
  pub referral: Account<'info, Referral>,

  #[account(mut)]
  pub owner: Signer<'info>,

  /// CHECK: authority account
  pub authority: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  _referral_bump: u8,
)]
pub struct DaoVerification<'info> {
  #[account(
    mut,
    seeds = [
      b"dao".as_ref(),
      owner.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = dao.bump == _bump,
    constraint = dao.owner == owner.key(),
    constraint = dao.authority == authority.key(),
  )]
  pub dao: Account<'info, Dao>,

  #[account(
    mut,
    seeds = [
      b"referral".as_ref(),
      dao.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _referral_bump,
    constraint = referral.bump == _referral_bump,
    constraint = referral.owner == dao.key(),
    constraint = referral.authority == authority.key(),
  )]
  pub referral: Account<'info, Referral>,

  /// CHECK: owner account
  pub owner: AccountInfo<'info>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  _referral_bump: u8,
)]
pub struct ClaimDaoReferral<'info> {
  #[account(
    mut,
    seeds = [
      b"referral".as_ref(),
      dao.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _referral_bump,
    constraint = referral.bump == _referral_bump,
    constraint = referral.owner == dao.key(),
    constraint = referral.authority == authority.key(),
  )]
  pub referral: Account<'info, Referral>,

  #[account(
    seeds = [
      b"dao".as_ref(),
      owner.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = dao.bump == _bump,
    constraint = dao.owner == owner.key(),
    constraint = dao.authority == authority.key(),
  )]
  pub dao: Account<'info, Dao>,

  #[account(mut)]
  pub owner: Signer<'info>,

  /// CHECK: authority account
  pub authority: AccountInfo<'info>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  _referral_bump: u8,
)]
pub struct ClaimDaoTokenReferral<'info> {
  #[account(
    init_if_needed,
    payer = owner,
    associated_token::mint = mint,
    associated_token::authority = referral,
  )]
  pub referral_ata: Box<Account<'info, TokenAccount>>,

  #[account(
    seeds = [
      b"referral".as_ref(),
      dao.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _referral_bump,
    constraint = referral.bump == _referral_bump,
    constraint = referral.owner == dao.key(),
    constraint = referral.authority == authority.key(),
  )]
  pub referral: Box<Account<'info, Referral>>,

  #[account(
    seeds = [
      b"dao".as_ref(),
      owner.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = dao.bump == _bump,
    constraint = dao.owner == owner.key(),
    constraint = dao.authority == authority.key(),
  )]
  pub dao: Box<Account<'info, Dao>>,

  #[account(
    init_if_needed,
    payer = owner,
    associated_token::mint = mint,
    associated_token::authority = owner,
  )]
  pub owner_ata: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub owner: Signer<'info>,

  /// CHECK: authority account
  pub authority: AccountInfo<'info>,

  pub mint: Account<'info, Mint>,

  pub rent: Sysvar<'info, Rent>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  pub token_program: Program<'info, Token>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  _referral_bump: u8,
  percentage: u8,
)]
pub struct ChangePercentageOfDaoReferral<'info> {
  #[account(
    mut,
    seeds = [
      b"referral".as_ref(),
      dao.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _referral_bump,
    constraint = referral.bump == _referral_bump,
    constraint = referral.owner == dao.key(),
    constraint = referral.authority == authority.key(),
  )]
  pub referral: Account<'info, Referral>,

  #[account(
    seeds = [
      b"dao".as_ref(),
      owner.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = dao.bump == _bump,
    constraint = dao.owner == owner.key(),
    constraint = dao.authority == authority.key(),
  )]
  pub dao: Account<'info, Dao>,

  /// CHECK: owner account
  pub owner: AccountInfo<'info>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub system_program: Program<'info, System>,
}

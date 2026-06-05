use super::*;

#[account]
#[derive(Default)]
pub struct Fee {
  pub bump: u8,
  pub authority: Pubkey,
  pub volume: u64,
  pub referral_volume: u64,
  pub referral_today_volume: u64,
}

impl Fee {
  pub const LEN: usize = 8  // DISCRIMINATOR
    + 1                     // BUMP
    + 32                    // AUTHORITY PUBKEY
    + 8                     // VOLUME
    + 8                     // REFERRAL VOLUME
    + 8;                    // REFERRAL TODAY VOLUME
}

#[derive(Accounts)]
#[instruction(
  bump: u8,
  volume: u64,
)]
pub struct InitFee<'info> {
  #[account(
    init,
    seeds = [
      b"fee".as_ref(),
      authority.key().as_ref(),
    ],
    payer = authority,
    space = Fee::LEN,
    bump,
  )]
  pub fee: Account<'info, Fee>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  bump: u8,
  volume: u64,
)]
pub struct InitTokenFee<'info> {
  #[account(
    init,
    payer = authority,
    associated_token::mint = mint,
    associated_token::authority = fee,
  )]
  pub fee_ata: Account<'info, TokenAccount>,

  #[account(
    init,
    seeds = [
      b"fee".as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
    ],
    payer = authority,
    space = Fee::LEN,
    bump,
  )]
  pub fee: Account<'info, Fee>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub mint: Account<'info, Mint>,

  pub rent: Sysvar<'info, Rent>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  pub token_program: Program<'info, Token>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  amount: u64,
)]
pub struct DepositFee<'info> {
  #[account(
    mut,
    seeds = [
      b"fee".as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = fee.bump == _bump,
    constraint = fee.authority == authority.key(),
  )]
  pub fee: Account<'info, Fee>,

  #[account(mut)]
  pub source: Signer<'info>,

  /// CHECK: authority account
  pub authority: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  amount: u64,
)]
pub struct DepositTokenFee<'info> {
  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = fee,
  )]
  pub fee_ata: Account<'info, TokenAccount>,

  #[account(
    seeds = [
      b"fee".as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = fee.bump == _bump,
    constraint = fee.authority == authority.key(),
  )]
  pub fee: Account<'info, Fee>,

  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = source,
  )]
  pub source_ata: Account<'info, TokenAccount>,

  #[account(mut)]
  pub source: Signer<'info>,

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
  amount: u64,
)]
pub struct WithdrawFee<'info> {
  #[account(
    mut,
    seeds = [
      b"fee".as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = fee.bump == _bump,
    constraint = fee.authority == authority.key(),
  )]
  pub fee: Account<'info, Fee>,

  /// CHECK: destination account
  #[account(mut)]
  pub destination: AccountInfo<'info>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  amount: u64,
)]
pub struct WithdrawTokenFee<'info> {
  #[account(
    mut,
    associated_token::mint = mint,
    associated_token::authority = fee,
  )]
  pub fee_ata: Account<'info, TokenAccount>,

  #[account(
    seeds = [
      b"fee".as_ref(),
      mint.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = fee.bump == _bump,
    constraint = fee.authority == authority.key(),
  )]
  pub fee: Account<'info, Fee>,

  #[account(
    init_if_needed,
    payer = authority,
    associated_token::mint = mint,
    associated_token::authority = destination,
  )]
  pub destination_ata: Account<'info, TokenAccount>,

  /// CHECK: destination account
  pub destination: AccountInfo<'info>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub mint: Account<'info, Mint>,

  pub rent: Sysvar<'info, Rent>,

  pub associated_token_program: Program<'info, AssociatedToken>,

  pub token_program: Program<'info, Token>,

  pub system_program: Program<'info, System>,
}

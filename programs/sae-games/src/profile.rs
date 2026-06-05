use super::*;

#[account]
#[derive(Default)]
pub struct Profile {
  pub bump: u8,
  pub authority: Pubkey,
  pub player: Pubkey,
  pub timestamp: i64,
  pub next_game_id: u64,
  pub dao: Option<Pubkey>,
  pub invitation: Option<Pubkey>,
}

impl Profile {
  pub const LEN: usize = 8  // DISCRIMINATOR
    + 1                     // BUMP
    + 32                    // AUTHORITY PUBKEY
    + 32                    // PLAYER PUBKEY
    + 8                     // TIMESTAMP
    + 8                     // NEXT GAME ID
    + 1 + 32                // DAO PUBKEY
    + 1 + 32;               // INVITATION PUBKEY;
}

#[derive(Accounts)]
#[instruction(
  bump: u8,
  referral_bump: u8,
)]
pub struct Register<'info> {
  #[account(
    init,
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    payer = player,
    space = Profile::LEN,
    bump,
  )]
  pub profile: Account<'info, Profile>,

  #[account(
    init,
    seeds = [
      b"referral".as_ref(),
      profile.key().as_ref(),
      authority.key().as_ref(),
    ],
    payer = player,
    space = Referral::LEN,
    bump,
  )]
  pub referral: Account<'info, Referral>,

  #[account(mut)]
  pub player: Signer<'info>,

  /// CHECK: authority account
  pub authority: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  bump: u8,
  referral_bump: u8,
  _invitation_bump: u8,
)]
pub struct RegisterWithReferral<'info> {
  #[account(
    init,
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    payer = player,
    space = Profile::LEN,
    bump,
  )]
  pub profile: Account<'info, Profile>,

  #[account(
    init,
    seeds = [
      b"referral".as_ref(),
      profile.key().as_ref(),
      authority.key().as_ref(),
    ],
    payer = player,
    space = Referral::LEN,
    bump,
  )]
  pub referral: Account<'info, Referral>,

  #[account(
    seeds = [
      b"referral".as_ref(),
      invite_owner.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _invitation_bump,
    constraint = invitation.bump == _invitation_bump,
    constraint = invitation.owner == invite_owner.key(),
    constraint = invitation.authority == authority.key(),
  )]
  pub invitation: Account<'info, Referral>,

  #[account(mut)]
  pub player: Signer<'info>,

  /// CHECK: invite_owner account
  pub invite_owner: AccountInfo<'info>,

  /// CHECK: authority account
  pub authority: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  _dao_bump: u8,
)]
pub struct SetProfileDao<'info> {
  #[account(
    mut,
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = profile.bump == _bump,
    constraint = profile.player == player.key(),
    constraint = profile.authority == authority.key(),
  )]
  pub profile: Account<'info, Profile>,

  #[account(
    mut,
    seeds = [
      b"dao".as_ref(),
      dao_owner.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _dao_bump,
    constraint = dao.bump == _dao_bump,
    constraint = dao.owner == dao_owner.key(),
    constraint = dao.authority == authority.key(),
  )]
  pub dao: Account<'info, Dao>,

  #[account(mut)]
  pub player: Signer<'info>,

  /// CHECK: dao_owner account
  pub dao_owner: AccountInfo<'info>,

  /// CHECK: authority account
  pub authority: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
)]
pub struct RemoveProfileDao<'info> {
  #[account(
    mut,
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = profile.bump == _bump,
    constraint = profile.player == player.key(),
    constraint = profile.authority == authority.key(),
  )]
  pub profile: Account<'info, Profile>,

  #[account(mut)]
  pub player: Signer<'info>,

  /// CHECK: authority account
  pub authority: AccountInfo<'info>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  _referral_bump: u8,
)]
pub struct ClaimProfileReferral<'info> {
  #[account(
    mut,
    seeds = [
      b"referral".as_ref(),
      profile.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _referral_bump,
    constraint = referral.bump == _referral_bump,
    constraint = referral.owner == profile.key(),
    constraint = referral.authority == authority.key(),
  )]
  pub referral: Account<'info, Referral>,

  #[account(
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = profile.bump == _bump,
    constraint = profile.player == player.key(),
    constraint = profile.authority == authority.key(),
  )]
  pub profile: Account<'info, Profile>,

  #[account(mut)]
  pub player: Signer<'info>,

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
pub struct ClaimProfileTokenReferral<'info> {
  #[account(
    init_if_needed,
    payer = player,
    associated_token::mint = mint,
    associated_token::authority = referral,
  )]
  pub referral_ata: Box<Account<'info, TokenAccount>>,

  #[account(
    seeds = [
      b"referral".as_ref(),
      profile.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _referral_bump,
    constraint = referral.bump == _referral_bump,
    constraint = referral.owner == profile.key(),
    constraint = referral.authority == authority.key(),
  )]
  pub referral: Box<Account<'info, Referral>>,

  #[account(
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = profile.bump == _bump,
    constraint = profile.player == player.key(),
    constraint = profile.authority == authority.key(),
  )]
  pub profile: Box<Account<'info, Profile>>,

  #[account(
    init_if_needed,
    payer = player,
    associated_token::mint = mint,
    associated_token::authority = player,
  )]
  pub player_ata: Box<Account<'info, TokenAccount>>,

  #[account(mut)]
  pub player: Signer<'info>,

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
)]
pub struct UnverifyProfileReferral<'info> {
  #[account(
    mut,
    seeds = [
      b"referral".as_ref(),
      profile.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _referral_bump,
    constraint = referral.bump == _referral_bump,
    constraint = referral.owner == profile.key(),
    constraint = referral.authority == authority.key(),
  )]
  pub referral: Account<'info, Referral>,

  #[account(
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = profile.bump == _bump,
    constraint = profile.player == player.key(),
    constraint = profile.authority == authority.key(),
  )]
  pub profile: Account<'info, Profile>,

  /// CHECK: player account
  pub player: AccountInfo<'info>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(
  _bump: u8,
  _referral_bump: u8,
  percentage: u8,
)]
pub struct ChangePercentageOfProfileReferral<'info> {
  #[account(
    mut,
    seeds = [
      b"referral".as_ref(),
      profile.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _referral_bump,
    constraint = referral.bump == _referral_bump,
    constraint = referral.owner == profile.key(),
    constraint = referral.authority == authority.key(),
  )]
  pub referral: Account<'info, Referral>,

  #[account(
    seeds = [
      b"profile".as_ref(),
      player.key().as_ref(),
      authority.key().as_ref(),
    ],
    bump = _bump,
    constraint = profile.bump == _bump,
    constraint = profile.player == player.key(),
    constraint = profile.authority == authority.key(),
  )]
  pub profile: Account<'info, Profile>,

  /// CHECK: player account
  pub player: AccountInfo<'info>,

  #[account(mut)]
  pub authority: Signer<'info>,

  pub rent: Sysvar<'info, Rent>,

  pub system_program: Program<'info, System>,
}

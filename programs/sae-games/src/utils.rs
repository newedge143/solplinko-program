use super::*;

pub fn transfer_invoke<'info>(
  from_account: &AccountInfo<'info>,
  to_account: &AccountInfo<'info>,
  amount: u64,
) -> Result<()> {
  let transfer_ix = invoke(
    &system_instruction::transfer(
      &from_account.to_account_info().key,
      &to_account.to_account_info().key,
      amount,
    ),
    &[
      from_account.to_account_info(),
      to_account.to_account_info(),
    ],
  );

  if transfer_ix.is_err() {
    return transfer_ix.map_err(Into::into);
  }

  Ok(())
}

pub fn transfer_invoke_signed(
  from_account: &mut AccountInfo,
  to_account: &mut AccountInfo,
  amount: u64,
) -> ProgramResult {
  **from_account.try_borrow_mut_lamports()? = from_account
    .lamports()
    .checked_sub(amount)
    .ok_or(ProgramError::InvalidArgument)?;

  **to_account.try_borrow_mut_lamports()? = to_account
    .lamports()
    .checked_add(amount)
    .ok_or(ProgramError::InvalidArgument)?;

  Ok(())
}

pub fn transfer_spl_invoke<'info>(
  from_account: &AccountInfo<'info>,
  to_account: &AccountInfo<'info>,
  amount: u64,
  program: &AccountInfo<'info>,
  authority: &AccountInfo<'info>,
) -> Result<()> {
  return transfer(
    CpiContext::new(
      program.to_account_info(),
      Transfer {
        from: from_account.to_account_info(),
        to: to_account.to_account_info(),
        authority: authority.to_account_info(),
      },
    ),
    amount,
  );
}

pub fn transfer_spl_invoke_signed<'info>(
  from_account: &AccountInfo<'info>,
  to_account: &AccountInfo<'info>,
  amount: u64,
  program: &AccountInfo<'info>,
  authority: &AccountInfo<'info>,
  seeds: &[&[&[u8]]],
) -> Result<()> {
  return transfer(
    CpiContext::new_with_signer(
      program.to_account_info(),
      Transfer {
        from: from_account.to_account_info(),
        to: to_account.to_account_info(),
        authority: authority.to_account_info(),
      },
      seeds,
    ),
    amount,
  );
}

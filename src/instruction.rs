/// instruction.rs -> program API, (de)serializing instruction data

use std::convert::TryInto;
use solana_program::program_error::ProgramError;

use crate::error::EscrowError::InvalidInstruction;


pub enum EscrowInstruction {

	/// Starts the trade by creating and populating an escrow account and transferring ownership of the given temp token account to the PDA
	///
	///
	/// Accounts expected:
	///
	/// 0. `[signer]` The account of the person initializing the escrow
	/// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
	/// 2. `[]` The initializer's token account for the token they will receive should the trade go through
	/// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
	/// 4. `[]` The rent sysvar (deprecated as of solana-program v1.6.5 as sysvars can be accessed without passing into entrypoint as an account)
	/// 5. `[]` The token program
	InitEscrow {
		/// The amount party A expects to receive of token Y
		/// This amount is not provided through an account but through the `instruction_data`
		amount: u64
	}
}

impl EscrowInstruction {

	/// Unpacks a byte buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
	pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
		let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

		Ok(match tag {
			0 => Self::InitEscrow {
				amount: Self::unpack_amount(rest)?,
			},
			_ => return Err(InvalidInstruction.into()),
		})
	}

	fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
		let amount = input
			.get(..8)
			.and_then(|slice| slice.try_into().ok())
			.map(u64::from_le_bytes)
			.ok_or(InvalidInstruction)?;
		Ok(amount)
	}
}

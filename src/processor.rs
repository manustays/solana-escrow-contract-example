/// processor.rs -> program logic

use solana_program::{
	account_info::{next_account_info, AccountInfo},
	entrypoint::ProgramResult,
    program_error::ProgramError,
	msg,
	pubkey::Pubkey,
	program_pack::{Pack, IsInitialized},
	sysvar::{rent::Rent, Sysvar},
	program::invoke,
};

use crate::{instruction::EscrowInstruction, error::EscrowError, state::Escrow};

pub struct Processor;
impl Processor {
	pub fn process(program_id: &Pubkey, accounts: &[AccountInfo], instruction_data: &[u8]) -> ProgramResult {
		let instruction = EscrowInstruction::unpack(instruction_data)?;

		match instruction {
			EscrowInstruction::InitEscrow { amount } => {
				msg!("Instruction: InitEscrow");
				Self::process_init_escrow(accounts, amount, program_id)
			}
		}
	}

	fn process_init_escrow(
        accounts: &[AccountInfo],
        amount: u64,
        program_id: &Pubkey,
    ) -> ProgramResult {
        let account_info_iter = &mut accounts.iter();
        let initializer = next_account_info(account_info_iter)?;

        if !initializer.is_signer {
            return Err(ProgramError::MissingRequiredSignature);
        }

		let temp_token_account = next_account_info(account_info_iter)?;

		let token_to_receive_account = next_account_info(account_info_iter)?;
		if *token_to_receive_account.owner != spl_token::id() {
			return Err(ProgramError::IncorrectProgramId);
		}

		let escrow_account = next_account_info(account_info_iter)?;
		let rent = &Rent::from_account_info(next_account_info(account_info_iter)?)?;

		if !rent.is_exempt(escrow_account.lamports(), escrow_account.data_len()) {
			return Err(EscrowError::NotRentExempt.into());
		}

		// Create the escrow struct instance and check that it is indeed uninitialized
		let mut escrow_info = Escrow::unpack_unchecked(&escrow_account.data.borrow())?;
		if escrow_info.is_initialized() {
			return Err(ProgramError::AccountAlreadyInitialized);
		}

		// Now, populate the Escrow struct's fields
		escrow_info.is_initialized = true;
		escrow_info.initializer_pubkey = *initializer.key;
		escrow_info.temp_token_account_pubkey = *temp_token_account.key;
		escrow_info.initializer_token_to_receive_account_pubkey = *token_to_receive_account.key;
		escrow_info.expected_amount = amount;

		Escrow::pack(escrow_info, &mut escrow_account.data.borrow_mut())?;		// `pack` is another default function which internally calls our pack_into_slice function.


		// Now, we need to transfer (user space) ownership of the temporary token account to the PDA...

		// What is a PDA (Program derived address)?
		//		0. https://docs.solana.com/developing/programming-model/calling-between-programs#program-derived-addresses
		//		1. Allows programmaticly generated signature to be used when calling between programs.
		//		2. To give a program the authority over an account and later transfer that authority to another.
		//		3. Allow programs to control specific addresses, called program addresses, in such a way that no external user can generate valid transactions with signatures for those addresses.
		//		4. Allow programs to programmatically sign for program addresses that are present in instructions invoked via Cross-Program Invocations.
		//		5. Given the previous two conditions, users can securely transfer or assign the authority of on-chain assets to program addresses and the program can then assign that authority elsewhere at its discretion.
		//		6. A Program address does not lie on the ed25519 curve and therefore has no valid private key associated with it, and thus generating a signature for it is impossible.
		//		7. While it has no private key of its own, it can be used by a program to issue an instruction that includes the Program address as a signer.

		// Create a PDA by passing in an array of seeds and the program_id to `find_program_address`.
		// Passing a static seed: "escrow".
		// We need 1 PDA that can own N temporary token accounts for different escrows occuring at any and possibly the same point in time.
		// We won't need the bump seed in Alice's tx.
		let (pda, _bump_seed) = Pubkey::find_program_address(&[b"escrow"], program_id);

		// To transfer the (user space) ownership of the temporary token account to the PDA,
		//		we will call the token program from our escrow program.
		//		This is called a Cross-Program Invocation (opens new window)
		//			and executed using either the invoke or the invoke_signed function.

		// Get the token_program account.
		// The program being called through a CPI (Cross-Program Invocation) must be included as an account in the 2nd argument of invoke
		let token_program = next_account_info(account_info_iter)?;

		// Now we create the instruction that the token program would expect were we executing a normal call.
		// `set_authority` is a builder helper function (in instruction.rs) to create such an instruction
		// Using [Signature Extension concept](https://docs.solana.com/developing/programming-model/calling-between-programs#instructions-that-require-privileges)
		//		because Alice signed the InitEscrow transaction, the program can make the token program set_authority CPI and include her pubkey as a signer pubkey.
		//		This is necessary because changing a token account's owner should of course require the approval of the current owner.
		let owner_change_ix = spl_token::instruction::set_authority(
			token_program.key,				// token program id
			temp_token_account.key,			// account whose authority we'd like to change
			Some(&pda),						// account that's the new authority (in this case the PDA)
			spl_token::instruction::AuthorityType::AccountOwner,	// the type of authority change (change the owner)
			initializer.key,				// the current account owner (Alice -> initializer.key)
			&[&initializer.key],			// the public keys signing the CPI
		)?;

		msg!("Calling the token program to transfer token account ownership...");
		invoke(
			&owner_change_ix,					// The instruction
			&[									// The accounts required by the instruction
				temp_token_account.clone(),
				initializer.clone(),
				token_program.clone(),
			],
		)?;

        Ok(())
    }
}

/**
 * entrypoint.rs -> entrypoint to the program
	*
	* Program Flow:
	* 		1. entrypoint is called
	* 		2. entrypoint forwards args to processor
	* 		3. processor asks `instruction.rs` to decode the `instruction_data` argument from the entrypoint function
	* 		4. Using the decoded data, the processor will now decide which processing function to use to process the request
	* 		5. The processor may use state.rs to encode state into or decode the state of an account which has been passed into the entrypoint
	*
	* While there is only one entrypoint, program execution can follow different paths depending on the given instruction data that is decoded inside `instruction.rs`
	*/

use solana_program::{
	account_info::AccountInfo, entrypoint, entrypoint::ProgramResult, msg, pubkey::Pubkey,
};

// Using the entrypoint! macro to declare the process_instruction function the entrypoint to the program.
//     Entrypoints are the only way to call a program;
//     all calls go through the function declared as the entrypoint.
// When called, a program is passed to its BPF Loader which processes the call.
//     Different BPF loaders may require different entrypoints.
entrypoint!(process_instruction);


/**
 * Accounts:
	* 		Accounts are used to store state (as Solana programs are stateless by default).
	* 		Each account can hold data & SOL.
	* 		Each account also has an owner and only the owner may debit the account and adjust its data.
	* 		Accounts can only be owned by programs. Eg: the `system_program`.
	* 		All accounts to be read or written to must be passed into the entrypoint function.
	* 			This allows the runtime to parallelise transactions.
	* 			Transactions can run in parallel that do not touch the same accounts
	* 				or touch the same accounts but only read and don't write.
	*/

fn process_instruction(
	program_id: &Pubkey,			//  id of the currently executing program
	accounts: &[AccountInfo],		// Accounts are used to store state
	instruction_data: &[u8],		// data passed to the program by the caller
) -> ProgramResult {
	msg!(
		"process_instruction: {}: {} accounts, data={:?}",
		program_id,
		accounts.len(),
		instruction_data
	);
	Ok(())
}

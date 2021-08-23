/// state.rs -> program objects, (de)serializing state

use solana_program::{
	program_pack::{IsInitialized, Pack, Sealed},
	program_error::ProgramError,
	pubkey::Pubkey,
};

// arrayref library is used to get references to sections of a slice
// https://docs.rs/arrayref/latest/arrayref/
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};

pub struct Escrow {
	pub is_initialized: bool,			// To Check if the given escrow account is already in use
	pub initializer_pubkey: Pubkey,
	pub temp_token_account_pubkey: Pubkey,
	pub initializer_token_to_receive_account_pubkey: Pubkey,
	pub expected_amount: u64,
}

impl Sealed for Escrow {}

impl IsInitialized for Escrow {
	fn is_initialized(&self) -> bool {
		self.is_initialized
	}
}

impl Pack for Escrow {

	// Calculating the length of the Escrow struct by adding the sizes of the individual data types:
	// 		1 (bool) + 3 * 32 (3 Pubkeys) + 8 (u64) = 105.
	//			Using entire u8 for the bool to make coding easier.
	//			The cost of those extra wasted bits is infinitesimal.
	const LEN: usize = 105;

	///  Turns an array of u8 into an instance of the Escrow struct.
	///  Not passing &self as there is no self yet.
	fn unpack_from_slice(src: &[u8]) -> Result<Self, ProgramError> {
		let src = array_ref![src, 0, Escrow::LEN];	// use array_ref to generate an array reference to a subset of a sliceable bit of data

		let (
			is_initialized,
			initializer_pubkey,
			temp_token_account_pubkey,
			initializer_token_to_receive_account_pubkey,
			expected_amount,
		) = array_refs![src, 1, 32, 32, 32, 8];		// use array_refs to generate a series of array references to an input array reference. The idea is if you want to break an array into a series of contiguous and non-overlapping arrays. array_refs is a bit funny in that it insists on slicing up the entire array. This is intentional, as I find it handy to make me ensure that my sub-arrays add up to the entire array. This macro will never panic, since the sizes are all checked at compile time.

		let is_initialized = match is_initialized {
			[0] => false,
			[1] => true,
			_ => return Err(ProgramError::InvalidAccountData),
		};

		Ok(Escrow {
			is_initialized,
			initializer_pubkey: Pubkey::new_from_array(*initializer_pubkey),
			temp_token_account_pubkey: Pubkey::new_from_array(*temp_token_account_pubkey),
			initializer_token_to_receive_account_pubkey: Pubkey::new_from_array(*initializer_token_to_receive_account_pubkey),
			expected_amount: u64::from_le_bytes(*expected_amount),
		})
	}


	/// Turns an instance of the Escrow struct into an array of u8.
	/// I.e, serialize it into the given dst slice
	fn pack_into_slice(&self, dst: &mut [u8]) {
		let dst = array_mut_ref![dst, 0, Escrow::LEN];	// use array_mut_ref to generate a mutable array reference to a subset of a sliceable bit of data

		let (
			is_initialized_dst,
			initializer_pubkey_dst,
			temp_token_account_pubkey_dst,
			initializer_token_to_receive_account_pubkey_dst,
			expected_amount_dst,
		) = mut_array_refs![dst, 1, 32, 32, 32, 8];		// use mut_array_refs to generate a series of mutable array references to an input mutable array reference. The idea is if you want to break an array into a series of contiguous and non-overlapping mutable array references. Like array_refs!, mut_array_refs! is a bit funny in that it insists on slicing up the entire array. This is intentional, as I find it handy to make me ensure that my sub-arrays add up to the entire array. This macro will never panic, since the sizes are all checked at compile time.

		let Escrow {
			is_initialized,
			initializer_pubkey,
			temp_token_account_pubkey,
			initializer_token_to_receive_account_pubkey,
			expected_amount,
		} = self;

		is_initialized_dst[0] = *is_initialized as u8;
		initializer_pubkey_dst.copy_from_slice(initializer_pubkey.as_ref());
		temp_token_account_pubkey_dst.copy_from_slice(temp_token_account_pubkey.as_ref());
		initializer_token_to_receive_account_pubkey_dst.copy_from_slice(initializer_token_to_receive_account_pubkey.as_ref());
		*expected_amount_dst = expected_amount.to_le_bytes();
	}
}

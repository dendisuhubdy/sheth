use crate::accounts::{random_accounts, AddressedAccount};
use crate::proof::offsets::calculate as calculate_offsets;
use crate::proof::uncompressed::generate as generate_uncompressed_proof;
use crate::transactions;
use imp::Imp;
use sheth::process::process_transactions;
use sheth::transaction::Transaction;
use sheth::u264::U264;

/// A `Blob` includes all the neccessary data to construct the input data blob to `sheth`.
#[derive(Clone)]
pub struct Blob {
    pub proof: Vec<u8>,
    pub transactions: Vec<Transaction>,
    pub accounts: Vec<AddressedAccount>,
}

impl Blob {
    /// Returns a serialized blob that can be used as input to `sheth`.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut ret = transactions::serialize(&self.transactions);
        ret.extend(&self.proof);
        ret
    }
}

/// Build a blob with specified tree height, accounts, and transactions.
pub fn generate(accounts: usize, transactions: usize, tree_height: usize) -> Blob {
    let accounts = random_accounts(accounts, tree_height);
    let proof = generate_uncompressed_proof(accounts.clone(), tree_height);
    let offsets = calculate_offsets(proof.indexes);
    let transactions = transactions::generate(transactions, accounts.clone());

    let mut compressed_proof = offsets.iter().fold(vec![], |mut acc, x| {
        acc.extend(&x.to_le_bytes());
        acc
    });

    compressed_proof = proof.values.iter().fold(compressed_proof, |mut acc, x| {
        acc.extend(x.as_bytes());
        acc
    });

    Blob {
        proof: compressed_proof,
        transactions,
        accounts,
    }
}

/// Returns a `Blob` and the pre-state root + post-state root for the blob.
pub fn generate_with_roots(
    accounts: usize,
    transactions: usize,
    tree_height: usize,
) -> (Blob, [u8; 32], [u8; 32]) {
    let mut blob = generate(accounts, transactions, tree_height);
    let ret_blob = blob.clone();

    let mut mem = Imp::<U264>::new(&mut blob.proof, tree_height);

    let pre_state = mem.root();
    assert_eq!(process_transactions(&mut mem, &blob.transactions), Ok(()));
    let post_state = mem.root();

    (ret_blob, pre_state, post_state)
}

#[cfg(test)]
mod test {
    use super::*;
    use arrayref::array_ref;

    #[test]
    fn generate_small_tree() {
        // NOTE: This has not been hand checked. It may be incorrect.
        // Indexes = [16, 17, 9, 40, 41, 42, 43, 11, 3]
        let mut proof = vec![
            0, 0, 0, 0, 9, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 3, 0, 0, 0, 0, 0, 0, 0, 2,
            0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 2, 0, 0, 0, 0, 0,
            0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 218, 109, 128, 123, 247, 149, 16,
            97, 70, 229, 130, 39, 117, 217, 20, 176, 39, 122, 101, 36, 15, 101, 14, 212, 200, 167,
            202, 119, 130, 78, 90, 223, 120, 72, 181, 215, 17, 188, 152, 131, 153, 99, 23, 163,
            249, 201, 2, 105, 213, 103, 113, 0, 93, 84, 10, 25, 24, 73, 57, 201, 232, 208, 219, 42,
            85, 242, 146, 169, 167, 93, 196, 41, 170, 134, 245, 251, 132, 117, 101, 88, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 197, 33, 10, 45, 228, 168, 212, 211, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 192, 1, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 63, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 118, 1,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
            0, 0, 0, 0, 0, 0, 0,
        ];

        let root = vec![
            217, 115, 1, 53, 85, 249, 225, 213, 207, 56, 156, 226, 44, 251, 121, 36, 215, 189, 200,
            251, 244, 167, 142, 171, 19, 81, 176, 186, 168, 31, 240, 78,
        ];

        assert_eq!(generate(1, 0, 1).to_bytes(), proof);
        let mut mem = Imp::<U264>::new(&mut proof[4..], 1);
        assert_eq!(mem.root(), *array_ref![root, 0, 32]);
    }
}

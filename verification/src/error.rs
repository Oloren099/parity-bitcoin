use primitives::hash::H256;

#[derive(Debug, PartialEq)]
/// All possible verification errors
pub enum Error {
	/// has an equal duplicate in the chain
	Duplicate,
	/// Contains duplicated transactions
	DuplicatedTransactions,
	/// No transactions in block
	Empty,
	/// Invalid proof-of-work (Block hash does not satisfy nBits)
	Pow,
	/// Futuristic timestamp
	FuturisticTimestamp,
	/// Invalid timestamp
	Timestamp,
	/// First transaction is not a coinbase transaction
	Coinbase,
	/// One of the transactions is invalid (corresponding index and specific transaction error)
	Transaction(usize, TransactionError),
	/// nBits do not match difficulty rules
	Difficulty,
	/// Invalid merkle root
	MerkleRoot,
	/// Coinbase spends too much
	CoinbaseOverspend { expected_max: u64, actual: u64 },
	/// Maximum sigops operations exceeded - will not provide how much it was in total
	/// since it stops counting once `MAX_BLOCK_SIGOPS` is reached
	MaximumSigops,
	/// Coinbase signature is not in the range 2-100
	CoinbaseSignatureLength(usize),
	/// Block size is invalid
	Size(usize),
	/// Block transactions are not final.
	NonFinalBlock,
}

#[derive(Debug, PartialEq)]
/// Possible transactions verification errors
pub enum TransactionError {
	/// Not found corresponding output for transaction input
	Input(usize),
	/// Referenced coinbase output for the transaction input is not mature enough
	Maturity,
	/// Signature invalid for given input
	Signature(usize),
	/// Unknown previous transaction referenced
	UnknownReference(H256),
	/// Spends more than claims
	Overspend,
	/// Signature script can't be properly parsed
	SignatureMallformed(String),
	/// Too many signature operations
	Sigops(usize),
	/// Too many signature operations once p2sh operations included
	SigopsP2SH(usize),
	/// Coinbase transaction is found at position that is not 0
	MisplacedCoinbase,
	/// Not fully spent transaction with the same hash already exists, bip30.
	UnspentTransactionWithTheSameHash,
	/// Using output that is surely spent
	UsingSpentOutput(H256, u32),
}


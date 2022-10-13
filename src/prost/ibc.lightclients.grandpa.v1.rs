#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commitment {
    /// block height
    ///
    /// mmr root
    #[prost(uint32, tag = "1")]
    pub block_number: u32,
    #[prost(bytes = "vec", tag = "2")]
    pub payload: ::prost::alloc::vec::Vec<u8>,
    ///validator_set_id
    #[prost(uint64, tag = "3")]
    pub validator_set_id: u64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedCommitment {
    #[prost(message, optional, tag = "1")]
    pub commitment: ::core::option::Option<Commitment>,
    #[prost(message, repeated, tag = "2")]
    pub signatures: ::prost::alloc::vec::Vec<InnerSignature>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InnerSignature {
    #[prost(message, optional, tag = "1")]
    pub inner_signature: ::core::option::Option<Signature>,
}

#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    #[prost(bytes = "vec", tag = "1")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorMerkleProof {
    //// Proof items (does not contain the leaf hash, nor the root obviously).
    ////
    //// This vec contains all inner node hashes necessary to reconstruct the root hash given the
    //// leaf hash.
    ///
    //// Number of leaves in the original tree.
    ////
    //// This is needed to detect a case where we have an odd number of leaves that "get promoted"
    //// to upper layers.
    //// pub number_of_leaves: usize,
    #[prost(bytes = "vec", repeated, tag = "1")]
    pub proof: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    #[prost(uint32, tag = "2")]
    pub number_of_leaves: u32,
    /// Index of the leaf the proof is for (0-based).
    /// pub leaf_index: usize,

    #[prost(uint32, tag = "3")]
    pub leaf_index: u32,
    //// Leaf content.
    ////pub leaf: Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub leaf: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MmrLeaf {
    //// Version of the leaf format.
    //// Can be used to enable future format migrations and compatibility.
    #[prost(uint32, tag = "1")]
    pub version: u32,
    //// Current block parent number and hash.
    #[prost(message, optional, tag = "2")]
    pub parent_number_and_hash: ::core::option::Option<ParentNumberAndHash>,
    //// A merkle root of the next BEEFY authority set.
    #[prost(message, optional, tag = "3")]
    pub beefy_next_authority_set: ::core::option::Option<ValidatorSet>,
    //// A merkle root of all registered parachain heads.
    #[prost(bytes = "vec", tag = "4")]
    pub leaf_extra: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParentNumberAndHash {
    #[prost(uint32, tag = "1")]
    pub block_number: u32,
    /// header hash
    #[prost(bytes = "vec", tag = "2")]
    pub mmr_root: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorSet {
    //// Id of the next set.
    ////
    //// Id is required to correlate BEEFY signed commitments with the validator set.
    //// Light Client can easily verify that the commitment witness it is getting is
    //// produced by the latest validator set.
    #[prost(uint64, tag = "1")]
    pub id: u64,
    //// Number of validators in the set.
    ////
    //// Some BEEFY Light Clients may use an interactive protocol to verify only subset
    //// of signatures. We put set length here, so that these clients can verify the minimal
    //// number of required signatures.
    #[prost(uint32, tag = "2")]
    pub len: u32,
    //// Merkle Root Hash build from BEEFY AuthorityIds.
    ////
    //// This is used by Light Clients to confirm that the commitments are signed by the correct
    //// validator set. Light Clients using interactive protocol, might verify only subset of
    //// signatures, hence don't require the full list here (will receive inclusion proofs).
    #[prost(bytes = "vec", tag = "3")]
    pub root: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MmrLeafProof {
    //// The index of the leaf the proof is for.
    #[prost(uint64, tag = "1")]
    pub leaf_index: u64,
    //// Number of leaves in MMR, when the proof was generated.
    #[prost(uint64, tag = "2")]
    pub leaf_count: u64,
    //// Proof elements (hashes of siblings of inner nodes on the path to the leaf).
    #[prost(bytes = "vec", repeated, tag = "3")]
    pub items: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// Client state
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    #[prost(string, tag = "1")]
    pub chain_id: ::prost::alloc::string::String,
    /// block_number is height?
    #[prost(uint32, tag = "2")]
    pub latest_height: u32,
    /// Block height when the client was frozen due to a misbehaviour
    #[prost(message, optional, tag = "3")]
    pub frozen_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
    #[prost(message, optional, tag = "4")]
    pub latest_commitment: ::core::option::Option<Commitment>,
    #[prost(message, optional, tag = "5")]
    pub validator_set: ::core::option::Option<ValidatorSet>,
}
// message InProcessState{
//  uint32 position = 1;
// 	bytes commitment_hash =2;
// 	SignedCommitment signed_commitment = 3;
// 	repeated ValidatorMerkleProof validator_proofs = 4;
// 	BeefyNextAuthoritySet validator_set = 5;
// }

/// Consensus state
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusState {
    /// bytes state_root = 3 ;
    /// /// The merkle root of the extrinsics.
    /// bytes extrinsics_root = 4 ;
    /// /// A chain-specific digest of data useful for light clients or referencing auxiliary data.
    /// bytes digest = 5 ;
    ///commitment: Option<Commitment>,used to verify mmr proof
    #[prost(message, optional, tag = "1")]
    pub commitment: ::core::option::Option<Commitment>,
    /// /// The state trie merkle root that used to verify storage proof
    #[prost(message, optional, tag = "2")]
    pub state_root: ::core::option::Option<super::super::super::core::commitment::v1::MerkleRoot>,
    /// timestamp that corresponds to the block height in which the ConsensusState
    /// was stored.
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<super::super::super::super::google::protobuf::Timestamp>,
}
/// Misbehaviour
/// The Misbehaviour type is used for detecting misbehaviour and freezing the client - to prevent further packet flow -
/// if applicable. GRANDPA client Misbehaviour consists of two headers at the same height both of which the light client
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Misbehaviour {
    #[prost(uint64, tag = "1")]
    pub client_id: u64,
    #[prost(message, optional, tag = "2")]
    pub header_1: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "3")]
    pub header_2: ::core::option::Option<Header>,
}
/// Block Header
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlockHeader {
    //// The parent hash.
    #[prost(bytes = "vec", tag = "1")]
    pub parent_hash: ::prost::alloc::vec::Vec<u8>,
    //// The block number.
    #[prost(uint32, tag = "2")]
    pub block_number: u32,
    //// The state trie merkle root
    #[prost(bytes = "vec", tag = "3")]
    pub state_root: ::prost::alloc::vec::Vec<u8>,
    //// The merkle root of the extrinsics.
    #[prost(bytes = "vec", tag = "4")]
    pub extrinsics_root: ::prost::alloc::vec::Vec<u8>,
    //// A chain-specific digest of data useful for light clients or referencing auxiliary data.
    #[prost(bytes = "vec", tag = "5")]
    pub digest: ::prost::alloc::vec::Vec<u8>,
}
///uint32 block_number = 1
///      [(gogoproto.nullable) = false, (gogoproto.moretags) = "yaml:\"block_number\""];
//// Block Header.
/// BlockHeader block_header = 1 [(gogoproto.nullable) = false];
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MmrRoot {
    #[prost(message, optional, tag = "1")]
    pub signed_commitment: ::core::option::Option<SignedCommitment>,
    #[prost(message, repeated, tag = "2")]
    pub validator_merkle_proofs: ::prost::alloc::vec::Vec<ValidatorMerkleProof>,
    #[prost(bytes = "vec", tag = "3")]
    pub mmr_leaf: ::prost::alloc::vec::Vec<u8>,
    #[prost(bytes = "vec", tag = "4")]
    pub mmr_leaf_proof: ::prost::alloc::vec::Vec<u8>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    //// Block Header.
    #[prost(message, optional, tag = "1")]
    pub block_header: ::core::option::Option<BlockHeader>,
    //// mmr root
    #[prost(message, optional, tag = "2")]
    pub mmr_root: ::core::option::Option<MmrRoot>,
    /// MmrLeaf mmr_leaf = 2
    ///       [(gogoproto.nullable) = false, (gogoproto.moretags) = "yaml:\"mmr_leaf\""];
    /// MmrLeafProof mmr_leaf_proof = 3
    ///       [(gogoproto.nullable) = false, (gogoproto.moretags) = "yaml:\"mmr_leaf_proof\""];
    //// block timestamp
    #[prost(message, optional, tag = "3")]
    pub timestamp: ::core::option::Option<super::super::super::super::google::protobuf::Timestamp>,
}

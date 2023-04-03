// chain type
// enum ChainType {
//    // subchain
//    CHAIN_TYPE_SOLOCHAIN_UNSPECIFIED = 0;
//    // parachain
//    CHAIN_TYPE_PARACHAIN = 1;
// }

/// ClientState from Beefy tracks the current validator set, latest height,
/// and a possible frozen height.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    /// 0: subchain
    /// 1: parachain
    #[prost(uint32, tag="1")]
    pub chain_type: u32,
    /// chain_id string type, eg: ibc-1
    #[prost(string, tag="2")]
    pub chain_id: ::prost::alloc::string::String,
    /// parachain id is uint type
    #[prost(uint32, tag="3")]
    pub parachain_id: u32,
    /// block number that the beefy protocol was activated on the relay chain.
    /// This should be the first block in the merkle-mountain-range tree.
    #[prost(uint32, tag="4")]
    pub beefy_activation_block: u32,
    /// the latest mmr_root_hash height
    #[prost(uint32, tag="5")]
    pub latest_beefy_height: u32,
    /// Latest mmr root hash
    #[prost(bytes="vec", tag="6")]
    pub mmr_root_hash: ::prost::alloc::vec::Vec<u8>,
    /// latest subchain or parachain height
    #[prost(uint32, tag="7")]
    pub latest_chain_height: u32,
    /// Block height when the client was frozen due to a misbehaviour
    #[prost(uint32, tag="8")]
    pub frozen_height: u32,
    /// authorities for the current round
    #[prost(message, optional, tag="9")]
    pub authority_set: ::core::option::Option<BeefyAuthoritySet>,
    /// authorities for the next round
    #[prost(message, optional, tag="10")]
    pub next_authority_set: ::core::option::Option<BeefyAuthoritySet>,
}
/// Beefy Authority Info
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeefyAuthoritySet {
    /// Id of the authority set, it should be strictly increasing
    #[prost(uint64, tag="1")]
    pub id: u64,
    /// Number of validators in the set.
    #[prost(uint32, tag="2")]
    pub len: u32,
    /// Merkle Root Hash build from BEEFY uncompressed AuthorityIds.
    #[prost(bytes="vec", tag="3")]
    pub root: ::prost::alloc::vec::Vec<u8>,
}
/// Actual payload items
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PayloadItem {
    /// 2-byte payload id
    #[prost(bytes="vec", tag="1")]
    pub id: ::prost::alloc::vec::Vec<u8>,
    /// arbitrary length payload data., eg mmr_root_hash
    #[prost(bytes="vec", tag="2")]
    pub data: ::prost::alloc::vec::Vec<u8>,
}
/// Commitment message signed by beefy validators
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Commitment {
    /// array of payload items signed by Beefy validators
    #[prost(message, repeated, tag="1")]
    pub payloads: ::prost::alloc::vec::Vec<PayloadItem>,
    /// block number for this commitment
    #[prost(uint32, tag="2")]
    pub block_number: u32,
    /// validator set that signed this commitment
    #[prost(uint64, tag="3")]
    pub validator_set_id: u64,
}
/// Signature with it`s index in merkle tree
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Signature {
    /// signature leaf index in the merkle tree.
    #[prost(uint32, tag="1")]
    pub index: u32,
    /// signature bytes
    #[prost(bytes="vec", tag="2")]
    pub signature: ::prost::alloc::vec::Vec<u8>,
}
/// signed commitment data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SignedCommitment {
    /// commitment data being signed
    #[prost(message, optional, tag="1")]
    pub commitment: ::core::option::Option<Commitment>,
    /// all the signatures
    #[prost(message, repeated, tag="2")]
    pub signatures: ::prost::alloc::vec::Vec<Signature>,
}
/// mmr data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BeefyMmr {
    /// signed commitment data
    #[prost(message, optional, tag="1")]
    pub signed_commitment: ::core::option::Option<SignedCommitment>,
    /// build merkle tree based on all the signature in signed commitment
    /// and generate the signature proof
    #[prost(bytes="vec", repeated, tag="2")]
    pub signature_proofs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// mmr proof
    #[prost(message, optional, tag="3")]
    pub mmr_leaves_and_batch_proof: ::core::option::Option<MmrLeavesAndBatchProof>,
    /// size of the mmr for the given proof
    #[prost(uint64, tag="4")]
    pub mmr_size: u64,
}
/// mmr leaves and proofs
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MmrLeavesAndBatchProof {
    /// mmr leaves
    #[prost(message, repeated, tag="1")]
    pub leaves: ::prost::alloc::vec::Vec<MmrLeaf>,
    /// mmr batch proof
    #[prost(message, optional, tag="2")]
    pub mmr_batch_proof: ::core::option::Option<MmrBatchProof>,
}
/// MmrLeaf leaf data
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MmrLeaf {
    /// leaf version
    #[prost(uint32, tag="1")]
    pub version: u32,
    /// // parent block for this leaf
    /// uint32 parent_number = 2;
    /// // parent hash for this leaf
    /// bytes parent_hash = 3;
    /// parent number and hash
    #[prost(message, optional, tag="2")]
    pub parent_number_and_hash: ::core::option::Option<ParentNumberAndHash>,
    /// beefy next authority set.
    #[prost(message, optional, tag="3")]
    pub beefy_next_authority_set: ::core::option::Option<BeefyAuthoritySet>,
    /// merkle root hash of parachain heads included in the leaf.
    #[prost(bytes="vec", tag="4")]
    pub parachain_heads: ::prost::alloc::vec::Vec<u8>,
}
/// parent number and hash
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParentNumberAndHash {
    /// parent block for this leaf
    #[prost(uint32, tag="1")]
    pub parent_number: u32,
    /// parent hash for this leaf
    #[prost(bytes="vec", tag="2")]
    pub parent_hash: ::prost::alloc::vec::Vec<u8>,
}
/// mmr batch proof
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MmrBatchProof {
    /// The index of the leaf the proof is for.
    #[prost(uint64, repeated, tag="1")]
    pub leaf_indexes: ::prost::alloc::vec::Vec<u64>,
    /// Number of leaves in MMR, when the proof was generated.
    #[prost(uint64, tag="2")]
    pub leaf_count: u64,
    /// Proof elements (hashes of siblings of inner nodes on the path to the leaf).
    #[prost(bytes="vec", repeated, tag="3")]
    pub items: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// ConsensusState
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusState {
    /// timestamp that corresponds to the block height in which the ConsensusState
    /// was stored.
    #[prost(message, optional, tag="1")]
    pub timestamp: ::core::option::Option<super::super::super::super::google::protobuf::Timestamp>,
    /// parachain header.state_root that used to verify chain storage proof
    #[prost(bytes="vec", tag="2")]
    pub root: ::prost::alloc::vec::Vec<u8>,
}
/// subchain header map
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubchainHeaderMap {
    /// LatestMMR latest_mmr = 1;
    /// map<blocknumber,scale-encoded blockheader>
    ///
    /// map<uint32,Timestamp> timestamp_map=2;
    #[prost(btree_map="uint32, message", tag="1")]
    pub subchain_header_map: ::prost::alloc::collections::BTreeMap<u32, SubchainHeader>,
}
/// subchain header
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SubchainHeader {
    /// scale-encoded subchain header bytes
    #[prost(bytes="vec", tag="1")]
    pub block_header: ::prost::alloc::vec::Vec<u8>,
    /// timestamp and proof
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<StateProof>,
}
/// / Parachain headers and their merkle proofs.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParachainHeaderMap {
    /// map<blocknumber,ParachainHeader>
    ///
    ///   map<uint32,Timestamp> timestamp_map=2;
    #[prost(btree_map="uint32, message", tag="1")]
    pub parachain_header_map: ::prost::alloc::collections::BTreeMap<u32, ParachainHeader>,
}
/// data needed to prove parachain header inclusion in mmr
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ParachainHeader {
    /// para id
    #[prost(uint32, tag="1")]
    pub parachain_id: u32,
    /// scale-encoded parachain header bytes
    #[prost(bytes="vec", tag="2")]
    pub block_header: ::prost::alloc::vec::Vec<u8>,
    /// proofs for parachain header in the mmr_leaf.parachain_heads
    #[prost(bytes="vec", repeated, tag="3")]
    pub proofs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
    /// merkle leaf index for parachain heads proof
    #[prost(uint32, tag="4")]
    pub header_index: u32,
    /// total number of para heads in parachain_heads_root
    #[prost(uint32, tag="5")]
    pub header_count: u32,
    /// timestamp and proof
    #[prost(message, optional, tag="6")]
    pub timestamp: ::core::option::Option<StateProof>,
}
/// state value and proof
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateProof {
    /// state key
    #[prost(bytes="vec", tag="1")]
    pub key: ::prost::alloc::vec::Vec<u8>,
    /// the state value
    #[prost(bytes="vec", tag="2")]
    pub value: ::prost::alloc::vec::Vec<u8>,
    /// these proof gets from parachain by rpc methord:state_getReadProof
    #[prost(bytes="vec", repeated, tag="3")]
    pub proofs: ::prost::alloc::vec::Vec<::prost::alloc::vec::Vec<u8>>,
}
/// header wrapper
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    /// the latest mmr data
    #[prost(message, optional, tag="1")]
    pub beefy_mmr: ::core::option::Option<BeefyMmr>,
    /// only one header
    #[prost(oneof="header::Message", tags="2, 3")]
    pub message: ::core::option::Option<header::Message>,
}
/// Nested message and enum types in `Header`.
pub mod header {
    /// only one header
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Message {
        /// subchain headers and their proofs
        #[prost(message, tag="2")]
        SubchainHeaderMap(super::SubchainHeaderMap),
        /// parachain headers and their proofs
        #[prost(message, tag="3")]
        ParachainHeaderMap(super::ParachainHeaderMap),
    }
}
/// Misbehaviour is a wrapper over two conflicting Headers
/// that implements Misbehaviour interface expected by ICS-02
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Misbehaviour {
    #[prost(string, tag="1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="2")]
    pub header_1: ::core::option::Option<Header>,
    #[prost(message, optional, tag="3")]
    pub header_2: ::core::option::Option<Header>,
}

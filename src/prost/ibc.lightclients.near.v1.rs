/// A NEAR protocol consensus header.
/// A NEAR protocol validator stake view.

#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CryptoHash {
    #[prost(bytes = "vec", tag = "1")]
    pub raw_data: ::prost::alloc::vec::Vec<u8>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Header {
    #[prost(bytes = "vec", tag = "1")]
    pub light_client_block: ::prost::alloc::vec::Vec<u8>,
    #[prost(message, repeated, tag = "2")]
    pub prev_state_root_of_chunks: ::prost::alloc::vec::Vec<CryptoHash>,
}
/// A NEAR protocol validator stake view.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ValidatorStakeView {
    #[prost(bytes = "vec", tag = "1")]
    pub raw_data: ::prost::alloc::vec::Vec<u8>,
}
/// A NEAR protocol consensus state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ConsensusState {
    #[prost(message, repeated, tag = "1")]
    pub current_bps: ::prost::alloc::vec::Vec<ValidatorStakeView>,
    #[prost(message, optional, tag = "2")]
    pub header: ::core::option::Option<Header>,
}
/// A NEAR protocol client state.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClientState {
    #[prost(message, optional, tag = "1")]
    pub trusting_period:
        ::core::option::Option<super::super::super::super::google::protobuf::Duration>,
    /// Block height when the client was frozen due to a misbehaviour
    #[prost(message, optional, tag = "2")]
    pub frozen_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
    /// Latest height the client was updated to
    #[prost(message, optional, tag = "3")]
    pub latest_height: ::core::option::Option<super::super::super::core::client::v1::Height>,
    /// Latest timestamp the client was updated to
    #[prost(uint64, tag = "4")]
    pub latest_timestamp: u64,
    ///
    #[prost(bytes = "vec", tag = "5")]
    pub upgrade_commitment_prefix: ::prost::alloc::vec::Vec<u8>,
    ///
    #[prost(bytes = "vec", tag = "6")]
    pub upgrade_key: ::prost::alloc::vec::Vec<u8>,
}

/// Misbehaviour is a wrapper over two conflicting Headers
/// that implements Misbehaviour interface expected by ICS-02
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Misbehaviour {
    /// ClientID is deprecated
    #[deprecated]
    #[prost(string, tag = "1")]
    pub client_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag = "2")]
    pub header_1: ::core::option::Option<Header>,
    #[prost(message, optional, tag = "3")]
    pub header_2: ::core::option::Option<Header>,
}

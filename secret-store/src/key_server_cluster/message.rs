// Copyright 2015-2020 Parity Technologies (UK) Ltd.
// This file is part of Parity Ethereum.

// Parity Ethereum is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Ethereum is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Ethereum.  If not, see <http://www.gnu.org/licenses/>.

use std::fmt;
use std::collections::{BTreeSet, BTreeMap};
use crypto::publickey::Secret;
use key_server_cluster::SessionId;
use super::{Error, SerializableH256, SerializablePublic, SerializableSecret,
	SerializableSignature, SerializableMessageHash, SerializableRequester, SerializableAddress};

pub type MessageSessionId = SerializableH256;
pub type MessageNodeId = SerializablePublic;

/// All possible messages that can be sent during encryption/decryption sessions.
#[derive(Clone, Debug)]
pub enum Message {
	/// Cluster message.
	Cluster(ClusterMessage),
	/// Key generation message.
	Generation(GenerationMessage),
	/// Encryption message.
	Encryption(EncryptionMessage),
	/// Decryption message.
	Decryption(DecryptionMessage),
	/// Schnorr signing message.
	SchnorrSigning(SchnorrSigningMessage),
	/// ECDSA signing message.
	EcdsaSigning(EcdsaSigningMessage),
	/// Key version negotiation message.
	KeyVersionNegotiation(KeyVersionNegotiationMessage),
	/// Share add message.
	ShareAdd(ShareAddMessage),
	/// Servers set change message.
	ServersSetChange(ServersSetChangeMessage),
}

/// All possible cluster-level messages.
#[derive(Clone, Debug)]
pub enum ClusterMessage {
	/// Introduce node public key.
	NodePublicKey(NodePublicKey),
	/// Confirm that node owns its private key.
	NodePrivateKeySignature(NodePrivateKeySignature),
	/// Keep alive message.
	KeepAlive(KeepAlive),
	/// Keep alive message response.
	KeepAliveResponse(KeepAliveResponse),
}

/// All possible messages that can be sent during key generation session.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum GenerationMessage {
	/// Initialize new DKG session.
	InitializeSession(InitializeSession),
	/// Confirm DKG session initialization.
	ConfirmInitialization(ConfirmInitialization),
	/// Broadcast data, calculated during session initialization phase.
	CompleteInitialization(CompleteInitialization),
	/// Generated keys are sent to every node.
	KeysDissemination(KeysDissemination),
	/// Broadcast self public key portion.
	PublicKeyShare(PublicKeyShare),
	/// When session error has occured.
	SessionError(SessionError),
	/// When session is completed.
	SessionCompleted(SessionCompleted),
}

/// All possible messages that can be sent during encryption session.
#[derive(Clone, Debug)]
pub enum EncryptionMessage {
	/// Initialize encryption session.
	InitializeEncryptionSession(InitializeEncryptionSession),
	/// Confirm/reject encryption session initialization.
	ConfirmEncryptionInitialization(ConfirmEncryptionInitialization),
	/// When encryption session error has occured.
	EncryptionSessionError(EncryptionSessionError),
}

/// All possible messages that can be sent during consensus establishing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConsensusMessage {
	/// Initialize consensus session.
	InitializeConsensusSession(InitializeConsensusSession),
	/// Confirm/reject consensus session initialization.
	ConfirmConsensusInitialization(ConfirmConsensusInitialization),
}

/// All possible messages that can be sent during servers-set consensus establishing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConsensusMessageWithServersSet {
	/// Initialize consensus session.
	InitializeConsensusSession(InitializeConsensusSessionWithServersSet),
	/// Confirm/reject consensus session initialization.
	ConfirmConsensusInitialization(ConfirmConsensusInitialization),
}

/// All possible messages that can be sent during share add consensus establishing.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ConsensusMessageOfShareAdd {
	/// Initialize consensus session.
	InitializeConsensusSession(InitializeConsensusSessionOfShareAdd),
	/// Confirm/reject consensus session initialization.
	ConfirmConsensusInitialization(ConfirmConsensusInitialization),
}

/// All possible messages that can be sent during decryption session.
#[derive(Clone, Debug)]
pub enum DecryptionMessage {
	/// Consensus establishing message.
	DecryptionConsensusMessage(DecryptionConsensusMessage),
	/// Request partial decryption from node.
	RequestPartialDecryption(RequestPartialDecryption),
	/// Partial decryption is completed.
	PartialDecryption(PartialDecryption),
	/// When decryption session error has occured.
	DecryptionSessionError(DecryptionSessionError),
	/// When decryption session is completed.
	DecryptionSessionCompleted(DecryptionSessionCompleted),
	/// When decryption session is delegated to another node.
	DecryptionSessionDelegation(DecryptionSessionDelegation),
	/// When delegated decryption session is completed.
	DecryptionSessionDelegationCompleted(DecryptionSessionDelegationCompleted),
}

/// All possible messages that can be sent during Schnorr signing session.
#[derive(Clone, Debug)]
pub enum SchnorrSigningMessage {
	/// Consensus establishing message.
	SchnorrSigningConsensusMessage(SchnorrSigningConsensusMessage),
	/// Session key generation message.
	SchnorrSigningGenerationMessage(SchnorrSigningGenerationMessage),
	/// Request partial signature from node.
	SchnorrRequestPartialSignature(SchnorrRequestPartialSignature),
	/// Partial signature is generated.
	SchnorrPartialSignature(SchnorrPartialSignature),
	/// Signing error occured.
	SchnorrSigningSessionError(SchnorrSigningSessionError),
	/// Signing session completed.
	SchnorrSigningSessionCompleted(SchnorrSigningSessionCompleted),
	/// When signing session is delegated to another node.
	SchnorrSigningSessionDelegation(SchnorrSigningSessionDelegation),
	/// When delegated signing session is completed.
	SchnorrSigningSessionDelegationCompleted(SchnorrSigningSessionDelegationCompleted),
}

/// All possible messages that can be sent during ECDSA signing session.
#[derive(Clone, Debug)]
pub enum EcdsaSigningMessage {
	/// Consensus establishing message.
	EcdsaSigningConsensusMessage(EcdsaSigningConsensusMessage),
	/// Signature nonce generation message.
	EcdsaSignatureNonceGenerationMessage(EcdsaSignatureNonceGenerationMessage),
	/// Inversion nonce generation message.
	EcdsaInversionNonceGenerationMessage(EcdsaInversionNonceGenerationMessage),
	/// Inversion zero generation message.
	EcdsaInversionZeroGenerationMessage(EcdsaInversionZeroGenerationMessage),
	/// Inversed nonce coefficient share.
	EcdsaSigningInversedNonceCoeffShare(EcdsaSigningInversedNonceCoeffShare),
	/// Request partial signature from node.
	EcdsaRequestPartialSignature(EcdsaRequestPartialSignature),
	/// Partial signature is generated.
	EcdsaPartialSignature(EcdsaPartialSignature),
	/// Signing error occured.
	EcdsaSigningSessionError(EcdsaSigningSessionError),
	/// Signing session completed.
	EcdsaSigningSessionCompleted(EcdsaSigningSessionCompleted),
	/// When signing session is delegated to another node.
	EcdsaSigningSessionDelegation(EcdsaSigningSessionDelegation),
	/// When delegated signing session is completed.
	EcdsaSigningSessionDelegationCompleted(EcdsaSigningSessionDelegationCompleted),
}

/// All possible messages that can be sent during servers set change session.
#[derive(Clone, Debug)]
pub enum ServersSetChangeMessage {
	/// Consensus establishing message.
	ServersSetChangeConsensusMessage(ServersSetChangeConsensusMessage),
	/// Unknown sessions ids request.
	UnknownSessionsRequest(UnknownSessionsRequest),
	/// Unknown sessions ids.
	UnknownSessions(UnknownSessions),
	/// Negotiating key version to use as a base for ShareAdd session.
	ShareChangeKeyVersionNegotiation(ShareChangeKeyVersionNegotiation),
	/// Initialize share change session(s).
	InitializeShareChangeSession(InitializeShareChangeSession),
	/// Confirm share change session(s) initialization.
	ConfirmShareChangeSessionInitialization(ConfirmShareChangeSessionInitialization),
	/// Share change session delegation.
	ServersSetChangeDelegate(ServersSetChangeDelegate),
	/// Share change session delegation response.
	ServersSetChangeDelegateResponse(ServersSetChangeDelegateResponse),
	/// Share add message.
	ServersSetChangeShareAddMessage(ServersSetChangeShareAddMessage),
	/// Servers set change session completed.
	ServersSetChangeError(ServersSetChangeError),
	/// Servers set change session completed.
	ServersSetChangeCompleted(ServersSetChangeCompleted),
}

/// All possible messages that can be sent during share add session.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum ShareAddMessage {
	/// Consensus establishing message.
	ShareAddConsensusMessage(ShareAddConsensusMessage),
	/// Common key share data is sent to new node.
	KeyShareCommon(KeyShareCommon),
	/// Generated keys are sent to every node.
	NewKeysDissemination(NewKeysDissemination),
	/// When session error has occured.
	ShareAddError(ShareAddError),
}

/// All possible messages that can be sent during key version negotiation message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum KeyVersionNegotiationMessage {
	/// Request key versions.
	RequestKeyVersions(RequestKeyVersions),
	/// Key versions.
	KeyVersions(KeyVersions),
	/// When session error has occured.
	KeyVersionsError(KeyVersionsError),
}

/// Introduce node public key.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodePublicKey {
	/// Node identifier (aka node public key).
	pub node_id: MessageNodeId,
	/// Random data, which must be signed by peer to prove that he owns the corresponding private key.
	pub confirmation_plain: SerializableH256,
	/// The same random `confirmation_plain`, signed with one-time session key.
	pub confirmation_signed_session: SerializableSignature,
}

/// Confirm that node owns the private key of previously passed public key (aka node id).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodePrivateKeySignature {
	/// Previously passed `confirmation_plain`, signed with node private key.
	pub confirmation_signed: SerializableSignature,
}

/// Ask if the node is still alive.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeepAlive {
}

/// Confirm that the node is still alive.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeepAliveResponse {
	/// Session id, if used for session-level keep alive.
	pub session_id: Option<MessageSessionId>,
}

/// Initialize new DKG session.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeSession {
	/// Session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Session origin address (if any).
	pub origin: Option<SerializableAddress>,
	/// Session author.
	pub author: SerializableAddress,
	/// All session participants along with their identification numbers.
	pub nodes: BTreeMap<MessageNodeId, SerializableSecret>,
	/// Is zero secret generation session?
	pub is_zero: bool,
	/// Decryption threshold. During decryption threshold-of-route.len() nodes must came to
	/// consensus to successfully decrypt message.
	pub threshold: usize,
	/// Derived generation point. Starting from originator, every node must multiply this
	/// point by random scalar (unknown by other nodes). At the end of initialization
	/// `point` will be some (k1 * k2 * ... * kn) * G = `point` where `(k1 * k2 * ... * kn)`
	/// is unknown for every node.
	pub derived_point: SerializablePublic,
}

/// Confirm DKG session initialization.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfirmInitialization {
	/// Session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Derived generation point.
	pub derived_point: SerializablePublic,
}

/// Broadcast generated point to every other node.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompleteInitialization {
	/// Session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Derived generation point.
	pub derived_point: SerializablePublic,
}

/// Generated keys are sent to every node.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeysDissemination {
	/// Session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Secret 1.
	pub secret1: SerializableSecret,
	/// Secret 2.
	pub secret2: SerializableSecret,
	/// Public values.
	pub publics: Vec<SerializablePublic>,
}

/// Node is sharing its public key share.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PublicKeyShare {
	/// Session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Public key share.
	pub public_share: SerializablePublic,
}

/// When session error has occured.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionError {
	/// Session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Error message.
	pub error: Error,
}

/// When session is completed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionCompleted {
	/// Session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
}

/// Node is requested to prepare for saving encrypted data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeEncryptionSession {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Requester.
	pub requester: SerializableRequester,
	/// Common point.
	pub common_point: SerializablePublic,
	/// Encrypted data.
	pub encrypted_point: SerializablePublic,
}

/// Node is responding to encryption initialization request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfirmEncryptionInitialization {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
}

/// When encryption session error has occured.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EncryptionSessionError {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Error message.
	pub error: Error,
}

/// Node is asked to be part of consensus group.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeConsensusSession {
	/// Requester.
	pub requester: SerializableRequester,
	/// Key version.
	pub version: SerializableH256,
}

/// Node is responding to consensus initialization request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfirmConsensusInitialization {
	/// Is node confirmed consensus participation.
	pub is_confirmed: bool,
}

/// Node is asked to be part of servers-set consensus group.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeConsensusSessionWithServersSet {
	/// Migration id (if any).
	pub migration_id: Option<SerializableH256>,
	/// Old nodes set.
	pub old_nodes_set: BTreeSet<MessageNodeId>,
	/// New nodes set.
	pub new_nodes_set: BTreeSet<MessageNodeId>,
	/// Old server set, signed by requester.
	pub old_set_signature: SerializableSignature,
	/// New server set, signed by requester.
	pub new_set_signature: SerializableSignature,
}

/// Node is asked to be part of servers-set consensus group.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeConsensusSessionOfShareAdd {
	/// Key version.
	pub version: SerializableH256,
	/// Nodes that have reported version ownership.
	pub version_holders: BTreeSet<MessageNodeId>,
	/// threshold+1 nodes from old_nodes_set selected for shares redistribution.
	pub consensus_group: BTreeSet<MessageNodeId>,
	/// Old nodes set: all non-isolated owners of selected key share version.
	pub old_nodes_set: BTreeSet<MessageNodeId>,
	/// New nodes map: node id => node id number.
	pub new_nodes_map: BTreeMap<MessageNodeId, SerializableSecret>,
	/// Old server set, signed by requester.
	pub old_set_signature: SerializableSignature,
	/// New server set, signed by requester.
	pub new_set_signature: SerializableSignature,
}

/// Consensus-related Schnorr signing message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchnorrSigningConsensusMessage {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Consensus message.
	pub message: ConsensusMessage,
}

/// Session key generation message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchnorrSigningGenerationMessage {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Generation message.
	pub message: GenerationMessage,
}

/// Request partial Schnorr signature.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchnorrRequestPartialSignature {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Request id.
	pub request_id: SerializableSecret,
	/// Message hash.
	pub message_hash: SerializableMessageHash,
	/// Selected nodes.
	pub nodes: BTreeSet<MessageNodeId>,
}

/// Partial Schnorr signature.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchnorrPartialSignature {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Request id.
	pub request_id: SerializableSecret,
	/// S part of signature.
	pub partial_signature: SerializableSecret,
}

/// When Schnorr signing session error has occured.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchnorrSigningSessionError {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Error message.
	pub error: Error,
}

/// Schnorr signing session completed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchnorrSigningSessionCompleted {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
}

/// When Schnorr signing session is delegated to another node.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchnorrSigningSessionDelegation {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Decryption session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Requester.
	pub requester: SerializableRequester,
	/// Key version.
	pub version: SerializableH256,
	/// Message hash.
	pub message_hash: SerializableH256,
}

/// When delegated Schnorr signing session is completed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SchnorrSigningSessionDelegationCompleted {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Decryption session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// S-portion of signature.
	pub signature_s: SerializableSecret,
	/// C-portion of signature.
	pub signature_c: SerializableSecret,
}

/// Consensus-related ECDSA signing message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcdsaSigningConsensusMessage {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Consensus message.
	pub message: ConsensusMessage,
}

/// ECDSA signature nonce generation message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcdsaSignatureNonceGenerationMessage {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Generation message.
	pub message: GenerationMessage,
}

/// ECDSA inversion nonce generation message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcdsaInversionNonceGenerationMessage {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Generation message.
	pub message: GenerationMessage,
}

/// ECDSA inversed nonce share message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcdsaSigningInversedNonceCoeffShare {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Inversed nonce coefficient share.
	pub inversed_nonce_coeff_share: SerializableSecret,
}

/// ECDSA inversion zero generation message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcdsaInversionZeroGenerationMessage {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Generation message.
	pub message: GenerationMessage,
}

/// Request partial ECDSA signature.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcdsaRequestPartialSignature {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Request id.
	pub request_id: SerializableSecret,
	/// ECDSA reversed-nonce coefficient
	pub inversed_nonce_coeff: SerializableSecret,
	/// Message hash.
	pub message_hash: SerializableMessageHash,
}

/// Partial ECDSA signature.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcdsaPartialSignature {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Request id.
	pub request_id: SerializableSecret,
	/// Partial S part of signature.
	pub partial_signature_s: SerializableSecret,
}

/// When ECDSA signing session error has occured.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcdsaSigningSessionError {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Error message.
	pub error: Error,
}

/// ECDSA signing session completed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcdsaSigningSessionCompleted {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
}

/// When ECDSA signing session is delegated to another node.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcdsaSigningSessionDelegation {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Decryption session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Requestor signature.
	pub requester: SerializableRequester,
	/// Key version.
	pub version: SerializableH256,
	/// Message hash.
	pub message_hash: SerializableH256,
}

/// When delegated ECDSA signing session is completed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EcdsaSigningSessionDelegationCompleted {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Decryption session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Signature.
	pub signature: SerializableSignature,
}

/// Consensus-related decryption message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptionConsensusMessage {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Signing session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Session origin (in consensus initialization message).
	pub origin: Option<SerializableAddress>,
	/// Consensus message.
	pub message: ConsensusMessage,
}

/// Node is requested to do a partial decryption.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestPartialDecryption {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Decryption session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Request id.
	pub request_id: SerializableSecret,
	/// Is shadow decryption requested? When true, decryption result
	/// will be visible to the owner of requestor public key only.
	pub is_shadow_decryption: bool,
	/// Decryption result must be reconstructed on all participating nodes. This is useful
	/// for service contract API so that all nodes from consensus group can confirm decryption.
	pub is_broadcast_session: bool,
	/// Nodes that are agreed to do a decryption.
	pub nodes: BTreeSet<MessageNodeId>,
}

/// Node has partially decrypted the secret.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PartialDecryption {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Decryption session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Request id.
	pub request_id: SerializableSecret,
	/// Partially decrypted secret.
	pub shadow_point: SerializablePublic,
	/// Decrypt shadow coefficient (if requested), encrypted with requestor public.
	pub decrypt_shadow: Option<Vec<u8>>,
}

/// When decryption session error has occured.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptionSessionError {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Decryption session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Error message.
	pub error: Error,
}

/// When decryption session is completed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptionSessionCompleted {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Decryption session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
}

/// When decryption session is delegated to another node.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptionSessionDelegation {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Decryption session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Session origin.
	pub origin: Option<SerializableAddress>,
	/// Requester.
	pub requester: SerializableRequester,
	/// Key version.
	pub version: SerializableH256,
	/// Is shadow decryption requested? When true, decryption result
	/// will be visible to the owner of requestor public key only.
	pub is_shadow_decryption: bool,
	/// Decryption result must be reconstructed on all participating nodes. This is useful
	/// for service contract API so that all nodes from consensus group can confirm decryption.
	pub is_broadcast_session: bool,
}

/// When delegated decryption session is completed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct DecryptionSessionDelegationCompleted {
	/// Encryption session Id.
	pub session: MessageSessionId,
	/// Decryption session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Decrypted secret point. It is partially decrypted if shadow decrpytion was requested.
	pub decrypted_secret: SerializablePublic,
	/// Shared common point.
	pub common_point: Option<SerializablePublic>,
	/// If shadow decryption was requested: shadow decryption coefficients, encrypted with requestor public.
	pub decrypt_shadows: Option<Vec<Vec<u8>>>,
}

/// Consensus-related servers set change message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServersSetChangeConsensusMessage {
	/// Servers set change session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Consensus message.
	pub message: ConsensusMessageWithServersSet,
}

/// Unknown sessions ids request.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnknownSessionsRequest {
	/// Servers set change session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
}

/// Unknown session ids.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UnknownSessions {
	/// Servers set change session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Unknown session id.
	pub unknown_sessions: BTreeSet<MessageSessionId>,
}

/// Key version negotiation message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShareChangeKeyVersionNegotiation {
	/// Servers set change session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Key version negotiation message.
	pub message: KeyVersionNegotiationMessage,
}

/// Master node opens share initialize session on other nodes.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct InitializeShareChangeSession {
	/// Servers set change session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Key id.
	pub key_id: MessageSessionId,
	/// Key vesion to use in ShareAdd session.
	pub version: SerializableH256,
	/// Nodes that have confirmed version ownership.
	pub version_holders: BTreeSet<MessageNodeId>,
	/// Master node.
	pub master_node_id: MessageNodeId,
	/// Consensus group to use in ShareAdd session.
	pub consensus_group: BTreeSet<MessageNodeId>,
	/// Shares to add. Values are filled for new nodes only.
	pub new_nodes_map: BTreeMap<MessageNodeId, Option<SerializableSecret>>,
}

/// Slave node confirms session initialization.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConfirmShareChangeSessionInitialization {
	/// Servers set change session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Sessions that are confirmed.
	pub key_id: MessageSessionId,
}

/// Share change is requested.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServersSetChangeDelegate {
	/// Servers set change session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Key id.
	pub key_id: MessageSessionId,
}

/// Share change is completed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServersSetChangeDelegateResponse {
	/// Servers set change session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Key id.
	pub key_id: MessageSessionId,
}

/// Servers set change share add message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServersSetChangeShareAddMessage {
	/// Servers set change session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Unknown session id.
	pub message: ShareAddMessage,
}

/// When servers set change session error has occured.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServersSetChangeError {
	/// Servers set change session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Error message.
	pub error: Error,
}

/// When servers set change session is completed.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ServersSetChangeCompleted {
	/// Servers set change session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
}

/// Consensus-related share add session message.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShareAddConsensusMessage {
	/// Share add session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Consensus message.
	pub message: ConsensusMessageOfShareAdd,
}

/// Key share common data is passed to new node.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyShareCommon {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Common key data.
	pub key_common: CommonKeyData,
	/// Common (shared) encryption point.
	pub common_point: Option<SerializablePublic>,
	/// Encrypted point.
	pub encrypted_point: Option<SerializablePublic>,
	/// Selected version id numbers.
	pub id_numbers: BTreeMap<MessageNodeId, SerializableSecret>,
}

/// Generated keys are sent to every node.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NewKeysDissemination {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Sub share of rcevier' secret share.
	pub secret_subshare: SerializableSecret,
}

/// When share add session error has occured.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShareAddError {
	/// Generation session Id.
	pub session: MessageSessionId,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Error message.
	pub error: Error,
}

/// Key versions are requested.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RequestKeyVersions {
	/// Generation session id.
	pub session: MessageSessionId,
	/// Version negotiation session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
}

/// Key versions are sent.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyVersions {
	/// Generation session id.
	pub session: MessageSessionId,
	/// Version negotiation session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Common key data, shared by all versions.
	pub key_common: Option<CommonKeyData>,
	/// Key versions.
	pub versions: Vec<SerializableH256>,
}

/// Common key data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CommonKeyData {
	/// Key threshold.
	pub threshold: usize,
	/// Author of the key entry.
	pub author: SerializableAddress,
	/// Joint public.
	pub public: SerializablePublic,
}

/// When key versions error has occured.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct KeyVersionsError {
	/// Generation session id.
	pub session: MessageSessionId,
	/// Version negotiation session Id.
	pub sub_session: SerializableSecret,
	/// Session-level nonce.
	pub session_nonce: u64,
	/// Error message.
	pub error: Error,
	/// Continue action from failed node (if any). This field is oly filled
	/// when error has occured when trying to compute result on master node.
	pub continue_with: Option<FailedKeyVersionContinueAction>,
}

/// Key version continue action from failed node.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum FailedKeyVersionContinueAction {
	/// Decryption session: origin + requester.
	Decrypt(Option<SerializableAddress>, SerializableAddress),
}

impl Message {
	pub fn is_initialization_message(&self) -> bool {
		match self {
			Self::Generation(GenerationMessage::InitializeSession(_)) => true,
			Self::Encryption(EncryptionMessage::InitializeEncryptionSession(_)) => true,
			Self::Decryption(DecryptionMessage::DecryptionConsensusMessage(msg)) => match msg.message {
				ConsensusMessage::InitializeConsensusSession(_) => true,
				_ => false
			},
			Self::SchnorrSigning(SchnorrSigningMessage::SchnorrSigningConsensusMessage(msg)) => match msg.message {
				ConsensusMessage::InitializeConsensusSession(_) => true,
				_ => false
			},
			Self::EcdsaSigning(EcdsaSigningMessage::EcdsaSigningConsensusMessage(msg)) => match msg.message {
				ConsensusMessage::InitializeConsensusSession(_) => true,
				_ => false
			},
			Self::KeyVersionNegotiation(KeyVersionNegotiationMessage::RequestKeyVersions(_)) => true,
			Self::KeyVersionNegotiation(KeyVersionNegotiationMessage::KeyVersionsError(msg)) if msg.continue_with.is_some() => true,
			Self::ShareAdd(ShareAddMessage::ShareAddConsensusMessage(msg)) => match msg.message {
				ConsensusMessageOfShareAdd::InitializeConsensusSession(_) => true,
				_ => false
			},
			Self::ServersSetChange(ServersSetChangeMessage::ServersSetChangeConsensusMessage(msg)) => match msg.message {
				ConsensusMessageWithServersSet::InitializeConsensusSession(_) => true,
				_ => false
			},
			_ => false,
		}
	}

	pub fn is_delegation_message(&self) -> bool {
		match self {
			Self::Decryption(DecryptionMessage::DecryptionSessionDelegation(_)) => true,
			Self::SchnorrSigning(SchnorrSigningMessage::SchnorrSigningSessionDelegation(_)) => true,
			Self::EcdsaSigning(EcdsaSigningMessage::EcdsaSigningSessionDelegation(_)) => true,
			_ => false,
		}
	}

	pub fn is_error_message(&self) -> bool {
		match self {
			Self::Generation(GenerationMessage::SessionError(_)) => true,
			Self::Encryption(EncryptionMessage::EncryptionSessionError(_)) => true,
			Self::Decryption(DecryptionMessage::DecryptionSessionError(_)) => true,
			Self::SchnorrSigning(SchnorrSigningMessage::SchnorrSigningSessionError(_)) => true,
			Self::EcdsaSigning(EcdsaSigningMessage::EcdsaSigningSessionError(_)) => true,
			Self::KeyVersionNegotiation(KeyVersionNegotiationMessage::KeyVersionsError(_)) => true,
			Self::ShareAdd(ShareAddMessage::ShareAddError(_)) => true,
			Self::ServersSetChange(ServersSetChangeMessage::ServersSetChangeError(_)) => true,
			_ => false,
		}
	}

	pub fn is_exclusive_session_message(&self) -> bool {
		match self {
			Self::ServersSetChange(_) => true,
			_ => false,
		}
	}

	pub fn session_nonce(&self) -> Option<u64> {
		match self {
			Self::Cluster(_) => None,
			Self::Generation(message) => Some(message.session_nonce()),
			Self::Encryption(message) => Some(message.session_nonce()),
			Self::Decryption(message) => Some(message.session_nonce()),
			Self::SchnorrSigning(message) => Some(message.session_nonce()),
			Self::EcdsaSigning(message) => Some(message.session_nonce()),
			Self::ShareAdd(message) => Some(message.session_nonce()),
			Self::ServersSetChange(message) => Some(message.session_nonce()),
			Self::KeyVersionNegotiation(message) => Some(message.session_nonce()),
		}
	}
}

impl GenerationMessage {
	pub fn session_id(&self) -> &SessionId {
		match self {
			Self::InitializeSession(msg) => &msg.session,
			Self::ConfirmInitialization(msg) => &msg.session,
			Self::CompleteInitialization(msg) => &msg.session,
			Self::KeysDissemination(msg) => &msg.session,
			Self::PublicKeyShare(msg) => &msg.session,
			Self::SessionError(msg) => &msg.session,
			Self::SessionCompleted(msg) => &msg.session,
		}
	}

	pub fn session_nonce(&self) -> u64 {
		match self {
			Self::InitializeSession(msg) => msg.session_nonce,
			Self::ConfirmInitialization(msg) => msg.session_nonce,
			Self::CompleteInitialization(msg) => msg.session_nonce,
			Self::KeysDissemination(msg) => msg.session_nonce,
			Self::PublicKeyShare(msg) => msg.session_nonce,
			Self::SessionError(msg) => msg.session_nonce,
			Self::SessionCompleted(msg) => msg.session_nonce,
		}
	}
}

impl EncryptionMessage {
	pub fn session_id(&self) -> &SessionId {
		match self {
			Self::InitializeEncryptionSession(msg) => &msg.session,
			Self::ConfirmEncryptionInitialization(msg) => &msg.session,
			Self::EncryptionSessionError(msg) => &msg.session,
		}
	}

	pub fn session_nonce(&self) -> u64 {
		match self {
			Self::InitializeEncryptionSession(msg) => msg.session_nonce,
			Self::ConfirmEncryptionInitialization(msg) => msg.session_nonce,
			Self::EncryptionSessionError(msg) => msg.session_nonce,
		}
	}
}

impl DecryptionMessage {
	pub fn session_id(&self) -> &SessionId {
		match self {
			Self::DecryptionConsensusMessage(msg) => &msg.session,
			Self::RequestPartialDecryption(msg) => &msg.session,
			Self::PartialDecryption(msg) => &msg.session,
			Self::DecryptionSessionError(msg) => &msg.session,
			Self::DecryptionSessionCompleted(msg) => &msg.session,
			Self::DecryptionSessionDelegation(msg) => &msg.session,
			Self::DecryptionSessionDelegationCompleted(msg) => &msg.session,
		}
	}

	pub fn sub_session_id(&self) -> &Secret {
		match self {
			Self::DecryptionConsensusMessage(msg) => &msg.sub_session,
			Self::RequestPartialDecryption(msg) => &msg.sub_session,
			Self::PartialDecryption(msg) => &msg.sub_session,
			Self::DecryptionSessionError(msg) => &msg.sub_session,
			Self::DecryptionSessionCompleted(msg) => &msg.sub_session,
			Self::DecryptionSessionDelegation(msg) => &msg.sub_session,
			Self::DecryptionSessionDelegationCompleted(msg) => &msg.sub_session,
		}
	}

	pub fn session_nonce(&self) -> u64 {
		match self {
			Self::DecryptionConsensusMessage(msg) => msg.session_nonce,
			Self::RequestPartialDecryption(msg) => msg.session_nonce,
			Self::PartialDecryption(msg) => msg.session_nonce,
			Self::DecryptionSessionError(msg) => msg.session_nonce,
			Self::DecryptionSessionCompleted(msg) => msg.session_nonce,
			Self::DecryptionSessionDelegation(msg) => msg.session_nonce,
			Self::DecryptionSessionDelegationCompleted(msg) => msg.session_nonce,
		}
	}
}

impl SchnorrSigningMessage {
	pub fn session_id(&self) -> &SessionId {
		match self {
			Self::SchnorrSigningConsensusMessage(msg) => &msg.session,
			Self::SchnorrSigningGenerationMessage(msg) => &msg.session,
			Self::SchnorrRequestPartialSignature(msg) => &msg.session,
			Self::SchnorrPartialSignature(msg) => &msg.session,
			Self::SchnorrSigningSessionError(msg) => &msg.session,
			Self::SchnorrSigningSessionCompleted(msg) => &msg.session,
			Self::SchnorrSigningSessionDelegation(msg) => &msg.session,
			Self::SchnorrSigningSessionDelegationCompleted(msg) => &msg.session,
		}
	}

	pub fn sub_session_id(&self) -> &Secret {
		match self {
			Self::SchnorrSigningConsensusMessage(msg) => &msg.sub_session,
			Self::SchnorrSigningGenerationMessage(msg) => &msg.sub_session,
			Self::SchnorrRequestPartialSignature(msg) => &msg.sub_session,
			Self::SchnorrPartialSignature(msg) => &msg.sub_session,
			Self::SchnorrSigningSessionError(msg) => &msg.sub_session,
			Self::SchnorrSigningSessionCompleted(msg) => &msg.sub_session,
			Self::SchnorrSigningSessionDelegation(msg) => &msg.sub_session,
			Self::SchnorrSigningSessionDelegationCompleted(msg) => &msg.sub_session,
		}
	}

	pub fn session_nonce(&self) -> u64 {
		match self {
			Self::SchnorrSigningConsensusMessage(msg) => msg.session_nonce,
			Self::SchnorrSigningGenerationMessage(msg) => msg.session_nonce,
			Self::SchnorrRequestPartialSignature(msg) => msg.session_nonce,
			Self::SchnorrPartialSignature(msg) => msg.session_nonce,
			Self::SchnorrSigningSessionError(msg) => msg.session_nonce,
			Self::SchnorrSigningSessionCompleted(msg) => msg.session_nonce,
			Self::SchnorrSigningSessionDelegation(msg) => msg.session_nonce,
			Self::SchnorrSigningSessionDelegationCompleted(msg) => msg.session_nonce,
		}
	}
}

impl EcdsaSigningMessage {
	pub fn session_id(&self) -> &SessionId {
		match self {
			Self::EcdsaSigningConsensusMessage(msg) => &msg.session,
			Self::EcdsaSignatureNonceGenerationMessage(msg) => &msg.session,
			Self::EcdsaInversionNonceGenerationMessage(msg) => &msg.session,
			Self::EcdsaInversionZeroGenerationMessage(msg) => &msg.session,
			Self::EcdsaSigningInversedNonceCoeffShare(msg) => &msg.session,
			Self::EcdsaRequestPartialSignature(msg) => &msg.session,
			Self::EcdsaPartialSignature(msg) => &msg.session,
			Self::EcdsaSigningSessionError(msg) => &msg.session,
			Self::EcdsaSigningSessionCompleted(msg) => &msg.session,
			Self::EcdsaSigningSessionDelegation(msg) => &msg.session,
			Self::EcdsaSigningSessionDelegationCompleted(msg) => &msg.session,
		}
	}

	pub fn sub_session_id(&self) -> &Secret {
		match self {
			Self::EcdsaSigningConsensusMessage(msg) => &msg.sub_session,
			Self::EcdsaSignatureNonceGenerationMessage(msg) => &msg.sub_session,
			Self::EcdsaInversionNonceGenerationMessage(msg) => &msg.sub_session,
			Self::EcdsaInversionZeroGenerationMessage(msg) => &msg.sub_session,
			Self::EcdsaSigningInversedNonceCoeffShare(msg) => &msg.sub_session,
			Self::EcdsaRequestPartialSignature(msg) => &msg.sub_session,
			Self::EcdsaPartialSignature(msg) => &msg.sub_session,
			Self::EcdsaSigningSessionError(msg) => &msg.sub_session,
			Self::EcdsaSigningSessionCompleted(msg) => &msg.sub_session,
			Self::EcdsaSigningSessionDelegation(msg) => &msg.sub_session,
			Self::EcdsaSigningSessionDelegationCompleted(msg) => &msg.sub_session,
		}
	}

	pub fn session_nonce(&self) -> u64 {
		match self {
			Self::EcdsaSigningConsensusMessage(msg) => msg.session_nonce,
			Self::EcdsaSignatureNonceGenerationMessage(msg) => msg.session_nonce,
			Self::EcdsaInversionNonceGenerationMessage(msg) => msg.session_nonce,
			Self::EcdsaInversionZeroGenerationMessage(msg) => msg.session_nonce,
			Self::EcdsaSigningInversedNonceCoeffShare(msg) => msg.session_nonce,
			Self::EcdsaRequestPartialSignature(msg) => msg.session_nonce,
			Self::EcdsaPartialSignature(msg) => msg.session_nonce,
			Self::EcdsaSigningSessionError(msg) => msg.session_nonce,
			Self::EcdsaSigningSessionCompleted(msg) => msg.session_nonce,
			Self::EcdsaSigningSessionDelegation(msg) => msg.session_nonce,
			Self::EcdsaSigningSessionDelegationCompleted(msg) => msg.session_nonce,
		}
	}
}

impl ServersSetChangeMessage {
	pub fn session_id(&self) -> &SessionId {
		match self {
			Self::ServersSetChangeConsensusMessage(msg) => &msg.session,
			Self::UnknownSessionsRequest(msg) => &msg.session,
			Self::UnknownSessions(msg) => &msg.session,
			Self::ShareChangeKeyVersionNegotiation(msg) => &msg.session,
			Self::InitializeShareChangeSession(msg) => &msg.session,
			Self::ConfirmShareChangeSessionInitialization(msg) => &msg.session,
			Self::ServersSetChangeDelegate(msg) => &msg.session,
			Self::ServersSetChangeDelegateResponse(msg) => &msg.session,
			Self::ServersSetChangeShareAddMessage(msg) => &msg.session,
			Self::ServersSetChangeError(msg) => &msg.session,
			Self::ServersSetChangeCompleted(msg) => &msg.session,
		}
	}

	pub fn session_nonce(&self) -> u64 {
		match self {
			Self::ServersSetChangeConsensusMessage(msg) => msg.session_nonce,
			Self::UnknownSessionsRequest(msg) => msg.session_nonce,
			Self::UnknownSessions(msg) => msg.session_nonce,
			Self::ShareChangeKeyVersionNegotiation(msg) => msg.session_nonce,
			Self::InitializeShareChangeSession(msg) => msg.session_nonce,
			Self::ConfirmShareChangeSessionInitialization(msg) => msg.session_nonce,
			Self::ServersSetChangeDelegate(msg) => msg.session_nonce,
			Self::ServersSetChangeDelegateResponse(msg) => msg.session_nonce,
			Self::ServersSetChangeShareAddMessage(msg) => msg.session_nonce,
			Self::ServersSetChangeError(msg) => msg.session_nonce,
			Self::ServersSetChangeCompleted(msg) => msg.session_nonce,
		}
	}
}

impl ShareAddMessage {
	pub fn session_id(&self) -> &SessionId {
		match self {
			Self::ShareAddConsensusMessage(msg) => &msg.session,
			Self::KeyShareCommon(msg) => &msg.session,
			Self::NewKeysDissemination(msg) => &msg.session,
			Self::ShareAddError(msg) => &msg.session,
		}
	}

	pub fn session_nonce(&self) -> u64 {
		match self {
			Self::ShareAddConsensusMessage(msg) => msg.session_nonce,
			Self::KeyShareCommon(msg) => msg.session_nonce,
			Self::NewKeysDissemination(msg) => msg.session_nonce,
			Self::ShareAddError(msg) => msg.session_nonce,
		}
	}
}

impl KeyVersionNegotiationMessage {
	pub fn session_id(&self) -> &SessionId {
		match self {
			Self::RequestKeyVersions(msg) => &msg.session,
			Self::KeyVersions(msg) => &msg.session,
			Self::KeyVersionsError(msg) => &msg.session,
		}
	}

	pub fn sub_session_id(&self) -> &Secret {
		match self {
			Self::RequestKeyVersions(msg) => &msg.sub_session,
			Self::KeyVersions(msg) => &msg.sub_session,
			Self::KeyVersionsError(msg) => &msg.sub_session,
		}
	}

	pub fn session_nonce(&self) -> u64 {
		match self {
			Self::RequestKeyVersions(msg) => msg.session_nonce,
			Self::KeyVersions(msg) => msg.session_nonce,
			Self::KeyVersionsError(msg) => msg.session_nonce,
		}
	}
}

impl fmt::Display for Message {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::Cluster(message) => write!(f, "Cluster.{}", message),
			Self::Generation(message) => write!(f, "Generation.{}", message),
			Self::Encryption(message) => write!(f, "Encryption.{}", message),
			Self::Decryption(message) => write!(f, "Decryption.{}", message),
			Self::SchnorrSigning(message) => write!(f, "SchnorrSigning.{}", message),
			Self::EcdsaSigning(message) => write!(f, "EcdsaSigning.{}", message),
			Self::ServersSetChange(message) => write!(f, "ServersSetChange.{}", message),
			Self::ShareAdd(message) => write!(f, "ShareAdd.{}", message),
			Self::KeyVersionNegotiation(message) => write!(f, "KeyVersionNegotiation.{}", message),
		}
	}
}

impl fmt::Display for ClusterMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::NodePublicKey(_) => write!(f, "NodePublicKey"),
			Self::NodePrivateKeySignature(_) => write!(f, "NodePrivateKeySignature"),
			Self::KeepAlive(_) => write!(f, "KeepAlive"),
			Self::KeepAliveResponse(_) => write!(f, "KeepAliveResponse"),
		}
	}
}

impl fmt::Display for GenerationMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::InitializeSession(_) => write!(f, "InitializeSession"),
			Self::ConfirmInitialization(_) => write!(f, "ConfirmInitialization"),
			Self::CompleteInitialization(_) => write!(f, "CompleteInitialization"),
			Self::KeysDissemination(_) => write!(f, "KeysDissemination"),
			Self::PublicKeyShare(_) => write!(f, "PublicKeyShare"),
			Self::SessionError(msg) => write!(f, "SessionError({})", msg.error),
			Self::SessionCompleted(_) => write!(f, "SessionCompleted"),
		}
	}
}

impl fmt::Display for EncryptionMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::InitializeEncryptionSession(_) => write!(f, "InitializeEncryptionSession"),
			Self::ConfirmEncryptionInitialization(_) => write!(f, "ConfirmEncryptionInitialization"),
			Self::EncryptionSessionError(msg) => write!(f, "EncryptionSessionError({})", msg.error),
		}
	}
}

impl fmt::Display for ConsensusMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::InitializeConsensusSession(_) => write!(f, "InitializeConsensusSession"),
			Self::ConfirmConsensusInitialization(msg) => write!(f, "ConfirmConsensusInitialization({})", msg.is_confirmed),
		}
	}
}

impl fmt::Display for ConsensusMessageWithServersSet {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::InitializeConsensusSession(_) => write!(f, "InitializeConsensusSession"),
			Self::ConfirmConsensusInitialization(msg) => write!(f, "ConfirmConsensusInitialization({})", msg.is_confirmed),
		}
	}
}

impl fmt::Display for ConsensusMessageOfShareAdd {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::InitializeConsensusSession(_) => write!(f, "InitializeConsensusSession"),
			Self::ConfirmConsensusInitialization(msg) => write!(f, "ConfirmConsensusInitialization({})", msg.is_confirmed),
		}
	}
}

impl fmt::Display for DecryptionMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::DecryptionConsensusMessage(m) => write!(f, "DecryptionConsensusMessage.{}", m.message),
			Self::RequestPartialDecryption(_) => write!(f, "RequestPartialDecryption"),
			Self::PartialDecryption(_) => write!(f, "PartialDecryption"),
			Self::DecryptionSessionError(_) => write!(f, "DecryptionSessionError"),
			Self::DecryptionSessionCompleted(_) => write!(f, "DecryptionSessionCompleted"),
			Self::DecryptionSessionDelegation(_) => write!(f, "DecryptionSessionDelegation"),
			Self::DecryptionSessionDelegationCompleted(_) => write!(f, "DecryptionSessionDelegationCompleted"),
		}
	}
}

impl fmt::Display for SchnorrSigningMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::SchnorrSigningConsensusMessage(m) => write!(f, "SchnorrSigningConsensusMessage.{}", m.message),
			Self::SchnorrSigningGenerationMessage(m) => write!(f, "SchnorrSigningGenerationMessage.{}", m.message),
			Self::SchnorrRequestPartialSignature(_) => write!(f, "SchnorrRequestPartialSignature"),
			Self::SchnorrPartialSignature(_) => write!(f, "SchnorrPartialSignature"),
			Self::SchnorrSigningSessionError(_) => write!(f, "SchnorrSigningSessionError"),
			Self::SchnorrSigningSessionCompleted(_) => write!(f, "SchnorrSigningSessionCompleted"),
			Self::SchnorrSigningSessionDelegation(_) => write!(f, "SchnorrSigningSessionDelegation"),
			Self::SchnorrSigningSessionDelegationCompleted(_) => write!(f, "SchnorrSigningSessionDelegationCompleted"),
		}
	}
}

impl fmt::Display for EcdsaSigningMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::EcdsaSigningConsensusMessage(m) => write!(f, "EcdsaSigningConsensusMessage.{}", m.message),
			Self::EcdsaSignatureNonceGenerationMessage(m) => write!(f, "EcdsaSignatureNonceGenerationMessage.{}", m.message),
			Self::EcdsaInversionNonceGenerationMessage(m) => write!(f, "EcdsaInversionNonceGenerationMessage.{}", m.message),
			Self::EcdsaInversionZeroGenerationMessage(m) => write!(f, "EcdsaInversionZeroGenerationMessage.{}", m.message),
			Self::EcdsaSigningInversedNonceCoeffShare(_) => write!(f, "EcdsaSigningInversedNonceCoeffShare"),
			Self::EcdsaRequestPartialSignature(_) => write!(f, "EcdsaRequestPartialSignature"),
			Self::EcdsaPartialSignature(_) => write!(f, "EcdsaPartialSignature"),
			Self::EcdsaSigningSessionError(_) => write!(f, "EcdsaSigningSessionError"),
			Self::EcdsaSigningSessionCompleted(_) => write!(f, "EcdsaSigningSessionCompleted"),
			Self::EcdsaSigningSessionDelegation(_) => write!(f, "EcdsaSigningSessionDelegation"),
			Self::EcdsaSigningSessionDelegationCompleted(_) => write!(f, "EcdsaSigningSessionDelegationCompleted"),
		}
	}
}

impl fmt::Display for ServersSetChangeMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::ServersSetChangeConsensusMessage(m) => write!(f, "ServersSetChangeConsensusMessage.{}", m.message),
			Self::UnknownSessionsRequest(_) => write!(f, "UnknownSessionsRequest"),
			Self::UnknownSessions(_) => write!(f, "UnknownSessions"),
			Self::ShareChangeKeyVersionNegotiation(m) => write!(f, "ShareChangeKeyVersionNegotiation.{}", m.message),
			Self::InitializeShareChangeSession(_) => write!(f, "InitializeShareChangeSession"),
			Self::ConfirmShareChangeSessionInitialization(_) => write!(f, "ConfirmShareChangeSessionInitialization"),
			Self::ServersSetChangeDelegate(_) => write!(f, "ServersSetChangeDelegate"),
			Self::ServersSetChangeDelegateResponse(_) => write!(f, "ServersSetChangeDelegateResponse"),
			Self::ServersSetChangeShareAddMessage(m) => write!(f, "ServersSetChangeShareAddMessage.{}", m.message),
			Self::ServersSetChangeError(_) => write!(f, "ServersSetChangeError"),
			Self::ServersSetChangeCompleted(_) => write!(f, "ServersSetChangeCompleted"),
		}
	}
}

impl fmt::Display for ShareAddMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::ShareAddConsensusMessage(m) => write!(f, "ShareAddConsensusMessage.{}", m.message),
			Self::KeyShareCommon(_) => write!(f, "KeyShareCommon"),
			Self::NewKeysDissemination(_) => write!(f, "NewKeysDissemination"),
			Self::ShareAddError(_) => write!(f, "ShareAddError"),

		}
	}
}

impl fmt::Display for KeyVersionNegotiationMessage {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		match self {
			Self::RequestKeyVersions(_) => write!(f, "RequestKeyVersions"),
			Self::KeyVersions(_) => write!(f, "KeyVersions"),
			Self::KeyVersionsError(_) => write!(f, "KeyVersionsError"),
		}
	}
}

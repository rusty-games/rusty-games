use serde::{Deserialize, Serialize};
use crate::{IsHost, SessionId};

/// Enum used by all Client, Mini-server and Signaling server to communicate with each other
/// Two main categories are messages used to setup signaling session
/// and messages used to setup WebRTC connection afterwards
#[derive(Debug, Serialize, Deserialize)]
pub enum SignalMessage {
    /// Either client or server connecting to signaling session
    SessionJoin(SessionId),
    /// Report back to the users that both of them are in session
    SessionReady(SessionId, IsHost),

    /// SDP Offer that gets passed to the other user without modifications
    SdpOffer(SessionId, String),
    /// SDP Answer that gets passed to the other user without modifications
    SdpAnswer(SessionId, String),
    /// Proposed ICE Candidate of one user passed to the other user without modifications
    IceCandidate(SessionId, String),

    /// Generic error containing detailed information about the cause
    Error(SessionId, String),
}

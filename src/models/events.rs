use serde::{Deserialize, Serialize};

use super::api::{ParticipantInfo, Room};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event")]
pub enum WebhookEvent {
	RoomStarted {
		room: Room,
	},
	RoomFinished {
		room: Room,
	},
	ParticipantJoined {
		room: Room,
		participant: ParticipantInfo,
	},
	ParticipantLeft {
		room: Room,
		participant: ParticipantInfo,
	},
}

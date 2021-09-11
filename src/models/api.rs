use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Room {
	pub sid: String,
	pub name: String,
	pub empty_timeout: String,
	pub max_participants: String,
	pub creation_time: String,
	pub turn_password: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantInfo {
	pub sid: String,
	pub identity: String,
	pub state: String,
	pub tracks: Vec<TrackInfo>,
	pub metadata: String,
	pub joined_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrackInfo {
	pub sid: String,
	pub r#type: String,
	pub name: String,
	pub muted: bool,
	pub width: u32,
	pub height: u32,
	pub simulcast: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TrackType {
	Audio,
	Video,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParticipantPermission {
	pub can_subscribe: bool,
	pub can_publish: bool,
}

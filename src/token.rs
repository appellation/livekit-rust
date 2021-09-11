use serde::{ser::SerializeMap, Serialize, Serializer};

#[derive(Debug, Default, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoGrant {
	#[serde(skip_serializing_if = "Option::is_none")]
	room_create: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	room_join: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	room_list: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	room_record: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	room_admin: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	room: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	can_publish: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	can_subscribe: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	can_publish_data: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
	hidden: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct Token {
	pub api_key: String,
	pub api_secret: String,
	pub identity: String,
	pub ttl: u64,
	pub video: Option<VideoGrant>,
	pub metadata: Option<String>,
	pub sha256: Option<String>,
}

impl Serialize for Token {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let mut map = serializer.serialize_map(Some(7))?;

		map.serialize_entry("exp", &self.ttl)?;
		map.serialize_entry("iss", &self.api_key)?;
		map.serialize_entry("sub", &self.identity)?;
		map.serialize_entry("nbf", &0)?;
		map.serialize_entry("video", &self.video)?;
		map.serialize_entry("metadata", &self.metadata)?;
		map.serialize_entry("sha256", &self.sha256)?;

		map.end()
	}
}

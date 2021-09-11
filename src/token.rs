use std::borrow::Cow;

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
pub struct Token<'a> {
	pub api_key: Cow<'a, str>,
	pub api_secret: Cow<'a, str>,
	pub identity: Cow<'a, str>,
	pub ttl: u64,
	pub video: Option<VideoGrant>,
	pub metadata: Option<Cow<'a, str>>,
	pub sha256: Option<Cow<'a, str>>,
}

impl<'a> Serialize for Token<'a> {
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

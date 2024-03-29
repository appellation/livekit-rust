use std::{borrow::Cow, convert::TryFrom};

use chrono::{Duration, Utc};
use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use serde::{ser::SerializeMap, Serialize, Serializer};
use sha2::Sha256;

#[derive(Debug, Default, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct VideoGrant<'a> {
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
	room: Option<Cow<'a, str>>,
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
	pub api_secret: Cow<'a, [u8]>,
	pub identity: Cow<'a, str>,
	pub ttl: Duration,
	pub video: Option<VideoGrant<'a>>,
	pub metadata: Option<Cow<'a, str>>,
	pub sha256: Option<Cow<'a, str>>,
}

impl<'a> TryFrom<&Token<'a>> for String {
	type Error = jwt::Error;

	fn try_from(value: &Token<'a>) -> Result<Self, Self::Error> {
		value.to_jwt()
	}
}

impl<'a> Token<'a> {
	pub fn to_jwt(&self) -> Result<String, jwt::Error> {
		let key = Hmac::<Sha256>::new_from_slice(&self.api_secret)?;
		self.sign_with_key(&key)
	}
}

impl<'a> Serialize for Token<'a> {
	fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
	where
		S: Serializer,
	{
		let exp = Utc::now() + self.ttl;
		let mut map = serializer.serialize_map(Some(6))?;

		map.serialize_entry("exp", &exp.timestamp())?;
		map.serialize_entry("iss", &self.api_key)?;
		map.serialize_entry("sub", &self.identity)?;
		map.serialize_entry("video", &self.video)?;
		map.serialize_entry("metadata", &self.metadata)?;
		map.serialize_entry("sha256", &self.sha256)?;

		map.end()
	}
}

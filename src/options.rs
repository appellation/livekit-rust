use std::borrow::Cow;

use serde::Serialize;

use crate::models::api::ParticipantPermission;

#[derive(Debug, Serialize)]
pub struct CreateRoomOptions<'a> {
	pub name: Cow<'a, str>,
	pub empty_timeout: u32,
	pub max_participants: u32,
}

#[derive(Debug, Serialize)]
pub struct ParticipantOptions<'a> {
	pub room: Cow<'a, str>,
	pub identity: Cow<'a, str>,
}

#[derive(Debug, Serialize)]
pub struct MuteTrackOptions<'a> {
	pub room: Cow<'a, str>,
	pub identity: Cow<'a, str>,
	pub track_sid: Cow<'a, str>,
	pub muted: bool,
}

#[derive(Debug, Serialize)]
pub struct UpdateParticipantOptions<'a> {
	pub room: Cow<'a, str>,
	pub identity: Cow<'a, str>,
	pub metadata: Option<Cow<'a, str>>,
	pub permission: Option<ParticipantPermission>,
}

#[derive(Debug, Serialize)]
pub struct UpdateSubscriptionsOptions<'a> {
	pub room: Cow<'a, str>,
	pub identity: Cow<'a, str>,
	pub track_sids: &'a [Cow<'a, str>],
	pub subscribe: bool,
}

#[derive(Debug, Serialize)]
pub(crate) struct RoomName<'a> {
	pub room: Cow<'a, str>,
}

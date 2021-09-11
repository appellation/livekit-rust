use std::borrow::Cow;

use reqwest::{Error, IntoUrl, RequestBuilder, Url};
use serde::{de::DeserializeOwned, Serialize};

use crate::{
	models::api::{ParticipantInfo, Room},
	options::{
		CreateRoomOptions, MuteTrackOptions, ParticipantOptions, RoomName,
		UpdateParticipantOptions, UpdateSubscriptionsOptions,
	},
};

#[derive(Debug, Clone)]
pub struct Client<'token> {
	/// The base URL used to make requests.
	pub base: Url,
	/// The reqwest client used to make requests.
	pub client: reqwest::Client,
	/// The authorization token used to make requests.
	pub token: &'token str,
}

impl<'token> Client<'token> {
	/// Makes a new client. Base URL is appended with the appropriate path segments, so most uses
	/// should provide only the protocol and hostname. Panics if base URL is cannot-be-a-base.
	pub fn new(
		base: impl IntoUrl,
		token: &'token str,
		client: reqwest::Client,
	) -> Result<Self, reqwest::Error> {
		let mut base = base.into_url()?;
		base.path_segments_mut()
			.unwrap()
			.push("twirp")
			.push("livekit.RoomService");

		Ok(Self {
			base,
			token,
			client,
		})
	}

	/// Create a new room.
	pub async fn create_room(&self, options: &CreateRoomOptions<'_>) -> Result<Room, Error> {
		self.send_with_body("CreateRoom", options).await
	}

	/// List rooms.
	pub async fn list_rooms(&self) -> Result<Vec<Room>, Error> {
		self.send("ListRooms").await
	}

	/// Delete a room.
	pub async fn delete_room(&self, room: &str) -> Result<(), Error> {
		self.send_with_body(
			"DeleteRoom",
			&RoomName {
				room: Cow::Borrowed(room),
			},
		)
		.await
	}

	/// List participants in a room.
	pub async fn list_participants(&self, room: &str) -> Result<Vec<ParticipantInfo>, Error> {
		self.send_with_body(
			"ListParticipants",
			&RoomName {
				room: Cow::Borrowed(room),
			},
		)
		.await
	}

	/// Get a participant in a room.
	pub async fn get_participant(
		&self,
		options: &ParticipantOptions<'_>,
	) -> Result<ParticipantInfo, Error> {
		self.send_with_body("GetParticipant", options).await
	}

	/// Remove a participant from a room.
	pub async fn remove_participant(&self, options: &ParticipantOptions<'_>) -> Result<(), Error> {
		self.send_with_body("RemoveParticipant", options).await
	}

	/// Mute a specific track.
	pub async fn mute_track(&self, options: &MuteTrackOptions<'_>) -> Result<(), Error> {
		self.send_with_body("MutePublishedTrack", options).await
	}

	/// Update a participant.
	pub async fn update_participant(
		&self,
		options: &UpdateParticipantOptions<'_>,
	) -> Result<(), Error> {
		self.send_with_body("UpdateParticipant", options).await
	}

	/// Update a participant's subscriptions.
	pub async fn update_subscriptions(
		&self,
		options: &UpdateSubscriptionsOptions<'_>,
	) -> Result<(), Error> {
		self.send_with_body("UpdateSubscriptions", options).await
	}

	fn make_url(&self, method: &str) -> Url {
		let mut url = self.base.clone();
		url.path_segments_mut().unwrap().push(method);
		url
	}

	fn request_builder(&self, method: &str) -> RequestBuilder {
		let url = self.make_url(method);

		self.client.post(url).bearer_auth(&self.token)
	}

	async fn send_with_body<B, R>(&self, method: &str, body: &B) -> Result<R, Error>
	where
		B: Serialize,
		R: DeserializeOwned,
	{
		self.request_builder(method)
			.json(body)
			.send()
			.await?
			.json()
			.await
	}

	async fn send<R>(&self, method: &str) -> Result<R, Error>
	where
		R: DeserializeOwned,
	{
		self.request_builder(method).send().await?.json().await
	}
}

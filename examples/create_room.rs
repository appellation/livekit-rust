use std::borrow::Cow;

use livekit::{api::Client, options::CreateRoomOptions, token::Token};

const BASE_URL: &'static str = "https://demo.livekit.io";
const API_KEY: &'static str = "abc";
const API_SECRET: &'static [u8; 12] = b"super secret";
const IDENTITY: &'static str = "hello world";

#[tokio::main]
async fn main() {
	let token = Token {
		api_key: API_KEY.into(),
		api_secret: Cow::Borrowed(API_SECRET),
		identity: IDENTITY.into(),
		ttl: 3600,
		video: None,
		metadata: None,
		sha256: None,
	};

	let client = Client::new(BASE_URL, token, Default::default()).expect("Invalid client");

	client
		.create_room(&CreateRoomOptions {
			name: "foo".into(),
			empty_timeout: 60,
			max_participants: 5,
		})
		.await
		.expect("Failed creating room");
}

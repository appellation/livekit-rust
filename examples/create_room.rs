use hmac::{Hmac, NewMac};
use jwt::SignWithKey;
use livekit::{api::Client, options::CreateRoomOptions, token::Token};
use sha2::Sha256;

const BASE_URL: &'static str = "https://demo.livekit.io";
const API_KEY: &'static str = "abc";
const API_SECRET: &'static str = "def";
const IDENTITY: &'static str = "hello world";

const SECRET_KEY: &'static [u8; 12] = b"super secret";

#[tokio::main]
async fn main() {
	let token = Token {
		api_key: API_KEY.into(),
		api_secret: API_SECRET.into(),
		identity: IDENTITY.into(),
		ttl: 3600,
		video: None,
		metadata: None,
		sha256: None,
	};

	let key = Hmac::<Sha256>::new_from_slice(SECRET_KEY).expect("Invalid signing key");
	let token = token.sign_with_key(&key).expect("signed token");

	let client = Client::new(BASE_URL, &token, Default::default()).expect("Invalid client");

	client
		.create_room(&CreateRoomOptions {
			name: "foo".into(),
			empty_timeout: 60,
			max_participants: 5,
		})
		.await
		.expect("Failed creating room");
}

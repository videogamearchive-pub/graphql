use crate::utils::pipe::PipeExt;
use async_graphql::connection::CursorType;

pub struct Base64JsonCursor<A>(pub A);

const FAILED_DECODE_BASE64: &str = "Failed to decode base64 cursor";
const FAILED_DESERIALIZE_JSON: &str = "Failed to deserialize json cursor";
const FAILED_SERIALIZE_JSON: &str = "Failed to serialize json cursor";

impl<A: serde::Serialize + serde::de::DeserializeOwned> CursorType for Base64JsonCursor<A> {
    type Error = String;

    fn decode_cursor(str: &str) -> Result<Self, Self::Error> {
        base64::decode_config(str, base64::URL_SAFE_NO_PAD)
            .map_err(|_| FAILED_DECODE_BASE64.to_string())
            .and_then(|bytes| {
                serde_json::from_slice(&bytes).map_err(|_| FAILED_DESERIALIZE_JSON.to_string())
            })
            .map(|a| Base64JsonCursor(a))
    }

    fn encode_cursor(&self) -> String {
        serde_json::to_string(&self.0)
            .expect(FAILED_SERIALIZE_JSON)
            .pipe(|json| base64::encode_config(json, base64::URL_SAFE_NO_PAD))
    }
}

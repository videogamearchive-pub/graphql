use crate::utils::pipe::PipeExt;
use async_graphql::connection::CursorType;

pub struct Base64Cursor<A>(pub A);

impl<A: CursorType> CursorType for Base64Cursor<A> {
    type Error = String;

    fn decode_cursor(s: &str) -> Result<Self, Self::Error> {
        base64::decode_config(s, base64::URL_SAFE_NO_PAD)
            .ok()
            .as_ref()
            .and_then(|s| std::str::from_utf8(s).ok())
            .and_then(|c| A::decode_cursor(c).ok())
            .map(|a| Base64Cursor(a))
            .ok_or_else(|| "Unable to parse cursor".to_owned())
    }

    fn encode_cursor(&self) -> String {
        self.0
            .encode_cursor()
            .pipe(|cursor| base64::encode_config(cursor, base64::URL_SAFE_NO_PAD))
    }
}

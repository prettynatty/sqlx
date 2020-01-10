use crate::decode::{Decode, DecodeError};
use crate::encode::Encode;
use crate::postgres::protocol::TypeId;
use crate::postgres::Postgres;
use crate::types::HasSqlType;

impl HasSqlType<bool> for Postgres {
    fn accepts() -> &'static [Self::TypeId] {
        &[TypeId::BOOL]
    }
}

impl Encode<Postgres> for bool {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.push(*self as u8);
    }
}

impl Decode<Postgres> for bool {
    fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        buf.get(0).map(|&b| b != 0).ok_or_else(|| {
            DecodeError::Message(Box::new("Expected minimum 1 byte but received none."))
        })
    }
}

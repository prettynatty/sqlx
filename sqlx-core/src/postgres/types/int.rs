use crate::decode::{Decode, DecodeError};
use crate::encode::Encode;
use crate::postgres::protocol::TypeId;
use crate::postgres::Postgres;
use crate::types::HasSqlType;
use byteorder::{ByteOrder, NetworkEndian};

impl HasSqlType<i16> for Postgres {
    fn accepts() -> &'static [Self::TypeId] {
        &[TypeId::INT2]
    }
}

impl Encode<Postgres> for i16 {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}

impl Decode<Postgres> for i16 {
    fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(NetworkEndian::read_i16(buf))
    }
}

impl HasSqlType<i32> for Postgres {
    fn accepts() -> &'static [Self::TypeId] {
        &[TypeId::INT4]
    }
}

impl Encode<Postgres> for i32 {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}

impl Decode<Postgres> for i32 {
    fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(NetworkEndian::read_i32(buf))
    }
}

impl HasSqlType<i64> for Postgres {
    fn accepts() -> &'static [Self::TypeId] {
        &[TypeId::INT8]
    }
}

impl Encode<Postgres> for i64 {
    fn encode(&self, buf: &mut Vec<u8>) {
        buf.extend_from_slice(&self.to_be_bytes());
    }
}

impl Decode<Postgres> for i64 {
    fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        Ok(NetworkEndian::read_i64(buf))
    }
}

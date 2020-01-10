use crate::io::Buf;
use crate::postgres::protocol::{Decode, TypeFormat, TypeId};
use byteorder::NetworkEndian;
use std::{io, io::BufRead};

#[derive(Debug)]
pub struct RowDescription {
    pub fields: Box<[Field]>,
}

#[derive(Debug)]
pub struct Field {
    pub name: Option<Box<str>>,
    pub table_id: Option<u32>,
    pub column_id: i16,
    pub type_id: TypeId,
    pub type_size: i16,
    pub type_mod: i32,
    pub type_format: TypeFormat,
}

impl Decode for RowDescription {
    fn decode(mut buf: &[u8]) -> crate::Result<Self> {
        let cnt = buf.get_u16::<NetworkEndian>()? as usize;
        let mut fields = Vec::with_capacity(cnt);

        for _ in 0..cnt {
            let name = buf.get_str_nul()?;
            let name = if name == "?column?" {
                None
            } else {
                Some(name.to_owned().into_boxed_str())
            };

            let table_id = buf.get_u32::<NetworkEndian>()?;

            fields.push(Field {
                name,

                table_id: if table_id > 0 { Some(table_id) } else { None },

                column_id: buf.get_i16::<NetworkEndian>()?,
                type_id: TypeId(buf.get_u32::<NetworkEndian>()?),
                type_size: buf.get_i16::<NetworkEndian>()?,
                type_mod: buf.get_i32::<NetworkEndian>()?,
                type_format: buf.get_i16::<NetworkEndian>()?.into(),
            });
        }

        Ok(Self {
            fields: fields.into_boxed_slice(),
        })
    }
}

#[cfg(test)]
mod test {
    use super::{Decode, RowDescription};

    #[test]
    fn it_decodes_row_description() {
        #[rustfmt::skip]
        let buf = bytes! {
            // Number of Parameters
            0_u8, 2_u8,

            // 1
            b"user_id\0", // name
            0_u8, 0_u8, 0_u8, 0_u8, // table_id
            0_u8, 0_u8, // attr_num
            0_u8, 0_u8, 0_u8, 0_u8, // type_id
            0_u8, 0_u8, // type_size
            0_u8, 0_u8, 0_u8, 0_u8, // type_mod
            0_u8, 0_u8, // format_code

            // 2
            b"number_of_pages\0", // name
            0_u8, 0_u8, 0_u8, 0_u8, // table_id
            0_u8, 0_u8, // attr_num
            0_u8, 0_u8, 5_u8, 0_u8, // type_id
            0_u8, 0_u8, // type_size
            0_u8, 0_u8, 0_u8, 0_u8, // type_mod
            0_u8, 0_u8 // format_code
        };

        let desc = RowDescription::decode(&buf).unwrap();

        assert_eq!(desc.fields.len(), 2);
        assert_eq!(desc.fields[0].type_id, 0x0000_0000);
        assert_eq!(desc.fields[1].type_id, 0x0000_0500);
    }

    #[test]
    fn it_decodes_empty_row_description() {
        let buf = b"\x00\x00";
        let desc = RowDescription::decode(buf).unwrap();

        assert_eq!(desc.fields.len(), 0);
    }
}

use std::fs::File;
use std::io::{self, Read};
use std::fmt;

use crate::archive_tools::structs::struc_form::{FormTrait, FormBase};
use crate::archive_tools::types::{FormID, FormType, GlobalID, StrSml};

#[derive(PartialEq, Eq, Clone)]
pub struct FormWorld {
    pub base: FormBase,
    pub world_name: StrSml,
    pub world_parts: Vec<GlobalID>,
}

impl FormWorld {
    pub const BYTE_COUNT_PARTS_COUNT: usize = 1;  // 1 byte for the count of world parts

    pub fn new(form_id: FormID, form_name: StrSml, world_name: StrSml, world_parts: Vec<GlobalID>) -> Self {
        let base = FormBase {
            form_id,
            form_type: FormType::WORLD,
            form_name,
        };
        Self {
            base,
            world_name,
            world_parts,
        }
    }

    pub fn get_byte_count(&self) -> usize {
        self.base.get_byte_count()
            + self.world_name.get_byte_count()
            + Self::BYTE_COUNT_PARTS_COUNT
            + (self.world_parts.len() * GlobalID::BYTE_COUNT)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.base.to_bytes();
        bytes.extend_from_slice(&self.world_name.to_bytes());
        bytes.extend_from_slice(&(self.world_parts.len() as u8).to_be_bytes());
        for part in &self.world_parts {
            bytes.extend_from_slice(&part.to_bytes());
        }
        bytes
    }
}

impl FormWorld {
    pub fn read_from_bytes(file: &mut File) -> std::io::Result<Self> {
        // Read the FormID and FormType
        let mut form_id_buffer = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buffer)?;  // This reads the bytes into form_id_buffer
        let form_id = FormID::from(form_id_buffer);

        // Read the FormType
        let mut form_type_buffer = [0u8; FormType::BYTE_COUNT];
        file.read_exact(&mut form_type_buffer)?;
        let form_type = FormType::from(form_type_buffer[0]);

        // Read the FormName
        let form_name = StrSml::read_from_bytes(file)?;

        // Read the WorldName
        let world_name = StrSml::read_from_bytes(file)?;

        // Read the WorldParts count
        let mut parts_count_buffer = [0u8; 1];
        file.read_exact(&mut parts_count_buffer)?;
        let parts_count = parts_count_buffer[0];

        // Read the WorldParts
        let mut world_parts = Vec::with_capacity(parts_count as usize);
        for _ in 0..parts_count {
            let mut part_data_buffer = [0u8; GlobalID::BYTE_COUNT];
            file.read_exact(&mut part_data_buffer)?;
            let part_data = GlobalID::from(part_data_buffer);
            world_parts.push(part_data);
        }

        Ok(FormWorld {
            base: FormBase {
                form_id,
                form_type,
                form_name,
            },
            world_name,
            world_parts,
        })
    }

    pub fn read_from_byte_buffer(bytes: &[u8]) -> io::Result<(Self, usize)> {
        let mut offset = 0;

        // Read the FormID
        if bytes.len() < offset + FormID::BYTE_COUNT {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for FormID"));
        }
        let form_id_array: [u8; FormID::BYTE_COUNT] = bytes[offset..offset + FormID::BYTE_COUNT].try_into().unwrap();
        let form_id = FormID::from(form_id_array);
        offset += FormID::BYTE_COUNT;

        // Read the FormType
        if bytes.len() < offset + FormType::BYTE_COUNT {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for FormType"));
        }
        let form_type = FormType::from(bytes[offset]);
        offset += FormType::BYTE_COUNT;

        // Read the FormName
        let (form_name, consumed) = StrSml::read_from_byte_buffer(&bytes[offset..])?;
        offset += consumed;

        // Read the WorldName
        let (world_name, consumed) = StrSml::read_from_byte_buffer(&bytes[offset..])?;
        offset += consumed;

        // Read the WorldParts count
        if bytes.len() < offset + 1 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for parts count"));
        }
        let parts_count = bytes[offset] as usize;
        offset += 1;

        // Read the WorldParts
        let mut world_parts = Vec::with_capacity(parts_count);
        for _ in 0..parts_count {
            if bytes.len() < offset + GlobalID::BYTE_COUNT {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for world part"));
            }
    
            // Convert the slice into a fixed-size array [u8; 3] and then into a GlobalID
            let part_array: [u8; GlobalID::BYTE_COUNT] = bytes[offset..offset + GlobalID::BYTE_COUNT].try_into().unwrap();
            let part = GlobalID::from(part_array);
            offset += GlobalID::BYTE_COUNT;
            
            world_parts.push(part);
        }
        

        Ok((
            FormWorld {
                base: FormBase {
                    form_id,
                    form_type,
                    form_name,
                },
                world_name,
                world_parts,
            },
            offset,
        ))
    }
}

impl FormTrait for FormWorld {

    fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes()
    }

    fn get_byte_count(&self) -> usize {
        self.get_byte_count()
    }

    fn form_id(&self) -> FormID {
        self.base.form_id
    }

    fn form_type(&self) -> FormType {
        self.base.form_type
    }

    fn form_name(&self) -> StrSml {
        self.base.form_name.clone()
    }
}

impl fmt::Display for FormWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormWorld {{ form_id: {}, form_type: {}, form_name: {}, world_name: {}, parts_count: {}, world_parts: {:?} }}",
            self.base.form_id.to_string(),
            self.base.form_type.to_string(),
            self.base.form_name.to_string(),
            self.world_name.to_string(),
            self.world_parts.len(),
            self.world_parts,
        )
    }
}

impl fmt::Debug for FormWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormWorld {{ form_id: {}, form_type: {}, form_name: \"{}\", world_name: \"{}\", byte_count: {}, world_parts: {:?} }}",
            self.base.form_id.to_string(),
            self.base.form_type.to_string(),
            self.base.form_name.to_string(),
            self.world_name.to_string(),
            self.get_byte_count(),
            self.world_parts
        )
    }
}

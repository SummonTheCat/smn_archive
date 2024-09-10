use std::fs::File;
use std::io::{self, Read};
use std::fmt;

use crate::core::structs::{forms::*, types::*};

use super::FormTrait;

#[derive(PartialEq, Eq, Clone)]
pub struct FormRefGroup {
    pub base: FormBase,
    pub form_references: Vec<GlobalID>,
}

impl FormRefGroup {
    pub const BYTE_COUNT_REFERENCES_COUNT: usize = 1;  // 1 byte for the count of form references

    pub fn new(form_id: FormID, form_name: StrSml, form_references: Vec<GlobalID>) -> Self {
        let base = FormBase {
            form_id,
            form_type: FormType::REFGROUP,
            form_name,
        };
        Self {
            base,
            form_references,
        }
    }

    pub fn get_byte_count(&self) -> usize {
        self.base.get_byte_count()
            + Self::BYTE_COUNT_REFERENCES_COUNT
            + (self.form_references.len() * FormID::BYTE_COUNT)
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.base.to_bytes();
        bytes.extend_from_slice(&(self.form_references.len() as u8).to_be_bytes());
        for reference in &self.form_references {
            bytes.extend_from_slice(&reference.to_bytes());
        }
        bytes
    }
}

impl FormRefGroup {
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

        // Read the count of form references
        let mut form_references_count_buffer = [0u8; 1];
        file.read_exact(&mut form_references_count_buffer)?;
        let form_references_count = form_references_count_buffer[0] as usize;

        // Read the form references
        let mut form_references = Vec::with_capacity(form_references_count as usize);
        for _ in 0..form_references_count {
            let mut form_reference_buffer = [0u8; GlobalID::BYTE_COUNT];
            file.read_exact(&mut form_reference_buffer)?;
            let form_reference = GlobalID::from(form_reference_buffer);
            form_references.push(form_reference);
        }

        Ok(FormRefGroup {
            base: FormBase {
                form_id,
                form_type,
                form_name,
            },
            form_references,
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

        // Read the count of form references
        if bytes.len() < offset + 1 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for form references count"));
        }
        let form_references_count = bytes[offset] as usize;
        offset += 1;

        // Read the form references
        let mut form_references = Vec::with_capacity(form_references_count);
        for _ in 0..form_references_count {
            if bytes.len() < offset + GlobalID::BYTE_COUNT {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for form reference"));
            }
            
            // Read the GlobalID
            let global_id_array: [u8; GlobalID::BYTE_COUNT] = bytes[offset..offset + GlobalID::BYTE_COUNT].try_into().unwrap();
            let global_id = GlobalID::from(global_id_array);
            form_references.push(global_id);
            offset += GlobalID::BYTE_COUNT;
        }

        Ok((
        FormRefGroup {
            base: FormBase {
                form_id,
                form_type,
                form_name,
            },
            form_references,
        }, 
        offset,
        ))
    }
}

impl FormTrait for FormRefGroup {
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

impl fmt::Display for FormRefGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FormRefGroup {{ form_id: {}, form_type: {}, form_name: {}, form_references: {:?} }}",
            self.base.form_id.to_string(), self.base.form_type.to_string(), self.base.form_name.to_string(), self.form_references)
    }
}

impl fmt::Debug for FormRefGroup {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "FormRefGroup {{ form_id: {:?}, form_type: {:?}, form_name: {:?}, form_references: {:?} }}",
        self.base.form_id, self.base.form_type, self.base.form_name, self.form_references)
    }
}
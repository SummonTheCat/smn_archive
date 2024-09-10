use std::fs::File;
use std::io::{self, Read, Seek};
use std::fmt;

use crate::core::structs::{forms::*, types::*};

pub trait FormTrait: fmt::Display + fmt::Debug {
    #[allow(unused)]
    fn to_bytes(&self) -> Vec<u8>;
    #[allow(unused)]
    fn get_byte_count(&self) -> usize;
    #[allow(unused)]
    fn form_id(&self) -> FormID;
    #[allow(unused)]
    fn form_type(&self) -> FormType;
    #[allow(unused)]
    fn form_name(&self) -> StrSml;
    
}

#[derive(PartialEq, Eq, Clone)]
pub struct FormBase {
    pub form_id: FormID,
    pub form_type: FormType,
    pub form_name: StrSml,
}

impl FormBase {
    pub const BYTE_COUNT_FORM_ID: usize = FormID::BYTE_COUNT;
    pub const BYTE_COUNT_FORM_TYPE: usize = FormType::BYTE_COUNT;


    // Convert FormBase to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = Vec::with_capacity(Self::BYTE_COUNT_FORM_ID + Self::BYTE_COUNT_FORM_TYPE + self.form_name.get_byte_count());
        bytes.extend_from_slice(&self.form_id.to_bytes());
        bytes.push(self.form_type.to_byte());
        bytes.extend_from_slice(&self.form_name.to_bytes());
        bytes
    }

    pub fn get_byte_count(&self) -> usize {
        Self::BYTE_COUNT_FORM_ID + Self::BYTE_COUNT_FORM_TYPE + self.form_name.get_byte_count()
    }

    pub fn read_from_bytes(file: &mut File) -> io::Result<Box<dyn FormTrait>> {
        let checkpoint = file.seek(std::io::SeekFrom::Current(0))?;

        let mut form_id_buffer = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buffer)?;  
        let _form_id = FormID::from(form_id_buffer);

        let mut form_type_buffer = [0u8; FormType::BYTE_COUNT];
        file.read_exact(&mut form_type_buffer)?;
        let form_type = FormType::from(form_type_buffer[0]);

        file.seek(std::io::SeekFrom::Start(checkpoint))?;

        match form_type {
            FormType::STRING => {
                let form_string = FormString::read_from_bytes(file)?;
                Ok(Box::new(form_string))
            }
            FormType::WORLD => {
                let form_world = FormWorld::read_from_bytes(file)?;
                Ok(Box::new(form_world))
            }
            FormType::REFGROUP => {
                let form_refgroup = FormRefGroup::read_from_bytes(file)?;
                Ok(Box::new(form_refgroup))
            }
            
        }
    }

    #[allow(unused)]
    pub fn read_from_byte_buffer(bytes: &[u8]) -> io::Result<(Box<dyn FormTrait>, usize)> {
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

        let byte_value = bytes[offset] as u8;
        let form_type = FormType::from(byte_value);


        offset += FormType::BYTE_COUNT;

        // Depending on the FormType, call the appropriate constructor
        match form_type {
            FormType::STRING => {
                let (form_string, consumed) = FormString::read_from_byte_buffer(bytes)?;
                offset += consumed;
                Ok((Box::new(form_string), offset))
            }
            FormType::WORLD => {
                let (form_world, consumed) = FormWorld::read_from_byte_buffer(bytes)?;
                offset += consumed;
                Ok((Box::new(form_world), offset))
            }
            FormType::REFGROUP => {
                let (form_refgroup, consumed) = FormRefGroup::read_from_byte_buffer(bytes)?;
                offset += consumed;
                Ok((Box::new(form_refgroup), offset))
            }
        }
    }
}

impl FormTrait for FormBase {

    fn to_bytes(&self) -> Vec<u8> {
        self.to_bytes()
    }

    fn get_byte_count(&self) -> usize {
        self.get_byte_count()
    }

    fn form_id(&self) -> FormID {
        self.form_id
    }

    fn form_type(&self) -> FormType {
        self.form_type
    }

    fn form_name(&self) -> StrSml {
        self.form_name.clone()
    }
}

impl fmt::Display for FormBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormBase {{ \nform_id: {}, \nform_type: {}, \nform_name: {} \n}}",
            self.form_id.to_string(),
            self.form_type.to_string(),
            self.form_name.to_string()
        )
    }
}

impl fmt::Debug for FormBase {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormBase {{ \nform_id: {}, \nform_type: {}, \nform_name: {}, \nbyte_count: {} \n}}",
            self.form_id.to_string(),
            self.form_type.to_string(),
            self.form_name,
            self.get_byte_count()
        )
    }
}

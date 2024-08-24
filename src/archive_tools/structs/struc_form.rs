use std::{fmt, fs::File, io::{self, Read, Seek}};
use crate::archive_tools::types::*;
use crate::archive_tools::structs::*;


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

    // Function to read a FormBase from a file and return the appropriate form struct.
    pub fn read_from_bytes(file: &mut File) -> io::Result<Box<dyn FormTrait>> {
        // Save the current file position (checkpoint)
        let checkpoint = file.seek(std::io::SeekFrom::Current(0))?;

        // Read the FormID and FormType
        let mut form_id_buffer = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buffer)?;  // This reads the bytes into form_id_buffer
        let _form_id = FormID::from(form_id_buffer);
        

        // Read the FormType
        let mut form_type_buffer = [0u8; FormType::BYTE_COUNT];
        file.read_exact(&mut form_type_buffer)?;
        let form_type = FormType::from(form_type_buffer[0]);

        // Seek back to the checkpoint
        file.seek(std::io::SeekFrom::Start(checkpoint))?;

        // Based on the form_type, return the appropriate form struct
        match form_type {
            FormType::STRING => {
                let form_string = FormString::read_from_bytes(file)?;
                Ok(Box::new(form_string))
            }
            FormType::WORLD => {
                let form_world = FormWorld::read_from_bytes(file)?;
                Ok(Box::new(form_world))
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

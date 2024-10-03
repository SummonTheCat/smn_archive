use std::io::Read;
use std::fs::File;
use std::{fmt, io};

use serde_json::{json, Value};

use crate::core::structs::{forms::*, types::*};

/// A struct that represents a string form, which contains multiple language
/// entries and associated strings.
#[derive(PartialEq, Eq, Clone)]
pub struct FormString {
    pub base: FormBase,
    pub languages: Vec<LangCode>,
    pub strings: Vec<StrLrg>,
}

#[allow(unused)]
impl FormString {
    /// Byte count for the language count
    pub const BYTE_COUNT_LANG_COUNT: usize = 1; 

    pub fn new(form_id: FormID, form_name: StrSml, languages: Vec<LangCode>, strings: Vec<StrLrg>) -> Self {
        if languages.len() != strings.len() {
            panic!("The number of languages must match the number of strings.");
        }
        let base = FormBase {
            form_id,
            form_type: FormType::STRING,
            form_name,
        };
        Self {
            base,
            languages,
            strings,
        }
    }

    /// Calculates the total byte count needed to serialize the form
    pub fn get_byte_count(&self) -> usize {
        let lang_byte_size = 1 * self.languages.len();
        let string_byte_size: usize = self.strings.iter().map(|s| s.get_byte_count()).sum();
        self.base.get_byte_count() + Self::BYTE_COUNT_LANG_COUNT + lang_byte_size + string_byte_size
    }

    /// Serializes the `FormString` into a byte array
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.base.to_bytes();
        bytes.extend_from_slice(&(self.languages.len() as u8).to_be_bytes());
        for lang in &self.languages {
            bytes.push(lang.to_byte());
        }
        for string in &self.strings {
            bytes.extend_from_slice(&string.to_bytes());
        }
        bytes
    }

    /// Converts the form into a dictionary-like JSON object for serialization
    pub fn to_dict(&self) -> Value {
        json!({
            "form_id": self.base.form_id.to_string(),
            "form_type": self.base.form_type.to_string(),
            "form_name": self.base.form_name.to_string(),
            "languages": self.languages.iter().map(|lang| lang.to_string()).collect::<Vec<_>>(),
            "strings": self.strings.iter().map(|string| string.to_string()).collect::<Vec<_>>()
        })
    }
}

impl FormString {
   
    /// Reads `FormString` from a binary file
    #[allow(unused)]
    pub fn read_from_bytes(file: &mut File) -> io::Result<Self> {
        // Read the FormID
        let mut form_id_buffer = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buffer)?;  
        let form_id = FormID::from(form_id_buffer);

        // Read the FormType
        let mut form_type_buffer = [0u8; FormType::BYTE_COUNT];
        file.read_exact(&mut form_type_buffer)?;
        let form_type = FormType::from(form_type_buffer[0]);

        // Read the FormName
        let form_name = StrSml::read_from_bytes(file)?;
        
        // Read the language count
        let mut lang_count_buffer = [0u8; 1];
        file.read_exact(&mut lang_count_buffer)?;
        let lang_count = lang_count_buffer[0];

        // Read the languages
        let mut languages = Vec::with_capacity(lang_count as usize);
        for _ in 0..lang_count {
            let mut lang_buffer = [0u8; 1];
            file.read_exact(&mut lang_buffer)?;
            languages.push(LangCode::from(lang_buffer[0]));
        }

        // Read the strings
        let mut strings = Vec::with_capacity(lang_count as usize);
        for _ in 0..lang_count {
            let string_data = StrLrg::read_from_bytes(file)?;
            strings.push(string_data);
        }

        Ok(FormString {
            base: FormBase {
                form_id,
                form_type,
                form_name,
            }, 
            languages,
            strings,
        })
    }

    /// Reads `FormString` from a byte buffer
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

        // Read the language count (1 byte)
        if bytes.len() < offset + 1 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for language count"));
        }
        let lang_count = bytes[offset] as usize;
        offset += 1;

        // Read the languages
        let mut languages = Vec::with_capacity(lang_count);
        for _ in 0..lang_count {
            if bytes.len() < offset + 1 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for language"));
            }
            let lang = LangCode::from(bytes[offset]);
            offset += 1;
            languages.push(lang);
        }

        // Read the strings
        let mut strings = Vec::with_capacity(lang_count);
        for _ in 0..lang_count {
            let (string_data, consumed) = StrLrg::read_from_byte_buffer(&bytes[offset..])?;
            offset += consumed;
            strings.push(string_data);
        }

        Ok((
            FormString {
                base: FormBase {
                    form_id,
                    form_type,
                    form_name,
                },
                languages,
                strings,
            },
            offset,
        ))
    }
}

/// Implementation of the `FormTrait` for `FormString`
impl FormTrait for FormString {

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

    fn to_dict(&self) -> Value {
        self.to_dict()
    }
}

/// Display implementation for `FormString`
impl fmt::Display for FormString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strings_repr: Vec<String> = self.languages.iter().zip(self.strings.iter())
            .map(|(lang, string)| format!("Language: {}, String: {}", lang.to_string(), string.to_string()))
            .collect();
        write!(
            f,
            "FormString {{ \nform_id: {}, \nform_type: {}, \nform_name: {}, langs: [{}] \n strings: [{}] \n}}",
            self.base.form_id.to_string(),
            self.base.form_type.to_string(),
            self.base.form_name.to_string(),
            self.languages.iter().map(|l| l.to_string()).collect::<Vec<String>>().join(", "),
            strings_repr.join(", ")
        )
    }
}

/// Debug implementation for `FormString`
impl fmt::Debug for FormString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormString {{ \nform_id: {}, \nform_type: {}, \nform_name: {}, \nbyte_count: {}, \nstrings: {:?} \n}}",
            self.base.form_id.to_string(),
            self.base.form_type.to_string(),
            self.base.form_name.to_string(),
            self.get_byte_count(),
            self.strings
        )
    }
}

use std::io::Read;
use std::fs::File;
use std::{fmt, io};

use crate::archive_tools::structs::{FormBase, FormTrait};
use crate::archive_tools::types::{FormID, FormType, LangCode, StrLrg, StrSml};

#[derive(PartialEq, Eq, Clone)]
pub struct FormString {
    pub base: FormBase,
    pub languages: Vec<LangCode>,
    pub strings: Vec<StrLrg>,
}

impl FormString {
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

    pub fn get_byte_count(&self) -> usize {
        let lang_byte_size = 1 * self.languages.len();
        let string_byte_size: usize = self.strings.iter().map(|s| s.get_byte_count()).sum();
        self.base.get_byte_count() + Self::BYTE_COUNT_LANG_COUNT + lang_byte_size + string_byte_size
    }

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
}

impl FormString {
    pub fn read_from_bytes(file: &mut File) -> io::Result<Self> {
        // Read the FormID and FormType
        let mut form_id_buffer = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buffer)?;  
        let form_id = FormID::from(form_id_buffer);

        // Read the FormType
        let mut form_type_buffer = [0u8; FormType::BYTE_COUNT];
        file.read_exact(&mut form_type_buffer)?;
        let form_type = FormType::from(form_type_buffer[0]);

        // Read the FormName
        let form_name = StrSml::read_from_bytes(file)?;
        
        // Read the string count
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
}

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
}

impl fmt::Display for FormString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let strings_repr: Vec<String> = self.languages.iter().zip(self.strings.iter())
            .map(|(lang, string)| format!("Language: {}, String: {}", lang.to_string(), string.to_string()))
            .collect();
        write!(
            f,
            "FormString {{ \nform_id: {}, \nform_type: {}, \nform_name: {}, \nstrings: [{}] \n}}",
            self.base.form_id.to_string(),
            self.base.form_type.to_string(),
            self.base.form_name.to_string(),
            strings_repr.join(", ")
        )
    }
}

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

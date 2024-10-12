use std::fmt;
use std::io;
use std::io::Read;

use serde_json::json;

use crate::core::structs::{FormBase, FormTrait};
use crate::core::structs::types::{FormID, EntInstance, StrSml, FormType};

/// Represents a world part form, which contains a base and a list of entity instances (EntInstance).
#[derive(PartialEq, Eq, Clone)]
pub struct FormWorldPart {
    pub base: FormBase,          // Base form structure (ID, type, name)
    pub entities: Vec<EntInstance>,    // List of entity instances in this world part
}

#[allow(unused)]
impl FormWorldPart {
    /// Byte count for the number of entities.
    pub const BYTE_COUNT_ENTITIES_COUNT: usize = 2;

    /// Constructor for `FormWorldPart`, initializing with form ID, name, and entity instance list.
    pub fn new(form_id: FormID, form_name: StrSml, entities: Vec<EntInstance>) -> Self {
        let base = FormBase {
            form_id,
            form_type: FormType::WORLDPART,
            form_name,
        };
        Self {
            base,
            entities,
        }
    }

    /// Returns the byte count needed to serialize the form.
    pub fn get_byte_count(&self) -> usize {
        self.base.get_byte_count()
            + Self::BYTE_COUNT_ENTITIES_COUNT
            + (self.entities.len() * EntInstance::BYTE_COUNT)
    }

    /// Serializes `FormWorldPart` to a byte array.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.base.to_bytes();
        bytes.extend_from_slice(&(self.entities.len() as u16).to_be_bytes());
        for entity in &self.entities {
            bytes.extend_from_slice(&entity.to_bytes());
        }
        bytes
    }

    /// Converts the form into a dictionary-like JSON object.
    pub fn to_dict(&self) -> serde_json::Value {
        json!({
            "form_id": self.base.form_id.to_string(),
            "form_type": self.base.form_type.to_string(),
            "form_name": self.base.form_name.to_string(),
            "entities": self.entities.iter().map(|ent_instance| ent_instance.to_string()).collect::<Vec<_>>(),
        })
    }
}

#[allow(unused)]
impl FormWorldPart {
    /// Reads `FormWorldPart` from a binary file
    pub fn read_from_bytes(file: &mut std::fs::File) -> std::io::Result<Self> {
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

        // Read the number of entities (2 bytes)
        let mut entity_count_buffer = [0u8; 2];
        file.read_exact(&mut entity_count_buffer)?;
        let entity_count = u16::from_be_bytes(entity_count_buffer) as usize;

        // Read the entities
        let mut entities = Vec::with_capacity(entity_count);
        for _ in 0..entity_count {
            let entity = EntInstance::read_from_bytes(file)?;
            entities.push(entity);
        }

        Ok(FormWorldPart {
            base: FormBase {
                form_id,
                form_type,
                form_name,
            },
            entities,
        })
    }

    /// Reads `FormWorldPart` from a byte buffer
    pub fn read_from_byte_buffer(bytes: &[u8]) -> io::Result<(Self, usize)> {
        let mut offset = 0;

        // Read FormID
        if bytes.len() < offset + FormID::BYTE_COUNT {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for FormID"));
        }
        let form_id_array: [u8; FormID::BYTE_COUNT] = bytes[offset..offset + FormID::BYTE_COUNT].try_into().unwrap();
        let form_id = FormID::from(form_id_array);
        offset += FormID::BYTE_COUNT;

        // Read FormType
        if bytes.len() < offset + FormType::BYTE_COUNT {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for FormType"));
        }
        let form_type = FormType::from(bytes[offset]);
        offset += FormType::BYTE_COUNT;

        // Read FormName
        let (form_name, consumed) = StrSml::read_from_byte_buffer(&bytes[offset..])?;
        offset += consumed;

        // Read number of entities (2 bytes)
        if bytes.len() < offset + 2 {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for entity count"));
        }
        let entity_count = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]) as usize;
        offset += 2;

        // Read the entities
        let mut entities = Vec::with_capacity(entity_count);
        for _ in 0..entity_count {
            if bytes.len() < offset + EntInstance::BYTE_COUNT {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for entity instance"));
            }
            let (entity, consumed) = EntInstance::from_byte_buffer(&bytes[offset..])?;
            offset += consumed;
            entities.push(entity);
        }

        Ok((
            FormWorldPart {
                base: FormBase {
                    form_id,
                    form_type,
                    form_name,
                },
                entities,
            },
            offset,
        ))
    }
}

/// Implementation of the `FormTrait` for `FormWorldPart`.
impl FormTrait for FormWorldPart {
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

    fn to_dict(&self) -> serde_json::Value {
        self.to_dict()
    }
}

/// Display implementation for `FormWorldPart`.
impl fmt::Display for FormWorldPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormWorldPart {{ \nform_id: {}, \nform_type: {}, \nform_name: {}, \nentities_count: {}, \nentities: {:?} }}",
            self.base.form_id.to_string(),
            self.base.form_type.to_string(),
            self.base.form_name.to_string(),
            self.entities.len(),
            self.entities
        )
    }
}

/// Debug implementation for `FormWorldPart`.
impl fmt::Debug for FormWorldPart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormWorldPart {{ \nform_id: {:?}, \nform_type: {:?}, \nform_name: {:?}, \nentities_count: {}, \nentities: {:?} }}",
            self.base.form_id,
            self.base.form_type,
            self.base.form_name,
            self.entities.len(),
            self.entities
        )
    }
}

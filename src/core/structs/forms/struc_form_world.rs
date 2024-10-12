use std::fs::File;
use std::io::{self, Read};
use std::fmt;

use serde_json::{json, Value};

use crate::core::structs::{forms::*, types::*};

/// A struct that represents a world form
#[derive(PartialEq, Eq, Clone)]
pub struct FormWorld {
    pub base: FormBase,
    pub world_name_id: GlobalID,  // Changed from StrSml to GlobalID
    pub world_map: StrSml,
    pub world_parts: Vec<GlobalID>,
    pub world_part_anchors: Vec<Vec3Int>, // New field for world part anchors
}


#[allow(unused)]
impl FormWorld {
    /// Byte count for the parts count (2 bytes for both parts and anchors).
    pub const BYTE_COUNT_PARTS_COUNT: usize = 2;

    pub fn new(
        form_id: FormID,
        form_name: StrSml,
        world_name_id: GlobalID,  // Updated to GlobalID
        world_map: StrSml,
        world_parts: Vec<GlobalID>,
        world_part_anchor: Vec<Vec3Int>,
    ) -> Self {
        assert_eq!(world_parts.len(), world_part_anchor.len(), "World parts and anchors count must match.");
    
        let base = FormBase {
            form_id,
            form_type: FormType::WORLD,
            form_name,
        };
        Self {
            base,
            world_name_id,  
            world_map,
            world_parts,
            world_part_anchors: world_part_anchor,
        }
    }
    

    /// Returns the total byte count needed to serialize the form
    pub fn get_byte_count(&self) -> usize {
        self.base.get_byte_count()
            + self.world_name_id.get_byte_count()  
            + self.world_map.get_byte_count()
            + Self::BYTE_COUNT_PARTS_COUNT
            + (self.world_parts.len() * GlobalID::BYTE_COUNT)
            + (self.world_part_anchors.len() * Vec3Int::BYTE_COUNT) 
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.base.to_bytes();
        bytes.extend_from_slice(&self.world_name_id.to_bytes());  // Changed from world_name to world_name_id
        bytes.extend_from_slice(&self.world_map.to_bytes());
        
        bytes.extend_from_slice(&(self.world_parts.len() as u16).to_be_bytes());
    
        for part in &self.world_parts {
            bytes.extend_from_slice(&part.to_bytes());
        }
    
        for anchor in &self.world_part_anchors {
            bytes.extend_from_slice(&anchor.to_bytes());
        }
    
        bytes
    }
    

    /// Converts the form into a dictionary-like JSON object for serialization.
    pub fn to_dict(&self) -> Value {
        json!({
            "form_id": self.base.form_id.to_string(),
            "form_type": self.base.form_type.to_string(),
            "form_name": self.base.form_name.to_string(),
            "world_name_id": self.world_name_id.to_string(),  // Changed from world_name to world_name_id
            "world_map": self.world_map.to_string(),
            "world_parts": self.world_parts.iter().map(|part| part.to_string()).collect::<Vec<_>>(),
            "world_part_anchor": self.world_part_anchors.iter().map(|anchor| anchor.to_dict()).collect::<Vec<_>>(), 
        })
    }
    
}

#[allow(unused)]
impl FormWorld {
    /// Reads `FormWorld` from a binary file
    pub fn read_from_bytes(file: &mut File) -> std::io::Result<Self> {
        // Read the FormID
        let mut form_id_buffer = [0u8; FormID::BYTE_COUNT];
        file.read_exact(&mut form_id_buffer)?;
        let form_id = FormID::from(form_id_buffer);
    
        // Read the FormType
        let mut form_type_buffer = [0u8; FormType::BYTE_COUNT];
        file.read_exact(&mut form_type_buffer)?;
        let form_type = FormType::from(form_type_buffer[0]);
    
        // Read the FormName, WorldName (now GlobalID), and WorldMap
        let form_name = StrSml::read_from_bytes(file)?;
    
        let mut world_name_id_buffer = [0u8; GlobalID::BYTE_COUNT];
        file.read_exact(&mut world_name_id_buffer)?;
        let world_name_id = GlobalID::from(world_name_id_buffer);  // Now reading GlobalID
    
        let world_map = StrSml::read_from_bytes(file)?;
    
        // Read WorldParts count (2 bytes)
        let mut parts_count_buffer = [0u8; 2];
        file.read_exact(&mut parts_count_buffer)?;
        let parts_count = u16::from_be_bytes(parts_count_buffer) as usize;
    
        // Read WorldParts
        let mut world_parts = Vec::with_capacity(parts_count);
        for _ in 0..parts_count {
            let mut part_data_buffer = [0u8; GlobalID::BYTE_COUNT];
            file.read_exact(&mut part_data_buffer)?;
            let part_data = GlobalID::from(part_data_buffer);
            world_parts.push(part_data);
        }

        // Read WorldPartAnchors
        let mut world_part_anchor = Vec::with_capacity(parts_count);
        for _ in 0..parts_count {
            let mut anchor_data_buffer = [0u8; Vec3Int::BYTE_COUNT];
            file.read_exact(&mut anchor_data_buffer)?;
            let anchor_data = Vec3Int::from(anchor_data_buffer);
            world_part_anchor.push(anchor_data);
        }
    
        // Return the FormWorld instance
        Ok(FormWorld {
            base: FormBase {
                form_id,
                form_type,
                form_name,
            },
            world_name_id,
            world_map,
            world_parts,
            world_part_anchors: world_part_anchor, // Include anchors
        })
    }
/// Reads `FormWorld` from a byte buffer
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

    // Read WorldName as GlobalID
    if bytes.len() < offset + GlobalID::BYTE_COUNT {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for GlobalID"));
    }
    let world_name_id_array: [u8; GlobalID::BYTE_COUNT] = bytes[offset..offset + GlobalID::BYTE_COUNT].try_into().unwrap();
    let world_name_id = GlobalID::from(world_name_id_array);  // Correct GlobalID extraction
    offset += GlobalID::BYTE_COUNT;

    // Read WorldMap
    let (world_map, consumed) = StrSml::read_from_byte_buffer(&bytes[offset..])?;
    offset += consumed;

    // Read WorldParts count (2 bytes)
    if bytes.len() < offset + 2 {
        return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for parts count"));
    }
    let parts_count = u16::from_be_bytes([bytes[offset], bytes[offset + 1]]) as usize;
    offset += 2;

    // Read WorldParts
    let mut world_parts = Vec::with_capacity(parts_count);
    for _ in 0..parts_count {
        if bytes.len() < offset + GlobalID::BYTE_COUNT {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for world part"));
        }

        let part_array: [u8; GlobalID::BYTE_COUNT] = bytes[offset..offset + GlobalID::BYTE_COUNT].try_into().unwrap();
        let part = GlobalID::from(part_array);
        offset += GlobalID::BYTE_COUNT;

        world_parts.push(part);
    }

    // Read WorldPartAnchors
    let mut world_part_anchor = Vec::with_capacity(parts_count);
    for _ in 0..parts_count {
        if bytes.len() < offset + Vec3Int::BYTE_COUNT {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for anchor part"));
        }

        let anchor_array: [u8; Vec3Int::BYTE_COUNT] = bytes[offset..offset + Vec3Int::BYTE_COUNT].try_into().unwrap();
        let anchor = Vec3Int::from(anchor_array);
        offset += Vec3Int::BYTE_COUNT;

        world_part_anchor.push(anchor);
    }

    // Return the FormWorld instance
    Ok((
        FormWorld {
            base: FormBase {
                form_id,
                form_type,
                form_name,
            },
            world_name_id,
            world_map,
            world_parts,
            world_part_anchors: world_part_anchor, // Include anchors
        },
        offset,
    ))
}

}

/// Implementation of the `FormTrait` for `FormWorld`
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

    fn to_dict(&self) -> Value {
        self.to_dict()
    }
}

/// Display implementation for `FormWorld`
impl fmt::Display for FormWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormWorld {{ \nform_id: {}, \nform_type: {}, \nform_name: {}, \nworld_name: {}, \nworld_map: {}, \nworld_parts_count: {}, \nworld_parts: {:?}, \nworld_part_anchors: {:?} \n}}",
            self.base.form_id.to_string(),
            self.base.form_type.to_string(),
            self.base.form_name.to_string(),
            self.world_name_id.to_string(),
            self.world_map.to_string(),
            self.world_parts.len(),
            self.world_parts,
            self.world_part_anchors,
        )
    }
}

/// Debug implementation for `FormWorld`
impl fmt::Debug for FormWorld {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormWorld {{ \nform_id: {}, \nform_type: {}, \nform_name: {}, \nworld_name: {}, \nworld_map: {}, \nworld_parts_count: {}, \nworld_parts: {:?}, \nworld_part_anchors: {:?} \n}}",
            self.base.form_id.to_string(),
            self.base.form_type.to_string(),
            self.base.form_name.to_string(),
            self.world_name_id.to_string(),
            self.world_map.to_string(),
            self.world_parts.len(),
            self.world_parts,
            self.world_part_anchors, 
        )
    }
}

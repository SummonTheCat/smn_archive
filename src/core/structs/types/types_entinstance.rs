use std::fmt;
use std::io;
use std::io::Read;

use serde_json::json;

use crate::core::structs::types::EntID;
use crate::core::structs::types::Vec3Float;

/// Represents an instance of an entity in the game world.
#[derive(Clone, Copy)]
pub struct EntInstance {
    pub entity_id: EntID,       // Entity ID (EntID)
    pub position: Vec3Float,    // Position in 3D space (Vec3Float)
    pub rotation: Vec3Float,    // Rotation in 3D space (Vec3Float)
    pub scale: f32,             // Scale factor (non-negative, f32)
}

#[allow(unused)]
impl EntInstance {
    /// Number of bytes for EntInstance:
    /// - EntID: 5 bytes
    /// - Position (Vec3Float): 12 bytes
    /// - Rotation (Vec3Float): 12 bytes
    /// - Scale (f32): 4 bytes
    /// Total: 33 bytes
    pub const BYTE_COUNT: usize = EntID::BYTE_COUNT + 2 * Vec3Float::BYTE_COUNT + 4;

    /// Converts the EntInstance to a byte array for serialization.
    /// This uses big-endian for the scale value.
    pub fn to_bytes(&self) -> [u8; Self::BYTE_COUNT] {
        let mut bytes = [0u8; Self::BYTE_COUNT];
        let mut offset = 0;

        // Entity ID (EntID) -> 5 bytes
        bytes[offset..offset + EntID::BYTE_COUNT].copy_from_slice(&self.entity_id.to_bytes());
        offset += EntID::BYTE_COUNT;

        // Position (Vec3Float) -> 12 bytes
        bytes[offset..offset + Vec3Float::BYTE_COUNT].copy_from_slice(&self.position.to_bytes());
        offset += Vec3Float::BYTE_COUNT;

        // Rotation (Vec3Float) -> 12 bytes
        bytes[offset..offset + Vec3Float::BYTE_COUNT].copy_from_slice(&self.rotation.to_bytes());
        offset += Vec3Float::BYTE_COUNT;

        // Scale (f32) -> 4 bytes (use big-endian for scale)
        bytes[offset..offset + 4].copy_from_slice(&self.scale.to_be_bytes());

        bytes
    }

    /// Converts the EntInstance to a human-readable string.
    pub fn to_string(&self) -> String {
        format!(
            "EntInstance {{ entity_id: {}, position: {}, rotation: {}, scale: {} }}",
            self.entity_id.to_string(),
            self.position.to_string(),
            self.rotation.to_string(),
            self.scale,
        )
    }

    /// Converts the EntInstance to a dictionary-like JSON object.
    pub fn to_dict(&self) -> serde_json::Value {
        json!({
            "entity_id": self.entity_id.to_string(),
            "position": self.position.to_dict(),
            "rotation": self.rotation.to_dict(),
            "scale": self.scale,
        })
    }

    /// Creates an EntInstance from a byte array.
    /// This assumes the scale value is stored in big-endian format.
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut offset = 0;

        // Extract EntID (5 bytes)
        let entity_id = EntID::from([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
            bytes[offset + 4],
        ]);
        offset += EntID::BYTE_COUNT;

        // Extract Position (Vec3Float) (12 bytes)
        let position = Vec3Float::from([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
            bytes[offset + 4],
            bytes[offset + 5],
            bytes[offset + 6],
            bytes[offset + 7],
            bytes[offset + 8],
            bytes[offset + 9],
            bytes[offset + 10],
            bytes[offset + 11],
        ]);
        offset += Vec3Float::BYTE_COUNT;

        // Extract Rotation (Vec3Float) (12 bytes)
        let rotation = Vec3Float::from([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
            bytes[offset + 4],
            bytes[offset + 5],
            bytes[offset + 6],
            bytes[offset + 7],
            bytes[offset + 8],
            bytes[offset + 9],
            bytes[offset + 10],
            bytes[offset + 11],
        ]);
        offset += Vec3Float::BYTE_COUNT;

        // Extract Scale (f32) (4 bytes, big-endian)
        let scale = f32::from_be_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);

        Self {
            entity_id,
            position,
            rotation,
            scale,
        }
    }

    /// Creates an EntInstance from a byte buffer, returning the number of bytes read.
    /// This assumes the scale value is stored in big-endian format.
    pub fn from_byte_buffer(bytes: &[u8]) -> io::Result<(Self, usize)> {
        let mut offset = 0;

        // Check if there are enough bytes for EntID, Position, Rotation, and Scale.
        if bytes.len() < Self::BYTE_COUNT {
            return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for EntInstance"));
        }

        // Extract EntID (5 bytes)
        let entity_id = EntID::from([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
            bytes[offset + 4],
        ]);
        offset += EntID::BYTE_COUNT;

        // Extract Position (Vec3Float) (12 bytes)
        let position = Vec3Float::from([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
            bytes[offset + 4],
            bytes[offset + 5],
            bytes[offset + 6],
            bytes[offset + 7],
            bytes[offset + 8],
            bytes[offset + 9],
            bytes[offset + 10],
            bytes[offset + 11],
        ]);
        offset += Vec3Float::BYTE_COUNT;

        // Extract Rotation (Vec3Float) (12 bytes)
        let rotation = Vec3Float::from([
            bytes[offset],
            bytes[offset + 1],
            bytes[offset + 2],
            bytes[offset + 3],
            bytes[offset + 4],
            bytes[offset + 5],
            bytes[offset + 6],
            bytes[offset + 7],
            bytes[offset + 8],
            bytes[offset + 9],
            bytes[offset + 10],
            bytes[offset + 11],
        ]);
        offset += Vec3Float::BYTE_COUNT;

        // Extract Scale (f32) (4 bytes, big-endian)
        let scale = f32::from_be_bytes([bytes[offset], bytes[offset + 1], bytes[offset + 2], bytes[offset + 3]]);
        offset += 4;

        Ok((Self {
            entity_id,
            position,
            rotation,
            scale,
        }, offset))
    }

    /// Reads an EntInstance from a file, assuming the exact byte count for the instance is available.
    pub fn read_from_bytes(file: &mut std::fs::File) -> io::Result<Self> {
        let mut buffer = [0u8; Self::BYTE_COUNT];
        file.read_exact(&mut buffer)?;
        Ok(Self::from_bytes(&buffer))
    }
}

impl From<(EntID, Vec3Float, Vec3Float, f32)> for EntInstance {
    /// Creates an `EntInstance` from a tuple of `EntID`, `Vec3Float` for position, `Vec3Float` for rotation, and `f32` for scale.
    fn from(values: (EntID, Vec3Float, Vec3Float, f32)) -> Self {
        Self {
            entity_id: values.0,
            position: values.1,
            rotation: values.2,
            scale: values.3,
        }
    }
}

impl fmt::Display for EntInstance {
    /// Formats the EntInstance as a string.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EntInstance {{ entity_id: {}, position: {}, rotation: {}, scale: {} }}",
            self.entity_id.to_string(),
            self.position.to_string(),
            self.rotation.to_string(),
            self.scale,
        )
    }
}

impl fmt::Debug for EntInstance {
    /// Formats the EntInstance with additional details for debugging.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "EntInstance {{ \nentity_id: {:?}, \nposition: {:?}, \nrotation: {:?}, \nscale: {} }}",
            self.entity_id,
            self.position,
            self.rotation,
            self.scale
        )
    }
}

impl PartialEq for EntInstance {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id
            && self.position == other.position
            && self.rotation == other.rotation
            && self.scale.to_bits() == other.scale.to_bits()  // Compare f32 using raw bits
    }
}

impl Eq for EntInstance {}

use std::fs::File;
use std::io::{self, Read};
use std::fmt;
use serde_json::{json, Value};
use crate::core::structs::{forms::*, types::*};

/// A struct that represents a weather form
#[derive(Clone)]
pub struct FormWeather {
    pub base: FormBase,

    pub gi_lighting_color: Vec<SmlColor>,     // Array of 4 values (Day, Dusk, Night, Dawn)
    pub gi_lighting_intensity: Vec<f32>,      // Array of 4 values (Day, Dusk, Night, Dawn)
    pub gi_shadow_intensity: Vec<f32>,        // Array of 4 values (Day, Dusk, Night, Dawn)

    pub precipitation_preset: Vec<GlobalID>,  // Array of 4 GlobalID values (Day, Dusk, Night, Dawn)
    pub precipitation_intensity: Vec<f32>,    // Array of 4 values (Day, Dusk, Night, Dawn)

    pub wind_speed: Vec<f32>,                 // Array of 4 values (Day, Dusk, Night, Dawn)
    pub wind_turbulence: Vec<f32>,            // Array of 4 values (Day, Dusk, Night, Dawn)
    pub wind_direction: Vec<Vec3Float>,       // Array of 4 Vec3Float values (Day, Dusk, Night, Dawn)

    pub skybox_texture: Vec<StrSml>,          // Array of 4 StrSml values (Day, Dusk, Night, Dawn)
    pub skybox_cloud_density: Vec<f32>,       // Array of 4 values (Day, Dusk, Night, Dawn)
    pub skybox_sun_color: Vec<SmlColor>,      // Array of 4 SmlColor values (Day, Dusk, Night, Dawn)
    pub skybox_sun_intensity: Vec<f32>,       // Array of 4 values (Day, Dusk, Night, Dawn)

    // Fog properties
    pub fog_density: Vec<f32>,                // Array of 4 values (Day, Dusk, Night, Dawn)
    pub fog_height: Vec<f32>,                 // Array of 4 values (Day, Dusk, Night, Dawn)
    pub fog_scattering: Vec<f32>,             // Array of 4 values (Day, Dusk, Night, Dawn)
    pub fog_color: Vec<SmlColor>,             // Array of 4 SmlColor values (Day, Dusk, Night, Dawn)

    // Sound properties
    pub sound_ambient_profile: Vec<GlobalID>, // Array of 4 GlobalID values (Day, Dusk, Night, Dawn)
    pub sound_env_reverb: Vec<f32>,           // Array of 4 values (Day, Dusk, Night, Dawn)
    pub sound_env_dampening: Vec<f32>,        // Array of 4 values (Day, Dusk, Night, Dawn)
    pub sound_env_echo_delay: Vec<f32>,       // Array of 4 values (Day, Dusk, Night, Dawn)
}

impl FormWeather {
    /// Constructor for `FormWeather`
    pub fn new(
        form_id: FormID,
        form_name: StrSml,
        gi_lighting_color: Vec<SmlColor>,
        gi_lighting_intensity: Vec<f32>,
        gi_shadow_intensity: Vec<f32>,
        precipitation_preset: Vec<GlobalID>,
        precipitation_intensity: Vec<f32>,
        wind_speed: Vec<f32>,
        wind_turbulence: Vec<f32>,
        wind_direction: Vec<Vec3Float>,
        skybox_texture: Vec<StrSml>,
        skybox_cloud_density: Vec<f32>,
        skybox_sun_color: Vec<SmlColor>,
        skybox_sun_intensity: Vec<f32>,
        fog_density: Vec<f32>,
        fog_height: Vec<f32>,
        fog_scattering: Vec<f32>,
        fog_color: Vec<SmlColor>,
        sound_ambient_profile: Vec<GlobalID>,
        sound_env_reverb: Vec<f32>,
        sound_env_dampening: Vec<f32>,
        sound_env_echo_delay: Vec<f32>,
    ) -> Self {
        assert_eq!(gi_lighting_color.len(), 4, "Expected 4 values for gi_lighting_color");
        assert_eq!(gi_lighting_intensity.len(), 4, "Expected 4 values for gi_lighting_intensity");
        assert_eq!(gi_shadow_intensity.len(), 4, "Expected 4 values for gi_shadow_intensity");
        assert_eq!(precipitation_preset.len(), 4, "Expected 4 values for precipitation_preset");
        assert_eq!(precipitation_intensity.len(), 4, "Expected 4 values for precipitation_intensity");
        assert_eq!(wind_speed.len(), 4, "Expected 4 values for wind_speed");
        assert_eq!(wind_turbulence.len(), 4, "Expected 4 values for wind_turbulence");
        assert_eq!(wind_direction.len(), 4, "Expected 4 values for wind_direction");
        assert_eq!(skybox_texture.len(), 4, "Expected 4 values for skybox_texture");
        assert_eq!(skybox_cloud_density.len(), 4, "Expected 4 values for skybox_cloud_density");
        assert_eq!(skybox_sun_color.len(), 4, "Expected 4 values for skybox_sun_color");
        assert_eq!(skybox_sun_intensity.len(), 4, "Expected 4 values for skybox_sun_intensity");
        assert_eq!(fog_density.len(), 4, "Expected 4 values for fog_density");
        assert_eq!(fog_height.len(), 4, "Expected 4 values for fog_height");
        assert_eq!(fog_scattering.len(), 4, "Expected 4 values for fog_scattering");
        assert_eq!(fog_color.len(), 4, "Expected 4 values for fog_color");
        assert_eq!(sound_ambient_profile.len(), 4, "Expected 4 values for sound_ambient_profile");
        assert_eq!(sound_env_reverb.len(), 4, "Expected 4 values for sound_env_reverb");
        assert_eq!(sound_env_dampening.len(), 4, "Expected 4 values for sound_env_dampening");
        assert_eq!(sound_env_echo_delay.len(), 4, "Expected 4 values for sound_env_echo_delay");

        let base = FormBase {
            form_id,
            form_type: FormType::WEATHER,
            form_name,
        };

        Self {
            base,
            gi_lighting_color,
            gi_lighting_intensity,
            gi_shadow_intensity,
            precipitation_preset,
            precipitation_intensity,
            wind_speed,
            wind_turbulence,
            wind_direction,
            skybox_texture,
            skybox_cloud_density,
            skybox_sun_color,
            skybox_sun_intensity,
            fog_density,
            fog_height,
            fog_scattering,
            fog_color,
            sound_ambient_profile,
            sound_env_reverb,
            sound_env_dampening,
            sound_env_echo_delay,
        }
    }

    /// Calculates the byte count needed for serialization.
    pub fn get_byte_count(&self) -> usize {
        self.base.get_byte_count()
            + (SmlColor::BYTE_COUNT * 4)              // gi_lighting_color
            + (std::mem::size_of::<f32>() * 4 * 13)   // f32 arrays (13 arrays of 4 elements)
            + (GlobalID::BYTE_COUNT * 4 * 2)          // precipitation_preset and sound_ambient_profile
            + (Vec3Float::BYTE_COUNT * 4)             // wind_direction
            + self.skybox_texture.iter().map(|s| s.get_byte_count()).sum::<usize>() // skybox_texture
            + (SmlColor::BYTE_COUNT * 4 * 2)          // skybox_sun_color and fog_color
    }

    /// Serializes `FormWeather` to a byte array.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut bytes = self.base.to_bytes();

        // Serialize gi_lighting_color (Vec<SmlColor>)
        for color in &self.gi_lighting_color {
            bytes.extend_from_slice(&color.to_bytes());
        }

        // Serialize gi_lighting_intensity (Vec<f32>)
        for intensity in &self.gi_lighting_intensity {
            bytes.extend_from_slice(&intensity.to_le_bytes());
        }

        // Serialize gi_shadow_intensity (Vec<f32>)
        for shadow_intensity in &self.gi_shadow_intensity {
            bytes.extend_from_slice(&shadow_intensity.to_le_bytes());
        }

        // Serialize precipitation_preset (Vec<GlobalID>)
        for preset in &self.precipitation_preset {
            bytes.extend_from_slice(&preset.to_bytes());
        }

        // Serialize precipitation_intensity (Vec<f32>)
        for intensity in &self.precipitation_intensity {
            bytes.extend_from_slice(&intensity.to_le_bytes());
        }

        // Serialize wind_speed (Vec<f32>)
        for speed in &self.wind_speed {
            bytes.extend_from_slice(&speed.to_le_bytes());
        }

        // Serialize wind_turbulence (Vec<f32>)
        for turbulence in &self.wind_turbulence {
            bytes.extend_from_slice(&turbulence.to_le_bytes());
        }

        // Serialize wind_direction (Vec<Vec3Float>)
        for direction in &self.wind_direction {
            bytes.extend_from_slice(&direction.to_bytes());
        }

        // Serialize skybox_texture (Vec<StrSml>)
        for texture in &self.skybox_texture {
            bytes.extend_from_slice(&texture.to_bytes());
        }

        // Serialize skybox_cloud_density (Vec<f32>)
        for cloud_density in &self.skybox_cloud_density {
            bytes.extend_from_slice(&cloud_density.to_le_bytes());
        }

        // Serialize skybox_sun_color (Vec<SmlColor>)
        for sun_color in &self.skybox_sun_color {
            bytes.extend_from_slice(&sun_color.to_bytes());
        }

        // Serialize skybox_sun_intensity (Vec<f32>)
        for sun_intensity in &self.skybox_sun_intensity {
            bytes.extend_from_slice(&sun_intensity.to_le_bytes());
        }

        // Serialize fog_density (Vec<f32>)
        for density in &self.fog_density {
            bytes.extend_from_slice(&density.to_le_bytes());
        }

        // Serialize fog_height (Vec<f32>)
        for height in &self.fog_height {
            bytes.extend_from_slice(&height.to_le_bytes());
        }

        // Serialize fog_scattering (Vec<f32>)
        for scattering in &self.fog_scattering {
            bytes.extend_from_slice(&scattering.to_le_bytes());
        }

        // Serialize fog_color (Vec<SmlColor>)
        for color in &self.fog_color {
            bytes.extend_from_slice(&color.to_bytes());
        }

        // Serialize sound_ambient_profile (Vec<GlobalID>)
        for profile in &self.sound_ambient_profile {
            bytes.extend_from_slice(&profile.to_bytes());
        }

        // Serialize sound_env_reverb (Vec<f32>)
        for reverb in &self.sound_env_reverb {
            bytes.extend_from_slice(&reverb.to_le_bytes());
        }

        // Serialize sound_env_dampening (Vec<f32>)
        for dampening in &self.sound_env_dampening {
            bytes.extend_from_slice(&dampening.to_le_bytes());
        }

        // Serialize sound_env_echo_delay (Vec<f32>)
        for echo_delay in &self.sound_env_echo_delay {
            bytes.extend_from_slice(&echo_delay.to_le_bytes());
        }

        bytes
    }

    /// Converts the form into a dictionary-like JSON object.
    pub fn to_dict(&self) -> Value {
        json!({
            "form_id": self.base.form_id.to_string(),
            "form_type": self.base.form_type.to_string(),
            "form_name": self.base.form_name.to_string(),
            "gi_lighting_color": self.gi_lighting_color.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
            "gi_lighting_intensity": self.gi_lighting_intensity,
            "gi_shadow_intensity": self.gi_shadow_intensity,
            "precipitation_preset": self.precipitation_preset.iter().map(|p| p.to_string()).collect::<Vec<_>>(),
            "precipitation_intensity": self.precipitation_intensity,
            "wind_speed": self.wind_speed,
            "wind_turbulence": self.wind_turbulence,
            "wind_direction": self.wind_direction.iter().map(|d| d.to_dict()).collect::<Vec<_>>(),
            "skybox_texture": self.skybox_texture.iter().map(|t| t.to_string()).collect::<Vec<_>>(),
            "skybox_cloud_density": self.skybox_cloud_density,
            "skybox_sun_color": self.skybox_sun_color.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
            "skybox_sun_intensity": self.skybox_sun_intensity,
            "fog_density": self.fog_density,
            "fog_height": self.fog_height,
            "fog_scattering": self.fog_scattering,
            "fog_color": self.fog_color.iter().map(|c| c.to_string()).collect::<Vec<_>>(),
            "sound_ambient_profile": self.sound_ambient_profile.iter().map(|p| p.to_string()).collect::<Vec<_>>(),
            "sound_env_reverb": self.sound_env_reverb,
            "sound_env_dampening": self.sound_env_dampening,
            "sound_env_echo_delay": self.sound_env_echo_delay,
        })
    }

    /// Reads `FormWeather` from a binary file.
    pub fn read_from_bytes(file: &mut File) -> std::io::Result<Self> {
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

        // Read gi_lighting_color (4 values)
        let mut gi_lighting_color = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut color_buffer = [0u8; SmlColor::BYTE_COUNT];
            file.read_exact(&mut color_buffer)?;
            gi_lighting_color.push(SmlColor::from(color_buffer));
        }

        // Read gi_lighting_intensity (4 values)
        let mut gi_lighting_intensity = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut intensity_buffer = [0u8; 4];
            file.read_exact(&mut intensity_buffer)?;
            gi_lighting_intensity.push(f32::from_le_bytes(intensity_buffer));
        }

        // Read gi_shadow_intensity (4 values)
        let mut gi_shadow_intensity = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut shadow_intensity_buffer = [0u8; 4];
            file.read_exact(&mut shadow_intensity_buffer)?;
            gi_shadow_intensity.push(f32::from_le_bytes(shadow_intensity_buffer));
        }

        // Read precipitation_preset (4 values)
        let mut precipitation_preset = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut preset_buffer = [0u8; GlobalID::BYTE_COUNT];
            file.read_exact(&mut preset_buffer)?;
            precipitation_preset.push(GlobalID::from(preset_buffer));
        }

        // Read precipitation_intensity (4 values)
        let mut precipitation_intensity = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut intensity_buffer = [0u8; 4];
            file.read_exact(&mut intensity_buffer)?;
            precipitation_intensity.push(f32::from_le_bytes(intensity_buffer));
        }

        // Read wind_speed (4 values)
        let mut wind_speed = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut speed_buffer = [0u8; 4];
            file.read_exact(&mut speed_buffer)?;
            wind_speed.push(f32::from_le_bytes(speed_buffer));
        }

        // Read wind_turbulence (4 values)
        let mut wind_turbulence = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut turbulence_buffer = [0u8; 4];
            file.read_exact(&mut turbulence_buffer)?;
            wind_turbulence.push(f32::from_le_bytes(turbulence_buffer));
        }

        // Read wind_direction (4 values)
        let mut wind_direction = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut direction_buffer = [0u8; Vec3Float::BYTE_COUNT];
            file.read_exact(&mut direction_buffer)?;
            wind_direction.push(Vec3Float::from(direction_buffer));
        }

        // Read skybox_texture (4 values)
        let mut skybox_texture = Vec::with_capacity(4);
        for _ in 0..4 {
            skybox_texture.push(StrSml::read_from_bytes(file)?);
        }

        // Read skybox_cloud_density (4 values)
        let mut skybox_cloud_density = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut cloud_density_buffer = [0u8; 4];
            file.read_exact(&mut cloud_density_buffer)?;
            skybox_cloud_density.push(f32::from_le_bytes(cloud_density_buffer));
        }

        // Read skybox_sun_color (4 values)
        let mut skybox_sun_color = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut color_buffer = [0u8; SmlColor::BYTE_COUNT];
            file.read_exact(&mut color_buffer)?;
            skybox_sun_color.push(SmlColor::from(color_buffer));
        }

        // Read skybox_sun_intensity (4 values)
        let mut skybox_sun_intensity = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut sun_intensity_buffer = [0u8; 4];
            file.read_exact(&mut sun_intensity_buffer)?;
            skybox_sun_intensity.push(f32::from_le_bytes(sun_intensity_buffer));
        }

        // Read fog_density (4 values)
        let mut fog_density = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut density_buffer = [0u8; 4];
            file.read_exact(&mut density_buffer)?;
            fog_density.push(f32::from_le_bytes(density_buffer));
        }

        // Read fog_height (4 values)
        let mut fog_height = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut height_buffer = [0u8; 4];
            file.read_exact(&mut height_buffer)?;
            fog_height.push(f32::from_le_bytes(height_buffer));
        }

        // Read fog_scattering (4 values)
        let mut fog_scattering = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut scattering_buffer = [0u8; 4];
            file.read_exact(&mut scattering_buffer)?;
            fog_scattering.push(f32::from_le_bytes(scattering_buffer));
        }

        // Read fog_color (4 values)
        let mut fog_color = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut color_buffer = [0u8; SmlColor::BYTE_COUNT];
            file.read_exact(&mut color_buffer)?;
            fog_color.push(SmlColor::from(color_buffer));
        }

        // Read sound_ambient_profile (4 values)
        let mut sound_ambient_profile = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut profile_buffer = [0u8; GlobalID::BYTE_COUNT];
            file.read_exact(&mut profile_buffer)?;
            sound_ambient_profile.push(GlobalID::from(profile_buffer));
        }

        // Read sound_env_reverb (4 values)
        let mut sound_env_reverb = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut reverb_buffer = [0u8; 4];
            file.read_exact(&mut reverb_buffer)?;
            sound_env_reverb.push(f32::from_le_bytes(reverb_buffer));
        }

        // Read sound_env_dampening (4 values)
        let mut sound_env_dampening = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut dampening_buffer = [0u8; 4];
            file.read_exact(&mut dampening_buffer)?;
            sound_env_dampening.push(f32::from_le_bytes(dampening_buffer));
        }

        // Read sound_env_echo_delay (4 values)
        let mut sound_env_echo_delay = Vec::with_capacity(4);
        for _ in 0..4 {
            let mut echo_delay_buffer = [0u8; 4];
            file.read_exact(&mut echo_delay_buffer)?;
            sound_env_echo_delay.push(f32::from_le_bytes(echo_delay_buffer));
        }

        // Return the FormWeather instance
        Ok(FormWeather {
            base: FormBase {
                form_id,
                form_type,
                form_name,
            },
            gi_lighting_color,
            gi_lighting_intensity,
            gi_shadow_intensity,
            precipitation_preset,
            precipitation_intensity,
            wind_speed,
            wind_turbulence,
            wind_direction,
            skybox_texture,
            skybox_cloud_density,
            skybox_sun_color,
            skybox_sun_intensity,
            fog_density,
            fog_height,
            fog_scattering,
            fog_color,
            sound_ambient_profile,
            sound_env_reverb,
            sound_env_dampening,
            sound_env_echo_delay,
        })
    }

    /// Reads `FormWeather` from a byte buffer.
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

        // Read gi_lighting_color (4 values)
        let mut gi_lighting_color = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + SmlColor::BYTE_COUNT {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for gi_lighting_color"));
            }
            let color_bytes: [u8; SmlColor::BYTE_COUNT] = bytes[offset..offset + SmlColor::BYTE_COUNT].try_into().unwrap();
            gi_lighting_color.push(SmlColor::from(color_bytes));
            offset += SmlColor::BYTE_COUNT;
        }

        // Read gi_lighting_intensity (4 values)
        let mut gi_lighting_intensity = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for gi_lighting_intensity"));
            }
            let intensity_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
            gi_lighting_intensity.push(f32::from_le_bytes(intensity_bytes));
            offset += 4;
        }

        // Read gi_shadow_intensity (4 values)
        let mut gi_shadow_intensity = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for gi_shadow_intensity"));
            }
            let shadow_intensity_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
            gi_shadow_intensity.push(f32::from_le_bytes(shadow_intensity_bytes));
            offset += 4;
        }

        // Read precipitation_preset (4 values)
        let mut precipitation_preset = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + GlobalID::BYTE_COUNT {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for precipitation_preset"));
            }
            let preset_bytes: [u8; GlobalID::BYTE_COUNT] = bytes[offset..offset + GlobalID::BYTE_COUNT].try_into().unwrap();
            precipitation_preset.push(GlobalID::from(preset_bytes));
            offset += GlobalID::BYTE_COUNT;
        }

        // Read precipitation_intensity (4 values)
        let mut precipitation_intensity = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for precipitation_intensity"));
            }
            let intensity_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
            precipitation_intensity.push(f32::from_le_bytes(intensity_bytes));
            offset += 4;
        }

        // Read wind_speed (4 values)
        let mut wind_speed = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for wind_speed"));
            }
            let speed_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
            wind_speed.push(f32::from_le_bytes(speed_bytes));
            offset += 4;
        }

        // Read wind_turbulence (4 values)
        let mut wind_turbulence = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for wind_turbulence"));
            }
            let turbulence_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
            wind_turbulence.push(f32::from_le_bytes(turbulence_bytes));
            offset += 4;
        }

        // Read wind_direction (4 values)
        let mut wind_direction = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + Vec3Float::BYTE_COUNT {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for wind_direction"));
            }
            let direction_bytes: [u8; Vec3Float::BYTE_COUNT] = bytes[offset..offset + Vec3Float::BYTE_COUNT].try_into().unwrap();
            wind_direction.push(Vec3Float::from(direction_bytes));
            offset += Vec3Float::BYTE_COUNT;
        }

        // Read skybox_texture (4 values)
        let mut skybox_texture = Vec::with_capacity(4);
        for _ in 0..4 {
            let (texture, consumed) = StrSml::read_from_byte_buffer(&bytes[offset..])?;
            skybox_texture.push(texture);
            offset += consumed;
        }

        // Read skybox_cloud_density (4 values)
        let mut skybox_cloud_density = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for skybox_cloud_density"));
            }
            let cloud_density_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
            skybox_cloud_density.push(f32::from_le_bytes(cloud_density_bytes));
            offset += 4;
        }

        // Read skybox_sun_color (4 values)
        let mut skybox_sun_color = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + SmlColor::BYTE_COUNT {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for skybox_sun_color"));
            }
            let color_bytes: [u8; SmlColor::BYTE_COUNT] = bytes[offset..offset + SmlColor::BYTE_COUNT].try_into().unwrap();
            skybox_sun_color.push(SmlColor::from(color_bytes));
            offset += SmlColor::BYTE_COUNT;
        }

        // Read skybox_sun_intensity (4 values)
        let mut skybox_sun_intensity = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for skybox_sun_intensity"));
            }
            let sun_intensity_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
            skybox_sun_intensity.push(f32::from_le_bytes(sun_intensity_bytes));
            offset += 4;
        }

        // Read fog_density (4 values)
        let mut fog_density = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for fog_density"));
            }
            let density_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
            fog_density.push(f32::from_le_bytes(density_bytes));
            offset += 4;
        }

        // Read fog_height (4 values)
        let mut fog_height = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for fog_height"));
            }
            let height_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
            fog_height.push(f32::from_le_bytes(height_bytes));
            offset += 4;
        }

        // Read fog_scattering (4 values)
        let mut fog_scattering = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for fog_scattering"));
            }
            let scattering_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
            fog_scattering.push(f32::from_le_bytes(scattering_bytes));
            offset += 4;
        }

        // Read fog_color (4 values)
        let mut fog_color = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + SmlColor::BYTE_COUNT {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for fog_color"));
            }
            let color_bytes: [u8; SmlColor::BYTE_COUNT] = bytes[offset..offset + SmlColor::BYTE_COUNT].try_into().unwrap();
            fog_color.push(SmlColor::from(color_bytes));
            offset += SmlColor::BYTE_COUNT;
        }

        // Read sound_ambient_profile (4 values)
        let mut sound_ambient_profile = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + GlobalID::BYTE_COUNT {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for sound_ambient_profile"));
            }
            let profile_bytes: [u8; GlobalID::BYTE_COUNT] = bytes[offset..offset + GlobalID::BYTE_COUNT].try_into().unwrap();
            sound_ambient_profile.push(GlobalID::from(profile_bytes));
            offset += GlobalID::BYTE_COUNT;
        }

        // Read sound_env_reverb (4 values)
        let mut sound_env_reverb = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for sound_env_reverb"));
            }
            let reverb_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
            sound_env_reverb.push(f32::from_le_bytes(reverb_bytes));
            offset += 4;
        }

        // Read sound_env_dampening (4 values)
        let mut sound_env_dampening = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for sound_env_dampening"));
            }
            let dampening_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
            sound_env_dampening.push(f32::from_le_bytes(dampening_bytes));
            offset += 4;
        }

        // Read sound_env_echo_delay (4 values)
        let mut sound_env_echo_delay = Vec::with_capacity(4);
        for _ in 0..4 {
            if bytes.len() < offset + 4 {
                return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "Not enough bytes for sound_env_echo_delay"));
            }
            let echo_delay_bytes: [u8; 4] = bytes[offset..offset + 4].try_into().unwrap();
            sound_env_echo_delay.push(f32::from_le_bytes(echo_delay_bytes));
            offset += 4;
        }

        // Return the FormWeather instance and the offset
        Ok((
            FormWeather {
                base: FormBase {
                    form_id,
                    form_type,
                    form_name,
                },
                gi_lighting_color,
                gi_lighting_intensity,
                gi_shadow_intensity,
                precipitation_preset,
                precipitation_intensity,
                wind_speed,
                wind_turbulence,
                wind_direction,
                skybox_texture,
                skybox_cloud_density,
                skybox_sun_color,
                skybox_sun_intensity,
                fog_density,
                fog_height,
                fog_scattering,
                fog_color,
                sound_ambient_profile,
                sound_env_reverb,
                sound_env_dampening,
                sound_env_echo_delay,
            },
            offset,
        ))
    }
}

/// Implementation of the `FormTrait` for `FormWeather`
impl FormTrait for FormWeather {
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

impl PartialEq for FormWeather {
    fn eq(&self, other: &Self) -> bool {
        let tolerance = 0.00001; // Define a small tolerance for f32 comparison

        self.base.form_id == other.base.form_id
            && (0..4).all(|i| (self.gi_lighting_intensity[i] - other.gi_lighting_intensity[i]).abs() < tolerance)
            && (0..4).all(|i| (self.gi_shadow_intensity[i] - other.gi_shadow_intensity[i]).abs() < tolerance)
            && (0..4).all(|i| (self.precipitation_intensity[i] - other.precipitation_intensity[i]).abs() < tolerance)
            && self.precipitation_preset == other.precipitation_preset
            && (0..4).all(|i| (self.wind_speed[i] - other.wind_speed[i]).abs() < tolerance)
            && (0..4).all(|i| (self.wind_turbulence[i] - other.wind_turbulence[i]).abs() < tolerance)
            && (0..4).all(|i| self.wind_direction[i] == other.wind_direction[i]) // Vec3Float equality check
            && (0..4).all(|i| self.skybox_texture[i] == other.skybox_texture[i]) // StrSml equality check
            && (0..4).all(|i| (self.skybox_cloud_density[i] - other.skybox_cloud_density[i]).abs() < tolerance)
            && (0..4).all(|i| self.skybox_sun_color[i] == other.skybox_sun_color[i]) // SmlColor equality check
            && (0..4).all(|i| (self.skybox_sun_intensity[i] - other.skybox_sun_intensity[i]).abs() < tolerance)
            && (0..4).all(|i| (self.fog_density[i] - other.fog_density[i]).abs() < tolerance)
            && (0..4).all(|i| (self.fog_height[i] - other.fog_height[i]).abs() < tolerance)
            && (0..4).all(|i| (self.fog_scattering[i] - other.fog_scattering[i]).abs() < tolerance)
            && (0..4).all(|i| self.fog_color[i] == other.fog_color[i]) // SmlColor equality check
            && self.sound_ambient_profile == other.sound_ambient_profile
            && (0..4).all(|i| (self.sound_env_reverb[i] - other.sound_env_reverb[i]).abs() < tolerance)
            && (0..4).all(|i| (self.sound_env_dampening[i] - other.sound_env_dampening[i]).abs() < tolerance)
            && (0..4).all(|i| (self.sound_env_echo_delay[i] - other.sound_env_echo_delay[i]).abs() < tolerance)
    }
}

impl Eq for FormWeather {}

/// Display implementation for `FormWeather`
impl fmt::Display for FormWeather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormWeather {{\n\
            form_id: {},\n\
            form_type: {},\n\
            form_name: {},\n\
            \ngi_lighting_color: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \ngi_lighting_intensity: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \ngi_shadow_intensity: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nprecipitation_preset: \nDay: {:?}, \nDusk: {:?}, \nNight: {:?}, \nDawn: {:?}\
            \nprecipitation_intensity: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nwind_speed: \nDay: {}, \nDusk: {}, Night: {}, \nDawn: {}\
            \nwind_turbulence: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nwind_direction: \nDay: {:?}, \nDusk: {:?}, \nNight: {:?}, \nDawn: {:?}\
            \nskybox_texture: \nDay: {:?}, \nDusk: {:?}, \nNight: {:?}, \nDawn: {:?}\
            \nskybox_cloud_density: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nskybox_sun_color: \nDay: {:?}, \nDusk: {:?}, \nNight: {:?}, \nDawn: {:?}\
            \nskybox_sun_intensity: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nfog_density: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nfog_height: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nfog_scattering: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nfog_color: \nDay: {:?}, \nDusk: {:?}, \nNight: {:?}, \nDawn: {:?}\
            \nsound_ambient_profile: \nDay: {:?}, \nDusk: {:?}, \nNight: {:?}, \nDawn: {:?}\
            \nsound_env_reverb: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nsound_env_dampening: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nsound_env_echo_delay: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            }}",
            self.base.form_id.to_string(),
            self.base.form_type.to_string(),
            self.base.form_name.to_string(),
            // GI Lighting Color
            self.gi_lighting_color[0], self.gi_lighting_color[1], self.gi_lighting_color[2], self.gi_lighting_color[3],
            // GI Lighting Intensity
            self.gi_lighting_intensity[0], self.gi_lighting_intensity[1], self.gi_lighting_intensity[2], self.gi_lighting_intensity[3],
            // GI Shadow Intensity
            self.gi_shadow_intensity[0], self.gi_shadow_intensity[1], self.gi_shadow_intensity[2], self.gi_shadow_intensity[3],
            // Precipitation Preset
            self.precipitation_preset[0], self.precipitation_preset[1], self.precipitation_preset[2], self.precipitation_preset[3],
            // Precipitation Intensity
            self.precipitation_intensity[0], self.precipitation_intensity[1], self.precipitation_intensity[2], self.precipitation_intensity[3],
            // Wind Speed
            self.wind_speed[0], self.wind_speed[1], self.wind_speed[2], self.wind_speed[3],
            // Wind Turbulence
            self.wind_turbulence[0], self.wind_turbulence[1], self.wind_turbulence[2], self.wind_turbulence[3],
            // Wind Direction
            self.wind_direction[0], self.wind_direction[1], self.wind_direction[2], self.wind_direction[3],
            // Skybox Texture
            self.skybox_texture[0], self.skybox_texture[1], self.skybox_texture[2], self.skybox_texture[3],
            // Skybox Cloud Density
            self.skybox_cloud_density[0], self.skybox_cloud_density[1], self.skybox_cloud_density[2], self.skybox_cloud_density[3],
            // Skybox Sun Color
            self.skybox_sun_color[0], self.skybox_sun_color[1], self.skybox_sun_color[2], self.skybox_sun_color[3],
            // Skybox Sun Intensity
            self.skybox_sun_intensity[0], self.skybox_sun_intensity[1], self.skybox_sun_intensity[2], self.skybox_sun_intensity[3],
            // Fog Density
            self.fog_density[0], self.fog_density[1], self.fog_density[2], self.fog_density[3],
            // Fog Height
            self.fog_height[0], self.fog_height[1], self.fog_height[2], self.fog_height[3],
            // Fog Scattering
            self.fog_scattering[0], self.fog_scattering[1], self.fog_scattering[2], self.fog_scattering[3],
            // Fog Color
            self.fog_color[0], self.fog_color[1], self.fog_color[2], self.fog_color[3],
            // Sound Ambient Profile
            self.sound_ambient_profile[0], self.sound_ambient_profile[1], self.sound_ambient_profile[2], self.sound_ambient_profile[3],
            // Sound Environment Reverb
            self.sound_env_reverb[0], self.sound_env_reverb[1], self.sound_env_reverb[2], self.sound_env_reverb[3],
            // Sound Environment Dampening
            self.sound_env_dampening[0], self.sound_env_dampening[1], self.sound_env_dampening[2], self.sound_env_dampening[3],
            // Sound Environment Echo Delay
            self.sound_env_echo_delay[0], self.sound_env_echo_delay[1], self.sound_env_echo_delay[2], self.sound_env_echo_delay[3],
        )
    }
}

/// Debug implementation for `FormWeather`
impl fmt::Debug for FormWeather {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FormWeather {{\n\
            form_id: {},\n\
            form_type: {},\n\
            form_name: {},\n\
            \ngi_lighting_color: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \ngi_lighting_intensity: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \ngi_shadow_intensity: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nprecipitation_preset: \nDay: {:?}, \nDusk: {:?}, \nNight: {:?}, \nDawn: {:?}\
            \nprecipitation_intensity: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nwind_speed: \nDay: {}, \nDusk: {}, Night: {}, \nDawn: {}\
            \nwind_turbulence: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nwind_direction: \nDay: {:?}, \nDusk: {:?}, \nNight: {:?}, \nDawn: {:?}\
            \nskybox_texture: \nDay: {:?}, \nDusk: {:?}, \nNight: {:?}, \nDawn: {:?}\
            \nskybox_cloud_density: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nskybox_sun_color: \nDay: {:?}, \nDusk: {:?}, \nNight: {:?}, \nDawn: {:?}\
            \nskybox_sun_intensity: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nfog_density: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nfog_height: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nfog_scattering: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nfog_color: \nDay: {:?}, \nDusk: {:?}, \nNight: {:?}, \nDawn: {:?}\
            \nsound_ambient_profile: \nDay: {:?}, \nDusk: {:?}, \nNight: {:?}, \nDawn: {:?}\
            \nsound_env_reverb: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nsound_env_dampening: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            \nsound_env_echo_delay: \nDay: {}, \nDusk: {}, \nNight: {}, \nDawn: {}\
            }}",
            self.base.form_id.to_string(),
            self.base.form_type.to_string(),
            self.base.form_name.to_string(),
            // GI Lighting Color
            self.gi_lighting_color[0], self.gi_lighting_color[1], self.gi_lighting_color[2], self.gi_lighting_color[3],
            // GI Lighting Intensity
            self.gi_lighting_intensity[0], self.gi_lighting_intensity[1], self.gi_lighting_intensity[2], self.gi_lighting_intensity[3],
            // GI Shadow Intensity
            self.gi_shadow_intensity[0], self.gi_shadow_intensity[1], self.gi_shadow_intensity[2], self.gi_shadow_intensity[3],
            // Precipitation Preset
            self.precipitation_preset[0], self.precipitation_preset[1], self.precipitation_preset[2], self.precipitation_preset[3],
            // Precipitation Intensity
            self.precipitation_intensity[0], self.precipitation_intensity[1], self.precipitation_intensity[2], self.precipitation_intensity[3],
            // Wind Speed
            self.wind_speed[0], self.wind_speed[1], self.wind_speed[2], self.wind_speed[3],
            // Wind Turbulence
            self.wind_turbulence[0], self.wind_turbulence[1], self.wind_turbulence[2], self.wind_turbulence[3],
            // Wind Direction
            self.wind_direction[0], self.wind_direction[1], self.wind_direction[2], self.wind_direction[3],
            // Skybox Texture
            self.skybox_texture[0], self.skybox_texture[1], self.skybox_texture[2], self.skybox_texture[3],
            // Skybox Cloud Density
            self.skybox_cloud_density[0], self.skybox_cloud_density[1], self.skybox_cloud_density[2], self.skybox_cloud_density[3],
            // Skybox Sun Color
            self.skybox_sun_color[0], self.skybox_sun_color[1], self.skybox_sun_color[2], self.skybox_sun_color[3],
            // Skybox Sun Intensity
            self.skybox_sun_intensity[0], self.skybox_sun_intensity[1], self.skybox_sun_intensity[2], self.skybox_sun_intensity[3],
            // Fog Density
            self.fog_density[0], self.fog_density[1], self.fog_density[2], self.fog_density[3],
            // Fog Height
            self.fog_height[0], self.fog_height[1], self.fog_height[2], self.fog_height[3],
            // Fog Scattering
            self.fog_scattering[0], self.fog_scattering[1], self.fog_scattering[2], self.fog_scattering[3],
            // Fog Color
            self.fog_color[0], self.fog_color[1], self.fog_color[2], self.fog_color[3],
            // Sound Ambient Profile
            self.sound_ambient_profile[0], self.sound_ambient_profile[1], self.sound_ambient_profile[2], self.sound_ambient_profile[3],
            // Sound Environment Reverb
            self.sound_env_reverb[0], self.sound_env_reverb[1], self.sound_env_reverb[2], self.sound_env_reverb[3],
            // Sound Environment Dampening
            self.sound_env_dampening[0], self.sound_env_dampening[1], self.sound_env_dampening[2], self.sound_env_dampening[3],
            // Sound Environment Echo Delay
            self.sound_env_echo_delay[0], self.sound_env_echo_delay[1], self.sound_env_echo_delay[2], self.sound_env_echo_delay[3],
        )
    }
}

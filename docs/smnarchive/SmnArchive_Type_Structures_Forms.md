# Structures - Forms
Forms are all structured with some shared Base Form fields, then additional fields per form type. For info on the structure of the fields that a form contains, reference [[SmnArchive - Type Structures - Values]]
### **FormBase**

|Data Field|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|---|
|`form_id`|`FormID`|2|Unique identifier for the form, `u16` value.|Stored as `u16` (Big Endian).|`12345`|
|`form_type`|`FormType`|1|Enum representing the form type, `u8` value.|Stored as `u8` (`0 = STRING`, `1 = WORLD`, etc.).|`0` (for `STRING`)|
|`form_name`|`StrSml`|1 + (1 * chars)|Small string (ASCII) representing the form name.|First byte (`u8`) for length, followed by ASCII characters.|`"form_name"`|

#### **Methods:**

- `to_bytes()`: Converts the `FormBase` into a byte array.
- `get_byte_count()`: Returns the total byte count of the `FormBase`.
- `read_from_bytes(file: &mut File)`: Reads the `FormBase` from a file and returns the appropriate form type.
- `read_from_byte_buffer(bytes: &[u8])`: Reads the `FormBase` from a byte buffer and returns the appropriate form type.
- `to_dict()`: Converts the `FormBase` into a JSON dictionary.

#### **Creation (`::from` variants):**

- `FormBase::from(form_id: FormID, form_type: FormType, form_name: StrSml)`: Creates a `FormBase` from the given parameters.

**Description:**  
`FormBase` is the foundational structure for all forms in the archive system. Every form derives from this base, including the essential fields: a unique ID (`form_id`), form type (`form_type`), and the name of the form (`form_name`).

---
### **FormWorld**

|Data Field|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|---|
|`form_id`|`FormID`|2|Unique identifier for the form, `u16` value.|Stored as `u16` (Big Endian).|`12345`|
|`form_type`|`FormType`|1|Enum representing the form type (`WORLD` = 1), `u8` value.|Stored as `u8`.|`1` (for `WORLD`)|
|`form_name`|`StrSml`|1 + (1 * chars)|Small string (ASCII) representing the form name.|First byte (`u8`) for length, followed by ASCII characters.|`"world_form"`|
|`world_name_id`|`GlobalID`|3|Global ID referencing the world name.|3 bytes for `GlobalID`.|`"00112345"`|
|`world_map`|`StrSml`|1 + (1 * chars)|Small string (ASCII) representing the map of the world.|First byte (`u8`) for length, followed by ASCII characters.|`"world_map"`|
|`world_parts_count`|`u16`|2|Number of parts that make up the world.|Stored as `u16`.|`5`|
|`world_parts`|`GlobalID[]`|3 * `world_parts_count`|List of world parts, each represented by `GlobalID`.|3 bytes for each `GlobalID`.|`[001123, 001456, 001789]`|
|`world_part_anchors`|`Vec3Int[]`|12 * `world_parts_count`|List of anchor positions for each world part, as `Vec3Int`.|12 bytes for each `Vec3Int`.|`[(1, 2, 3), ...]`|

#### **Methods:**

- `to_bytes()`: Converts the `FormWorld` into a byte array.
- `get_byte_count()`: Returns the total byte count of the `FormWorld`.
- `read_from_bytes(file: &mut File)`: Reads the `FormWorld` from a file.
- `read_from_byte_buffer(bytes: &[u8])`: Reads the `FormWorld` from a byte buffer.
- `to_dict()`: Converts the `FormWorld` into a JSON dictionary.

#### **Creation (`::from` variants):**

- `FormWorld::from(form_id: FormID, form_name: StrSml, world_name_id: GlobalID, world_map: StrSml, world_parts: Vec<GlobalID>, world_part_anchors: Vec<Vec3Int>)`: Creates a `FormWorld` from the given parameters.

**Description:**  
`FormWorld` represents world data within the system. It contains fields for identifying world names, maps, and a list of parts that make up the world, including anchor positions for each part. This form allows handling complex world structures, broken down into manageable parts.

---
### **FormString**

|Data Field|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|---|
|`form_id`|`FormID`|2|Unique identifier for the form, `u16` value.|Stored as `u16` (Big Endian).|`12345`|
|`form_type`|`FormType`|1|Enum representing the form type (`STRING` = 0), `u8` value.|Stored as `u8`.|`0` (for `STRING`)|
|`form_name`|`StrSml`|1 + (1 * chars)|Small string (ASCII) representing the form name.|First byte (`u8`) for length, followed by ASCII characters.|`"string_form"`|
|`lang_count`|`u8`|1|Number of languages associated with the form.|Stored as `u8`.|`2`|
|`languages`|`LangCode[]`|1 * `lang_count`|List of language codes, each represented as `LangCode`.|1 byte for each `LangCode`.|`[EN, FR]`|
|`strings`|`StrLrg[]`|Sum of each `StrLrg` byte size|List of large strings, one for each language, encoded as `UTF-16`.|Each string has a 2-byte length followed by UTF-16 characters.|`["Hello", "Bonjour"]`|

#### **Methods:**

- `to_bytes()`: Converts the `FormString` into a byte array.
- `get_byte_count()`: Returns the total byte count of the `FormString`.
- `read_from_bytes(file: &mut File)`: Reads the `FormString` from a file.
- `read_from_byte_buffer(bytes: &[u8])`: Reads the `FormString` from a byte buffer.
- `to_dict()`: Converts the `FormString` into a JSON dictionary.

#### **Creation (`::from` variants):**

- `FormString::from(form_id: FormID, form_name: StrSml, languages: Vec<LangCode>, strings: Vec<StrLrg>)`: Creates a `FormString` from the given parameters.

**Description:**  
`FormString` is designed to handle multilingual string data. It supports storing strings in multiple languages, each associated with a language code. This form is particularly useful for representing localized text content, like names or descriptions, in different languages.

---
### **FormRefGroup**

|Data Field|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|---|
|`form_id`|`FormID`|2|Unique identifier for the form, `u16` value.|Stored as `u16` (Big Endian).|`12345`|
|`form_type`|`FormType`|1|Enum representing the form type (`REFGROUP` = 2), `u8` value.|Stored as `u8`.|`2` (for `REFGROUP`)|
|`form_name`|`StrSml`|1 + (1 * chars)|Small string (ASCII) representing the form name.|First byte (`u8`) for length, followed by ASCII characters.|`"refgroup_form"`|
|`references_count`|`u8`|1|Number of references in the group.|Stored as `u8`.|`3`|
|`form_references`|`GlobalID[]`|3 * `references_count`|List of form references, each represented by `GlobalID`.|3 bytes for each `GlobalID`.|`[001123, 001456, 001789]`|

#### **Methods:**

- `to_bytes()`: Converts the `FormRefGroup` into a byte array.
- `get_byte_count()`: Returns the total byte count of the `FormRefGroup`.
- `read_from_bytes(file: &mut File)`: Reads the `FormRefGroup` from a file.
- `read_from_byte_buffer(bytes: &[u8])`: Reads the `FormRefGroup` from a byte buffer.
- `to_dict()`: Converts the `FormRefGroup` into a JSON dictionary.

#### **Creation (`::from` variants):**

- `FormRefGroup::from(form_id: FormID, form_name: StrSml, form_references: Vec<GlobalID>)`: Creates a `FormRefGroup` from the given parameters.

**Description:**  
`FormRefGroup` is used to represent groups of references to other forms. This allows forms to be grouped together for batch operations or other organizational purposes. Each reference is stored as a `GlobalID`, ensuring that groups are flexible and easy to manage.

---

### **FormWorldPart**

| Data Field       | Data Type       | Byte Size             | Description                                                    | Reading Rules                                               | Example               |
| ---------------- | --------------- | --------------------- | -------------------------------------------------------------- | ----------------------------------------------------------- | --------------------- |
| `form_id`        | `FormID`        | 2                     | Unique identifier for the form, `u16` value.                   | Stored as `u16` (Big Endian).                               | `12345`               |
| `form_type`      | `FormType`      | 1                     | Enum representing the form type (`WORLDPART` = 3), `u8` value. | Stored as `u8`.                                             | `3` (for `WORLDPART`) |
| `form_name`      | `StrSml`        | 1 + (1 * chars)       | Small string (ASCII) representing the form name.               | First byte (`u8`) for length, followed by ASCII characters. | `"world_part_form"`   |
| `entities_count` | `u16`           | 2                     | Number of entities in this world part.                         | Stored as `u16`.                                            | `4`                   |
| `entities`       | `EntInstance[]` | 33 * `entities_count` | List of entity instances (`EntInstance`).                      | 33 bytes for each `EntInstance`.                            | `[EntInstance1, ...]` |

#### **Methods:**

- `to_bytes()`: Converts the `FormWorldPart` into a byte array.
- `get_byte_count()`: Returns the total byte count of the `FormWorldPart`.
- `read_from_bytes(file: &mut File)`: Reads the `FormWorldPart` from a file.
- `read_from_byte_buffer(bytes: &[u8])`: Reads the `FormWorldPart` from a byte buffer.
- `to_dict()`: Converts the `FormWorldPart` into a JSON dictionary.

#### **Creation (`::from` variants):**

- `FormWorldPart::from(form_id: FormID, form_name: StrSml, entities: Vec<EntInstance>)`: Creates a `FormWorldPart` from the given parameters.

**Description:**  
`FormWorldPart` handles the data for parts of the game world, specifically related to entities within that part. Each world part contains a list of entity instances (`EntInstance`), allowing for efficient management of areas in the world, including their associated entities.

---

### **FormWeather**

|Data Field|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|---|
|`form_id`|`FormID`|2|Unique identifier for the form, `u16` value.|Stored as `u16` (Big Endian).|`12345`|
|`form_type`|`FormType`|1|Enum representing the form type (`WEATHER` = 4), `u8` value.|Stored as `u8`.|`4` (for `WEATHER`)|
|`form_name`|`StrSml`|1+(1*chars)|Small string (ASCII) representing the form name.|First byte (`u8`) for length, followed by ASCII chars.|`"weather_form"`|
|`gi_lighting_color`|`SmlColor[4]`|4 * `SmlColor` byte size|Array of 4 lighting colors (Day, Dusk, Night, Dawn).|Stored as 4 `SmlColor` values.|`[(255,255,255), ...]`|
|`gi_lighting_intensity`|`f32[4]`|4 * 4|Array of 4 lighting intensity values.|Stored as 4 `f32` values.|`[1.0, 0.8, 0.6, 0.4]`|
|`gi_shadow_intensity`|`f32[4]`|4 * 4|Array of 4 shadow intensity values.|Stored as 4 `f32` values.|`[0.5, 0.3, 0.2, 0.1]`|
|`precipitation_preset`|`GlobalID[4]`|4 * 3|Array of 4 `GlobalID` values for precipitation presets.|Stored as 4 `GlobalID` values.|`["00112345", ...]`|
|`precipitation_intensity`|`f32[4]`|4 * 4|Array of 4 precipitation intensity values.|Stored as 4 `f32` values.|`[0.1, 0.3, 0.5, 0.7]`|
|`wind_speed`|`f32[4]`|4 * 4|Array of 4 wind speed values.|Stored as 4 `f32` values.|`[10.0, 8.0, 6.0, 4.0]`|
|`wind_turbulence`|`f32[4]`|4 * 4|Array of 4 wind turbulence values.|Stored as 4 `f32` values.|`[1.0, 0.8, 0.6, 0.4]`|
|`wind_direction`|`Vec3Float[4]`|4 * 12|Array of 4 wind direction vectors.|Stored as 4 `Vec3Float` values.|`[(1.0, 0.0, 0.0), ...]`|
|`skybox_texture`|`StrSml[4]`|Sum of each `StrSml` byte size|Array of 4 skybox texture names.|Stored as 4 `StrSml` values.|`["day_sky", "night_sky", ...]`|
|`skybox_cloud_density`|`f32[4]`|4 * 4|Array of 4 cloud density values.|Stored as 4 `f32` values.|`[0.5, 0.7, 0.9, 1.0]`|
|`skybox_sun_color`|`SmlColor[4]`|4 * `SmlColor` byte size|Array of 4 sun colors.|Stored as 4 `SmlColor` values.|`[(255,255,100), ...]`|
|`skybox_sun_intensity`|`f32[4]`|4 * 4|Array of 4 sun intensity values.|Stored as 4 `f32` values.|`[1.0, 0.8, 0.6, 0.4]`|
|`fog_density`|`f32[4]`|4 * 4|Array of 4 fog density values.|Stored as 4 `f32` values.|`[0.1, 0.2, 0.3, 0.4]`|
|`fog_height`|`f32[4]`|4 * 4|Array of 4 fog height values.|Stored as 4 `f32` values.|`[100.0, 120.0, 140.0, 160.0]`|
|`fog_scattering`|`f32[4]`|4 * 4|Array of 4 fog scattering values.|Stored as 4 `f32` values.|`[0.5, 0.6, 0.7, 0.8]`|
|`fog_color`|`SmlColor[4]`|4 * `SmlColor` byte size|Array of 4 fog colors.|Stored as 4 `SmlColor` values.|`[(200,200,200), ...]`|
|`sound_ambient_profile`|`GlobalID[4]`|4 * 3|Array of 4 sound ambient profile `GlobalID` values.|Stored as 4 `GlobalID` values.|`["00112345", ...]`|
|`sound_env_reverb`|`f32[4]`|4 * 4|Array of 4 sound environment reverb values.|Stored as 4 `f32` values.|`[0.8, 0.6, 0.4, 0.2]`|
|`sound_env_dampening`|`f32[4]`|4 * 4|Array of 4 sound environment dampening values.|Stored as 4 `f32` values.|`[0.3, 0.5, 0.7, 0.9]`|
|`sound_env_echo_delay`|`f32[4]`|4 * 4|Array of 4 sound environment echo delay values.|Stored as 4 `f32` values.|`[0.1, 0.2, 0.3, 0.4]`|

#### **Methods:**

- `to_bytes()`: Converts the `FormWeather` into a byte array.
- `get_byte_count()`: Returns the total byte count of the `FormWeather`.
- `read_from_bytes(file: &mut File)`: Reads the `FormWeather` from a file.
- `read_from_byte_buffer(bytes: &[u8])`: Reads the `FormWeather` from a byte buffer.
- `to_dict()`: Converts the `FormWeather` into a JSON dictionary.

#### **Creation (`::from` variants):**

- `FormWeather::from(form_id: FormID, form_name: StrSml, gi_lighting_color: Vec<SmlColor>, gi_lighting_intensity: Vec<f32>, ...)`: Creates a `FormWeather` from the given parameters.

**Description:**  
`FormWeather` handles weather-related configurations, such as lighting, precipitation, wind, skybox properties, fog, and ambient sound profiles. Each weather form is divided into day, dusk, night, and dawn cycles, with specific configurations for each time period.


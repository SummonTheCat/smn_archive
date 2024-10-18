# Structures - Value Types
Values in a SmnArchive are strictly defined for performance and standardization in read/write operations. Value types can be modularly added and form the backbone of value control in the system:

### **Overview**:

Below is a list of value types included as a base, more can be added as needed:

|Data Type|Byte Size|Description|Example|
|---|---|---|---|
|`FormID`|2|2-byte identifier for forms.|12345|
|`ArchiveID`|1|1-byte identifier for the archive.|001|
|`GlobalID`|3|Combines `ArchiveID` and `FormID`.|00112345|
|`Version`|2 (1 major, 1 minor)|Major and minor version numbers.|1.0|
|`LangCode`|1|Enum representing language codes (EN, FR, etc.).|EN, 1|
|`FormType`|1|Enum representing form types (STRING, WORLD, etc.).|STRING, 0|
|`StrSml`|1 + chars|Stores ASCII characters, up to 255 characters.|"hello"|
|`StrLrg`|2 + (2 * chars)|Stores digit codes of UTF-16, up to 65,535 characters.|"this is a large string example"|

---

## Type Details and Usage:

### **FormID**

FormID is a 2-byte unique identifier used for forms in the SmnArchive system. It ensures each form is distinctly identifiable.

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|FormID|2|Unique 2-byte identifier for forms.|[u16: form_id] (Big Endian; first byte is high, second byte is low)|`12345`|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<u16>`: Converts a `u16` into a `FormID`.
    - `From<&str>`: Converts a 5-character string into a `FormID`.
    - `From<String>`: Converts a `String` into a `FormID`.
    - `From<[u8; 2]>`: Converts a 2-byte array into a `FormID` using `u16::from_be_bytes()`.
    - `From<&GlobalID>`: Extracts the `form_id` from a `GlobalID`.
- **Methods:**
    
    - `to_bytes()`: Returns the `FormID` as a 2-byte array.
    - `to_u16()`: Converts the `FormID` to its `u16` representation.
    - `to_string()`: Converts the `FormID` to a zero-padded 5-digit string.
    - `get_byte_count()`: Returns the size of the `FormID` (always 2 bytes).

---

### **ArchiveID**

ArchiveID is a 1-byte unique identifier for archives, ensuring that forms from different archives can be distinguished.

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|ArchiveID|1|Unique 1-byte identifier for archives.|[u8: archive_id]|`001`|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<u8>`: Converts a `u8` into an `ArchiveID`.
    - `From<&str>`: Converts a 3-character string into an `ArchiveID`.
    - `From<String>`: Converts a `String` into an `ArchiveID`.
    - `From<[u8; 1]>`: Converts a 1-byte array into an `ArchiveID`.
    - `From<&GlobalID>`: Extracts the `archive_id` from a `GlobalID`.
- **Methods:**
    
    - `to_bytes()`: Returns the `ArchiveID` as a 1-byte array.
    - `to_u8()`: Converts the `ArchiveID` to its `u8` representation.
    - `to_string()`: Converts the `ArchiveID` to a zero-padded 3-digit string.
    - `get_byte_count()`: Returns the size of the `ArchiveID` (always 1 byte).

---

### **GlobalID**

GlobalID combines the ArchiveID and FormID into a 3-byte identifier. It is used to uniquely identify forms across different archives.

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|GlobalID|3|Combines ArchiveID (1 byte) and FormID (2 bytes) for unique identification.|[u8: archive_id] [u16: form_id] (Big Endian)|`00112345`|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<(ArchiveID, FormID)>`: Combines an `ArchiveID` and a `FormID` tuple into a `GlobalID`.
    - `From<&str>`: Converts an 8-character string into a `GlobalID`.
    - `From<String>`: Converts a `String` into a `GlobalID`.
    - `From<[u8; 3]>`: Converts a 3-byte array into a `GlobalID` (1 byte for `ArchiveID`, 2 bytes for `FormID`).
    - `From<(&ArchiveID, &FormID)>`: Combines references to an `ArchiveID` and a `FormID` into a `GlobalID`.
- **Methods:**
    
    - `to_bytes()`: Converts the `GlobalID` into a 3-byte array.
    - `to_string()`: Combines `ArchiveID` and `FormID` into an 8-digit string.
    - `get_byte_count()`: Returns the byte size of `GlobalID` (always 3 bytes).

---
### **EntID**

EntID is a 5-byte unique identifier for entities, combining a GlobalID and a reference FormID. It is used for identifying entity instances.

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|EntID|5|Combines `GlobalID` (3 bytes) and reference `FormID` (2 bytes) for entity identification.|[u8: archive_id] [u16: form_id] [u16: reference_id] (Big Endian)|`0011234500012`|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<(GlobalID, FormID)>`: Combines a `GlobalID` and reference `FormID` into an `EntID`.
    - `From<&str>`: Converts a 13-character string into an `EntID`.
    - `From<String>`: Converts a `String` into an `EntID`.
    - `From<[u8; 5]>`: Converts a 5-byte array into an `EntID` (3 bytes for `GlobalID`, 2 bytes for `FormID`).
- **Methods:**
    
    - `to_bytes()`: Converts the `EntID` into a 5-byte array.
    - `to_string()`: Combines `GlobalID` and `FormID` into a 13-character string.
    - `get_byte_count()`: Returns the byte size of `EntID` (always 5 bytes).

---
### **Version**

Version represents the major and minor version numbers of the archive, stored in 2 bytes.

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|Version|2|Major and minor version numbers of the archive.|[u8: major_version] [u8: minor_version]|`1.0`|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<[u8; 2]>`: Converts a 2-byte array into a `Version`.
    - `From<f32>`: Converts a floating-point number into a `Version`.
    - `From<(u8, u8)>`: Converts a tuple of two `u8` values into a `Version`.
- **Methods:**
    
    - `to_bytes()`: Converts the `Version` into a 2-byte array.
    - `to_string()`: Converts the `Version` into a string in the format "major.minor".
    - `to_f32()`: Converts the `Version` into a floating-point number.
    - `get_byte_count()`: Returns the byte size of `Version` (always 2 bytes).

---

### **LangCode**

LangCode is a 1-byte code representing a language, stored as an enum.

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|LangCode|1|Language code, represented as an enum.|[u8: lang_code]|`EN`, `1`|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<&str>`: Converts a 2-character string into a `LangCode`.
    - `From<u8>`: Converts a `u8` byte into a `LangCode`.
- **Methods:**
    
    - `to_string()`: Converts the `LangCode` enum into its string representation.
    - `to_int()`: Converts the `LangCode` enum into its corresponding `u8` value.
    - `to_byte()`: Converts the `LangCode` enum to its byte representation.

---

### **FormType**

FormType is a 1-byte enum representing different form types (e.g., STRING, WORLD, REFGROUP).

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|FormType|1|Enum representing the form type (e.g., STRING, WORLD, REFGROUP).|[u8: form_type]|`STRING`, `0`|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<u8>`: Converts a byte (`u8`) into a `FormType`.
    - `From<&str>`: Converts a string into a `FormType`.
- **Methods:**
    
    - `to_string()`: Converts the `FormType` enum into its string representation.
    - `to_u8()`: Converts the `FormType` enum into its `u8` value.
    - `to_byte()`: Converts the `FormType` enum into a byte.
    - `get_byte_count()`: Returns the size of `FormType` (always 1 byte).

---
### **StrSml (Small String)**

StrSml is a small string stored in ASCII encoding, limited to 255 characters. It is ideal for small form names or short strings.

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|StrSml|1 + `chars`|ASCII encoded string, up to 255 characters.|[u8: length] [u8*: ascii_chars]|`"hello"`|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<&str>`: Converts a string into a `StrSml`, encoding it as ASCII.
    - `From<String>`: Converts a `String` into a `StrSml`.
- **Methods:**
    
    - `to_bytes()`: Converts the `StrSml` into a byte array. The first byte stores the string length, followed by ASCII characters.
    - `to_string()`: Converts the `StrSml` into a `String` by decoding the ASCII character data.
    - `get_byte_count()`: Returns the total byte size of the `StrSml`, including the length byte and the string data.
    - `read_from_bytes()`: Reads a `StrSml` from a file by reading the length byte and the character data.
    - `read_from_byte_buffer()`: Reads a `StrSml` from a byte buffer, returning both the string and the number of bytes consumed.

---

### **StrLrg (Large String)**

StrLrg is a large string stored in UTF-16 encoding, supporting strings of up to 65,535 characters. It is suitable for longer textual data.

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|StrLrg|2 + (2 * `chars`)|UTF-16 encoded string, max 65,535 characters.|[u16: length] [u16*: utf16_chars]|`"this is a large string example"`|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<&str>`: Converts a string into a `StrLrg`, encoding it as UTF-16.
    - `From<String>`: Converts a `String` into a `StrLrg`.
- **Methods:**
    
    - `to_bytes()`: Converts the `StrLrg` into a byte array. The first 2 bytes store the string length, followed by UTF-16 characters.
    - `to_string()`: Converts the `StrLrg` into a `String` by decoding the UTF-16 character data.
    - `get_byte_count()`: Returns the total byte size of the `StrLrg`, including the length bytes and the string data.
    - `read_from_bytes()`: Reads a `StrLrg` from a file by reading the length bytes and the character data.
    - `read_from_byte_buffer()`: Reads a `StrLrg` from a byte buffer, returning both the string and the number of bytes consumed.

---

### **Vec2Int**

Vec2Int represents a 2D vector with integer components. It's stored as two `i32` values (x and y), and is often used for storing position or grid coordinates.

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|Vec2Int|8|2D vector with integer components (`i32` for x and y).|[i32: x] [i32: y] (Big Endian, 4 bytes each)|`(1, 2)`|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<(i32, i32)>`: Combines two `i32` values into a `Vec2Int`.
    - `From<[u8; 8]>`: Converts an 8-byte array into a `Vec2Int`.
- **Methods:**
    
    - `to_bytes()`: Converts the `Vec2Int` into an 8-byte array.
    - `to_string()`: Converts the `Vec2Int` into a string format `(x, y)`.
    - `get_byte_count()`: Returns the byte size of `Vec2Int` (always 8 bytes).

---

### **Vec3Int**

Vec3Int is a 3D vector with integer components, stored as three `i32` values (x, y, and z). It’s often used to represent rough positions or checks in 3D space.

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|Vec3Int|12|3D vector with integer components (`i32` for x, y, and z).|[i32: x] [i32: y] [i32: z] (Big Endian, 4 bytes each)|`(1, 2, 3)`|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<(i32, i32, i32)>`: Combines three `i32` values into a `Vec3Int`.
    - `From<[u8; 12]>`: Converts a 12-byte array into a `Vec3Int`.
- **Methods:**
    
    - `to_bytes()`: Converts the `Vec3Int` into a 12-byte array.
    - `to_string()`: Converts the `Vec3Int` into a string format `(x, y, z)`.
    - `get_byte_count()`: Returns the byte size of `Vec3Int` (always 12 bytes).

---

### **Vec2Float**

Vec2Float represents a 2D vector with floating-point components, typically used for precise position or scaling values.

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|Vec2Float|8|2D vector with floating-point components (`f32` for x and y).|[f32: x] [f32: y] (Big Endian, 4 bytes each)|`(1.0, 2.0)`|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<(f32, f32)>`: Combines two `f32` values into a `Vec2Float`.
    - `From<[u8; 8]>`: Converts an 8-byte array into a `Vec2Float`.
- **Methods:**
    
    - `to_bytes()`: Converts the `Vec2Float` into an 8-byte array.
    - `to_string()`: Converts the `Vec2Float` into a string format `(x, y)`.
    - `get_byte_count()`: Returns the byte size of `Vec2Float` (always 8 bytes).

---

### **Vec3Float**

Vec3Float is a 3D vector with floating-point components, stored as three `f32` values (x, y, and z). It is widely used in 3D space representation, such as for position, rotation, or scale.

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|Vec3Float|12|3D vector with floating-point components (`f32` for x, y, and z).|[f32: x] [f32: y] [f32: z] (Big Endian, 4 bytes each)|`(1.0, 2.0, 3.0)`|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<(f32, f32, f32)>`: Combines three `f32` values into a `Vec3Float`.
    - `From<[u8; 12]>`: Converts a 12-byte array into a `Vec3Float`.
- **Methods:**
    
    - `to_bytes()`: Converts the `Vec3Float` into a 12-byte array.
    - `to_string()`: Converts the `Vec3Float` into a string format `(x, y, z)`.
    - `get_byte_count()`: Returns the byte size of `Vec3Float` (always 12 bytes).

---

### **EntInstance**

EntInstance represents an entity instance, consisting of an entity ID, position, rotation, and scale. It is used to track entities in the game world, including their spatial data.

|Data Type|Byte Size|Description|Reading Rules|Example|
|---|---|---|---|---|
|EntInstance|33|Represents an entity instance, including position, rotation, and scale.|`[EntID: 5 bytes] [Vec3Float: position 12 bytes] [Vec3Float: rotation 12 bytes] [f32: scale 4 bytes]`|`{entity_id: "0011234500012", position: (1.0, 2.0, 3.0), rotation: (0.0, 90.0, 0.0), scale: 1.0}`|

#### **Component Rules:**

|Component|Byte Size|Reading Rules|Description|
|---|---|---|---|
|`EntID`|5|`[GlobalID: 3 bytes] [FormID: 2 bytes]`|Unique identifier for the entity|
|`Position`|12|`[f32: x 4 bytes] [f32: y 4 bytes] [f32: z 4 bytes]`|3D position vector|
|`Rotation`|12|`[f32: x 4 bytes] [f32: y 4 bytes] [f32: z 4 bytes]`|3D rotation vector|
|`Scale`|4|`[f32: 4 bytes]`|Scale factor of the entity|

#### **Usage:**

- **Creation (From variants):**
    
    - `From<(EntID, Vec3Float, Vec3Float, f32)>`: Combines an `EntID`, `Vec3Float` for position, `Vec3Float` for rotation, and `f32` scale into an `EntInstance`.
    - `From<[u8; 33]>`: Converts a 33-byte array into an `EntInstance`.
- **Methods:**
    
    - `to_bytes()`: Converts the `EntInstance` into a 33-byte array.
    - `to_string()`: Converts the `EntInstance` into a human-readable string format.
    - `to_dict()`: Converts the `EntInstance` into a JSON-like dictionary.
    - `from_bytes()`: Creates an `EntInstance` from a byte array.
    - `read_from_bytes()`: Reads an `EntInstance` from a file or byte buffer.
    - `get_byte_count()`: Returns the byte size of `EntInstance` (always 33 bytes).

---

### **SmlColor** (Small Color)

`SmlColor` represents a color with 8-bit (1-byte) channels for red, green, blue, and alpha. It is efficient for handling color data when high precision is not necessary.

| Data Type | Byte Size | Description                                | Reading Rules                                  | Example                 |
|-----------|-----------|--------------------------------------------|------------------------------------------------|-------------------------|
| SmlColor  | 4         | RGBA color with 8-bit channels (1 byte per channel). | [u8: r] [u8: g] [u8: b] [u8: a]               | `RGBA(255, 128, 64, 255)` |

#### **Usage:**

- **Creation (From variants):**
    - `From<(u8, u8, u8, u8)>`: Combines 4 `u8` values into an `SmlColor`.
    - `From<[u8; 4]>`: Converts a 4-byte array into an `SmlColor`.
  
- **Methods:**
    - `to_bytes()`: Converts the `SmlColor` into a 4-byte array.
    - `to_string()`: Converts the `SmlColor` into a string format `RGBA(r, g, b, a)`.
    - `get_byte_count()`: Returns the byte size of `SmlColor` (always 4 bytes).

---

### **LrgColor** (Large Color)

`LrgColor` represents a color with 16-bit (2-byte) channels for red, green, blue, and alpha. It is used for higher precision color representation.

| Data Type | Byte Size | Description                                | Reading Rules                                  | Example                        |
|-----------|-----------|--------------------------------------------|------------------------------------------------|--------------------------------|
| LrgColor  | 8         | RGBA color with 16-bit channels (2 bytes per channel). | [u16: r] [u16: g] [u16: b] [u16: a] (Big Endian) | `RGBA(65535, 32768, 16384, 65535)` |

#### **Usage:**

- **Creation (From variants):**
    - `From<(u16, u16, u16, u16)>`: Combines 4 `u16` values into an `LrgColor`.
    - `From<[u8; 8]>`: Converts an 8-byte array into an `LrgColor`.

- **Methods:**
    - `to_bytes()`: Converts the `LrgColor` into an 8-byte array.
    - `to_string()`: Converts the `LrgColor` into a string format `RGBA(r, g, b, a)`.
    - `get_byte_count()`: Returns the byte size of `LrgColor` (always 8 bytes).

# Structure - Binary Archive File 
The SmnArchive (smn) file type is a binary file with various blocks for different content. It stores archive data and data for different form formats containing the form data.

## File Layout

The file structure can be broken down into the following **blocks**:
1. `Header`: Contains general archive info.
2. `ByteStart`: Contains the starting bytes for the data and index blocks.
3. `data`: Contains the data for each form in the archive.
4. `index`: Contains the index that points to the data byte start in the data block.

#### Header:

| **Data Name** | **Data Type**           | **Byte Size** | **Data Description**                                                | **Example**          |
| ------------- | ----------------------- | ------------- | ------------------------------------------------------------------- | -------------------- |
| `archive_id`  | `ArchiveID`             | 4             | Unique identifier for the archive.                                  | `12345`              |
| `version`     | `Version`               | 4             | Version of the archive.                                             | `1.0`                |
| `description` | `StrLrg` (large string) | 2 + (2*chars) | Large string containing a description of the archive (size varies). | `"An empty archive"` |
| `form_count`  | `u16`                   | 2             | Number of forms contained within the archive.                       | `5`                  |

#### ByteStart:

| **Data Name**     | **Data Type** | **Byte Size** | **Data Description**                                      | **Example** |
| ----------------- | ------------- | ------------- | --------------------------------------------------------- | ----------- |
| `bytestart_index` | `u32`         | 4             | Byte offset indicating where form index data starts.      | `1024`      |
| `bytestart_data`  | `u32`         | 4             | Byte offset indicating where the actual form data starts. | `2048`      |
#### Data:

The **Data Block** holds the actual form data for each form in the archive. Each form's structure depends on its type (refer to **[[SmnArchive - Type Structures - Forms]]** for more details on the data representation).


#### Index:

| **Data Name**       | **Data Type** | **Byte Size** | **Data Description**                                                | **Example** |
| ------------------- | ------------- | ------------- | ------------------------------------------------------------------- | ----------- |
| `form_id`           | `FormID`      | 4             | Unique identifier for each form in the list.                        | `67890`     |
| `form_type`         | `FormType`    | 1             | Type of each form, indicating the structure or data format.         | `1`         |
| `byte_start_offset` | `u32`         | 4             | Offset from the **Data Block's Bytestart**, where form data starts. | `256`       |

The index is a ***list***, where each item follows the structure above. The list size is determined by the number of forms in the archive, as indicated by the `form_count` field.

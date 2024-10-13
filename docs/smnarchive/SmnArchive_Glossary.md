# Glossary
A glossary of terms used in the SmnArchive library.

---

| **Name**         | **Description**                                                                                              | **Reference Link**        |
| ---------------- | ------------------------------------------------------------------------------------------------------------ | ------------------------- |
| **Archive**      | A binary file structure that holds multiple forms and their data in different blocks for efficient access.   | [SmnArchive](./SmnArchive.md) |
| **Form**         | The fundamental unit of data stored in an archive, which can represent strings, world data, or references.   | [SmnArchive - Type Structures - Forms](./SmnArchive_Type_Structures_Forms.md) |
| **ArchiveID**    | A unique 1-byte identifier used to distinguish different archives in the system.                             | [SmnArchive - Type Structures - Values](./SmnArchive_Type_Structures_Values.md) |
| **FormID**       | A 2-byte unique identifier used to represent forms in the archive.                                           | [SmnArchive - Type Structures - Values](./SmnArchive_Type_Structures_Values.md) |
| **FormType**     | A 1-byte enum that specifies the type of form (e.g., STRING, WORLD, REFGROUP).                               | [SmnArchive - Type Structures - Forms](./SmnArchive_Type_Structures_Forms.md) |
| **GlobalID**     | A 3-byte identifier that combines the `ArchiveID` and `FormID`. It provides a globally unique identifier.    | [SmnArchive - Type Structures - Values](./SmnArchive_Type_Structures_Values.md) |
| **StrSml**       | A small UTF-16 encoded string (up to 255 characters). Used for shorter string values in forms.               | [SmnArchive - Type Structures - Values](./SmnArchive_Type_Structures_Values.md) |
| **StrLrg**       | A large UTF-16 encoded string (up to 65,535 characters). Used for longer string values in forms.             | [SmnArchive - Type Structures - Values](./SmnArchive_Type_Structures_Values.md) |
| **FormWorld**    | A form that represents a world structure with a name, map, and parts that make up the world.                 | [SmnArchive - Type Structures - Forms](./SmnArchive_Type_Structures_Forms.md) |
| **FormString**   | A form that stores strings in multiple languages. Each form can contain several strings for different langs. | [SmnArchive - Type Structures - Forms](./SmnArchive_Type_Structures_Forms.md) |
| **FormRefGroup** | A form that holds references to other forms, used for grouping related forms together.                       | [SmnArchive - Type Structures - Forms](./SmnArchive_Type_Structures_Forms.md) |
| **Version**      | A 2-byte value representing the major and minor version numbers of the archive.                              | [SmnArchive - Type Structures - Values](./SmnArchive_Type_Structures_Values.md) |
| **LangCode**     | A 1-byte enum that represents language codes (e.g., EN for English, FR for French).                          | [SmnArchive - Type Structures - Values](./SmnArchive_Type_Structures_Values.md) |
| **ByteStart**    | A block that holds byte offsets indicating where the form index and form data start in the file.             | [SmnArchive - Type Structures - Binary File](./SmnArchive_Type_Structures_Binary_File.md) |
| **Index Block**  | A block that holds the index pointing to the location of form data within the archive.                       | [SmnArchive - Type Structures - Binary File](./SmnArchive_Type_Structures_Binary_File.md) |
| **Data Block**   | A block that stores the actual content or data of each form in the archive.                                  | [SmnArchive - Type Structures - Binary File](./SmnArchive_Type_Structures_Binary_File.md) |


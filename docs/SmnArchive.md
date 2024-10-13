# SmnArchive
A library for managing the SmnArchive (.smn) binary archive structure. The binary structure is set up for real-time streaming of data, with a focus on performant reading. 

**Notable Info:**
- [SmnArchive - Type Structures - Binary File](./smnarchive/SmnArchive_Type_Structures_Binary_File.md) - Describes the archive file's block-based layout.
- [SmnArchive - Type Structures - Values](./smnarchive/SmnArchive_Type_Structures_Values.md) - Defines sizes and formats for a form's stored value types.
- [SmnArchive - Type Structures - Forms](./smnarchive/SmnArchive_Type_Structures_Forms.md) - Describes structures for different form types.
- [SmnArchive - Glossary](./smnarchive/SmnArchive_Glossary.md) - Glossary of terms used in the SmnArchive library.
- [SmnArchive - Statistics](./smnarchive/SmnArchive_Statistics.md) - Performance statistics for reading and writing forms.


The library supports various CRUD operations on the archive:

**Archive:**
- `write_archive_skeleton`: Creating archive.
- `read_archive_info`: Getting archive info.
- `write_archive_info:` Writing archive info.
- `read_lite_archive`: Getting lite archive (list of simple form data).

**Forms:**
- `read_form`: Getting form data.
- `read_forms`: Getting multiple form data.
- `write_form`: Writing form data.
- `delete_form`: Delete form data.
- `get_form_exists`: Checking if a form exists in an archive..

The library also holds structures for safely working with different types of forms, and data structures for form content (managed 'types'):

**Form Structures:**
- `struc_form` (Base form structure for all forms)
- `struc_form_world` (Structure for *WORLD* forms)
- `struc_form_string` (Structure for *STRING* forms)
- `struc_form_refgroup` (Structure for *REFGROUP* forms)
- `struc_form_worldpart` (Structure for *WORLDPART* forms)

**Type Structures:**
 - `types_id` (Different ID Types; *ArchiveID*, *FormID*, *GlobalID*)
 - `types_misc` (Different Misc Types; *FormType*, *LangCode*)
 - `types_str`(Different String Types; *StrSml*, *StrLrg*)
 - `types_vector`(Different Vector Types; *Vec2Int*, *Vec3Int* *Vec2Float*, *Vec3Float*)
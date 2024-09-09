### **ArchiveID**

- **Size:** 1 byte
- **Range:** Integer between 0 and 255.
- **Representation:** Written and displayed either as a 3-character string (e.g., "001" for 1) or its integer counterpart (e.g., 1).
- **Example:** `"001"` or `1`

### **FormID**

- **Size:** 2 bytes
- **Range:** Integer between 0 and 65,535.
- **Representation:** Written and displayed as a 5-character string (e.g., "00203" for 203) or its integer counterpart (e.g., 203).
- **Example:** `"00203"` or `203`

### **GlobalID**

- **Size:** 3 bytes
- **Structure:** A composite identifier that combines the ArchiveID (1 byte) and FormID (2 bytes) into a single reference. This allows referencing forms across archives.
- **Representation:** A combination of the 3-character ArchiveID and the 5-character FormID, resulting in an 8-character string (e.g., "00100203").
- **Example:** `"00100203"`
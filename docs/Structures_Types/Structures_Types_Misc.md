### **Version**

- **Size:** 2 bytes (major and minor version each take 1 byte).
- **Representation:**
    - Major and minor version numbers represented as a string (e.g., `"0.5"`).
    - Can also be represented as a 2-byte array `[major, minor]`.
    - Can convert to a `f32` float value (e.g., `0.5`).
- **Example:** `"0.5"`, `[0, 5]`, or `0.5`

### **LangCode**

- **Size:** 1 byte
- **Representation:**
    - Enum for language codes, each represented as an integer value:
        - `EN = 1`
        - `FR = 2`
        - `ES = 3`
        - `DE = 4`
    - Can be displayed as a string (`"EN"`, `"FR"`, etc.) or an integer byte (`1`, `2`, etc.).
- **Example:** `"EN"` or `1`

### **FormType**

- **Size:** 1 byte
- **Representation:**
    - Enum for form types, each represented as an integer value:
        - `STRING = 0`
        - `WORLD = 1`
        - `REFGROUP = 2`
    - Can be displayed as a string (`"STRING"`, `"WORLD"`, etc.) or as an integer byte (`0`, `1`, etc.).
- **Example:** `"STRING"` or `0`

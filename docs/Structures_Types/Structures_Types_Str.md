### **StrSml**

- **Size:**
    - The length of the string is represented in **1 byte**.
    - Each character is represented using **UTF-16 encoding**, where each character is stored as a **2-byte digit code** representing the character's Unicode value, not the raw character itself.
- **Representation:**
    - The total size is the **1-byte length field**, followed by the **digit codes** of each character in UTF-16 (2 bytes per character).
    - The UTF-16 **digit codes** correspond to the character's position in the Unicode character set, where ASCII characters (e.g., letters) are represented by their Unicode values (e.g., 'H' is 0x0048, 'e' is 0x0065).
    - **Maximum string length:** 255 characters (since the length is stored in 1 byte).
- **Example:**
    - For the string `"Hello"`, the UTF-16 **digit codes** are:
        - `"H"`: `0x0048`
        - `"e"`: `0x0065`
        - `"l"`: `0x006C`
        - `"l"`: `0x006C`
        - `"o"`: `0x006F`
    - In bytes: `05 00 48 00 65 00 6C 00 6C 00 6F`
    - **Length byte**: `05` (indicating 5 characters), followed by the UTF-16 digit codes for each character.

### **StrLrg**

- **Size:**
    - The length of the string is represented in **2 bytes**.
    - Each character is represented using **UTF-16 encoding**, where each character is stored as a **2-byte digit code** representing the character's Unicode value, not the raw character itself.
- **Representation:**
    - The total size is the **2-byte length field**, followed by the **digit codes** of each character in UTF-16 (2 bytes per character).
    - The UTF-16 **digit codes** correspond to the character's Unicode value, where ASCII characters (e.g., letters) are represented by their Unicode values (e.g., 'H' is 0x0048).
    - **Maximum string length:** 65,535 characters (since the length is stored in 2 bytes).
- **Example:**
    - For the string `"Hello"`, the UTF-16 **digit codes** are:
        - `"H"`: `0x0048`
        - `"e"`: `0x0065`
        - `"l"`: `0x006C`
        - `"l"`: `0x006C`
        - `"o"`: `0x006F`
    - In bytes: `00 05 00 48 00 65 00 6C 00 6C 00 6F`
    - **Length bytes**: `00 05` (indicating 5 characters), followed by the UTF-16 digit codes for each character.

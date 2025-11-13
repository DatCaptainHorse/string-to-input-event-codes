use std::collections::HashMap;

// Linux input event codes
pub const KEY_LEFTCTRL: u32 = 29;
pub const KEY_LEFTSHIFT: u32 = 42;
pub const KEY_U: u32 = 22;
pub const KEY_SPACE: u32 = 57;

lazy_static::lazy_static! {
    static ref CHAR_TO_SCANCODE: HashMap<char, u32> = {
        let mut m = HashMap::new();

        // Mapping from input-event-codes.h..
        m.insert('a', 30); // KEY_A
        m.insert('b', 48); // KEY_B
        m.insert('c', 46); // KEY_C
        m.insert('d', 32); // KEY_D
        m.insert('e', 18); // KEY_E
        m.insert('f', 33); // KEY_F
        m.insert('g', 34); // KEY_G
        m.insert('h', 35); // KEY_H
        m.insert('i', 23); // KEY_I
        m.insert('j', 36); // KEY_J
        m.insert('k', 37); // KEY_K
        m.insert('l', 38); // KEY_L
        m.insert('m', 50); // KEY_M
        m.insert('n', 49); // KEY_N
        m.insert('o', 24); // KEY_O
        m.insert('p', 25); // KEY_P
        m.insert('q', 16); // KEY_Q
        m.insert('r', 19); // KEY_R
        m.insert('s', 31); // KEY_S
        m.insert('t', 20); // KEY_T
        m.insert('u', 22); // KEY_U
        m.insert('v', 47); // KEY_V
        m.insert('w', 17); // KEY_W
        m.insert('x', 45); // KEY_X
        m.insert('y', 21); // KEY_Y
        m.insert('z', 44); // KEY_Z

        // Numbers
        m.insert('1', 2);  // KEY_1
        m.insert('2', 3);  // KEY_2
        m.insert('3', 4);  // KEY_3
        m.insert('4', 5);  // KEY_4
        m.insert('5', 6);  // KEY_5
        m.insert('6', 7);  // KEY_6
        m.insert('7', 8);  // KEY_7
        m.insert('8', 9);  // KEY_8
        m.insert('9', 10); // KEY_9
        m.insert('0', 11); // KEY_0

        // Special keys
        m.insert(' ', 57);  // KEY_SPACE
        m.insert('\n', 28); // KEY_ENTER
        m.insert('\t', 15); // KEY_TAB
        m.insert('-', 12);  // KEY_MINUS
        m.insert('=', 13);  // KEY_EQUAL
        m.insert('[', 26);  // KEY_LEFTBRACE
        m.insert(']', 27);  // KEY_RIGHTBRACE
        m.insert('\\', 43); // KEY_BACKSLASH
        m.insert(';', 39);  // KEY_SEMICOLON
        m.insert('\'', 40); // KEY_APOSTROPHE
        m.insert(',', 51);  // KEY_COMMA
        m.insert('.', 52);  // KEY_DOT
        m.insert('/', 53);  // KEY_SLASH
        m.insert('`', 41);  // KEY_GRAVE

        m
    };

    static ref SHIFTED_CHARS: HashMap<char, u32> = {
        let mut m = HashMap::new();

        // Uppercase letters
        m.insert('A', 30); m.insert('B', 48); m.insert('C', 46); m.insert('D', 32);
        m.insert('E', 18); m.insert('F', 33); m.insert('G', 34); m.insert('H', 35);
        m.insert('I', 23); m.insert('J', 36); m.insert('K', 37); m.insert('L', 38);
        m.insert('M', 50); m.insert('N', 49); m.insert('O', 24); m.insert('P', 25);
        m.insert('Q', 16); m.insert('R', 19); m.insert('S', 31); m.insert('T', 20);
        m.insert('U', 22); m.insert('V', 47); m.insert('W', 17); m.insert('X', 45);
        m.insert('Y', 21); m.insert('Z', 44);

        // Shifted symbols
        m.insert('!', 2);  // Shift+1
        m.insert('@', 3);  // Shift+2
        m.insert('#', 4);  // Shift+3
        m.insert('$', 5);  // Shift+4
        m.insert('%', 6);  // Shift+5
        m.insert('^', 7);  // Shift+6
        m.insert('&', 8);  // Shift+7
        m.insert('*', 9);  // Shift+8
        m.insert('(', 10); // Shift+9
        m.insert(')', 11); // Shift+0
        m.insert('_', 12); // Shift+-
        m.insert('+', 13); // Shift+=
        m.insert('{', 26); // Shift+[
        m.insert('}', 27); // Shift+]
        m.insert('|', 43); // Shift+\
        m.insert(':', 39); // Shift+;
        m.insert('"', 40); // Shift+'
        m.insert('<', 51); // Shift+,
        m.insert('>', 52); // Shift+.
        m.insert('?', 53); // Shift+/
        m.insert('~', 41); // Shift+`

        m
    };
}

/// Convert a string to a flat Vec of Linux input event codes
/// Handles Unicode via Ctrl+Shift+U + hex + space sequence
pub fn string_to_scancodes(s: &str) -> Vec<u32> {
    let mut scancodes = Vec::new();

    for c in s.chars() {
        // try direct mapping first
        if let Some(&code) = CHAR_TO_SCANCODE.get(&c) {
            scancodes.push(code);
        }
        // try shifted mapping
        else if let Some(&code) = SHIFTED_CHARS.get(&c) {
            scancodes.push(KEY_LEFTSHIFT);
            scancodes.push(code);
        }
        // unicode hex input..
        else {
            scancodes.push(KEY_LEFTCTRL);
            scancodes.push(KEY_LEFTSHIFT);
            scancodes.push(KEY_U);

            let hex = format!("{:x}", c as u32);
            for hex_char in hex.chars() {
                let code = match hex_char {
                    '0' => 11, // KEY_0
                    '1' => 2,  // KEY_1
                    '2' => 3,  // KEY_2
                    '3' => 4,  // KEY_3
                    '4' => 5,  // KEY_4
                    '5' => 6,  // KEY_5
                    '6' => 7,  // KEY_6
                    '7' => 8,  // KEY_7
                    '8' => 9,  // KEY_8
                    '9' => 10, // KEY_9
                    'a' => 30, // KEY_A
                    'b' => 48, // KEY_B
                    'c' => 46, // KEY_C
                    'd' => 32, // KEY_D
                    'e' => 18, // KEY_E
                    'f' => 33, // KEY_F
                    _ => continue,
                };
                scancodes.push(code);
            }

            scancodes.push(KEY_SPACE);
        }
    }

    scancodes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii() {
        let codes = string_to_scancodes("hello");
        assert_eq!(codes, vec![35, 18, 38, 38, 24]); // h e l l o
    }

    #[test]
    fn test_shifted() {
        let codes = string_to_scancodes("Hello!");
        // H = shift + h, ! = shift + 1
        assert!(codes.contains(&KEY_LEFTSHIFT));
        assert!(codes.contains(&35)); // h
        assert!(codes.contains(&2));  // 1 (for !)
    }

    #[test]
    fn test_unicode() {
        let codes = string_to_scancodes("中");
        // Should contain Ctrl+Shift+U sequence
        assert!(codes.contains(&KEY_LEFTCTRL));
        assert!(codes.contains(&KEY_U));
        assert!(codes.contains(&KEY_SPACE));
    }
}
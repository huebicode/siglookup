pub fn is_pattern_in_hex(hex_bytes: &[String], hex_pattern: &str) -> bool {
    let pattern_elements: Vec<&str> = hex_pattern.split_whitespace().take(64).collect();

    // Enumerate over the mapped hex bytes
    'outer: for (i, byte) in hex_bytes.iter().enumerate() {
        // Check for wildcard
        if pattern_elements[i] == "_" {
            if i == pattern_elements.len() - 1 {
                return true;
            } else {
                continue;
            }
        }

        // Check for match in multiple values
        if pattern_elements[i].contains('|') {
            let splitted_parts: Vec<&str> = pattern_elements[i].split('|').collect();
            for (j, _byte) in splitted_parts.iter().enumerate() {
                if byte == _byte {
                    if i == pattern_elements.len() - 1 {
                        return true;
                    } else {
                        continue 'outer;
                    }
                } else if j == splitted_parts.len() - 1 {
                    return false;
                }
            }
        }

        // Check for range match
        if pattern_elements[i].contains('-') {
            let range: Vec<&str> = pattern_elements[i].split('-').collect();
            let start = u8::from_str_radix(range[0], 16).unwrap();
            let end = u8::from_str_radix(range[1], 16).unwrap();
            let current_byte = u8::from_str_radix(byte, 16).unwrap();

            if current_byte >= start && current_byte <= end {
                if i == pattern_elements.len() - 1 {
                    return true;
                } else {
                    continue 'outer;
                }
            } else {
                return false;
            }
        }

        // Check for single value match
        if *byte == pattern_elements[i] {
            if i == pattern_elements.len() - 1 {
                return true;
            } else {
                continue;
            }
        } else {
            return false;
        }
    }

    false
}

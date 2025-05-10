pub fn is_pattern_in_hex(hex_bytes: &[String], hex_pattern: &str) -> bool {
    let pattern_elements: Vec<&str> = hex_pattern.split_whitespace().take(64).collect();
    let mut i = 0; // Index for hex_bytes
    let mut p = 0; // Index for pattern_elements

    fn is_wildcard_match(pattern: &str, hex_byte: &str) -> bool {
        if pattern.len() == 2 && hex_byte.len() == 2 {
            match pattern.as_bytes() {
                // First half is wildcard (e.g. ?A)
                [b'?', second] => hex_byte.as_bytes()[1] == *second,
                // Second half is wildcard (e.g. A?)
                [first, b'?'] => hex_byte.as_bytes()[0] == *first,
                _ => false,
            }
        } else {
            false
        }
    }

    while p < pattern_elements.len() && i < hex_bytes.len() {
        let pattern = pattern_elements[p];

        // Check for multiplier pattern (e.g. ??x5, AAx5 or ?Ax5)
        if let Some(x_pos) = pattern.find('x') {
            if x_pos > 0 && x_pos < pattern.len() - 1 {
                let base_pattern = &pattern[0..x_pos];
                if let Ok(repeat_count) = pattern[x_pos + 1..].parse::<usize>() {
                    if i + repeat_count > hex_bytes.len() {
                        return false;
                    }

                    if base_pattern == "??" {
                        i += repeat_count;
                        p += 1;
                        continue;
                    } else {
                        for j in 0..repeat_count {
                            let current_pattern = base_pattern;
                            let current_byte = &hex_bytes[i + j];

                            if current_pattern.contains('?') {
                                if !is_wildcard_match(current_pattern, current_byte) {
                                    return false;
                                }
                            } else if current_byte != current_pattern {
                                return false;
                            }
                        }
                        i += repeat_count;
                        p += 1;
                        continue;
                    }
                }
            }
        }

        // Check for wildcards
        if pattern.contains('?') {
            if pattern == "??" {
                i += 1;
                p += 1;
                if p >= pattern_elements.len() {
                    return true;
                }
                continue;
            }

            if pattern.len() == 2 {
                let hex_byte = &hex_bytes[i];
                if !is_wildcard_match(pattern, hex_byte) {
                    return false;
                }
                i += 1;
                p += 1;
                continue;
            }
        }

        // Check for match in multiple values
        if pattern.contains('|') {
            let splitted_parts: Vec<&str> = pattern.split('|').collect();
            let mut matched = false;

            for part in splitted_parts.iter() {
                if &hex_bytes[i] == part {
                    matched = true;
                    break;
                }
            }

            if !matched {
                return false;
            }
        }
        // Check for range match
        else if pattern.contains('-') {
            let range: Vec<&str> = pattern.split('-').collect();
            let start = u8::from_str_radix(range[0], 16).unwrap_or(0);
            let end = u8::from_str_radix(range[1], 16).unwrap_or(255);
            let current_byte = u8::from_str_radix(&hex_bytes[i], 16).unwrap_or(0);

            if !(current_byte >= start && current_byte <= end) {
                return false;
            }
        }
        // Check for single value match
        else if hex_bytes[i] != pattern {
            return false;
        }

        i += 1;
        p += 1;
    }

    p >= pattern_elements.len()
}

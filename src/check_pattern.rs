pub fn is_pattern_in_hex(hex_bytes: &[String], hex_pattern: &str) -> bool {
    let pattern_elements: Vec<&str> = hex_pattern.split_whitespace().take(64).collect();
    let mut i = 0;

    while i < pattern_elements.len() && i < hex_bytes.len() {
        let pattern = pattern_elements[i];

        // Check for skip pattern [n]
        if pattern.starts_with('[') && pattern.ends_with(']') {
            if let Ok(skip_count) = pattern[1..pattern.len() - 1].parse::<usize>() {
                i += skip_count;
                if i >= pattern_elements.len() {
                    return true;
                }
                continue;
            }
        }

        // Check for wildcards
        if pattern.contains('?') {
            if pattern == "??" {
                i += 1;
                if i >= pattern_elements.len() {
                    return true;
                }
                continue;
            }

            if pattern.len() == 2 {
                let hex_byte = &hex_bytes[i];
                if hex_byte.len() == 2 {
                    let matches = match pattern.as_bytes() {
                        // First half is wildcard (e.g., ?A)
                        [b'?', second] => {
                            let hex_second = hex_byte.as_bytes()[1];
                            hex_second == *second
                        }
                        // Second half is wildcard (e.g., A?)
                        [first, b'?'] => {
                            let hex_first = hex_byte.as_bytes()[0];
                            hex_first == *first
                        }
                        _ => false,
                    };

                    if !matches {
                        return false;
                    }
                    i += 1;
                    continue;
                }
            }
        }

        // Check for match in multiple values
        if pattern.contains('|') {
            let cleaned = pattern.replace(['(', ')'], "");
            let splitted_parts: Vec<&str> = cleaned.split('|').collect();
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
    }

    i >= pattern_elements.len()
}

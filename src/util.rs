/// Clean the message from newlines and carriage returns and convert it to lowercase. Also remove all brackets.
pub fn clean_message(text: &str) -> String {
    let mut result = String::new();
    let mut inside_braces = false;

    for part in text.split_inclusive(&['{', '}']) {
        if part.starts_with('{') && part.ends_with('}') {
            // Part is fully inside braces
            inside_braces = true;
            result.push_str(part);
        } else if inside_braces {
            // We're processing a segment after an opening brace
            inside_braces = false;
            result.push_str(&part);
        } else {
            // Process the part outside braces
            result.push_str(&part.replace('\n', "").replace('\r', "").to_lowercase());
        }
    }

    result
}

pub fn parse_arguments(input: &str) -> Vec<&str> {
    let mut arguments = Vec::new();
    let mut start = 0;
    let mut in_brackets = 0; // Tracks nesting level for curly braces

    for (i, c) in input.chars().enumerate() {
        match c {
            '{' => in_brackets += 1,
            '}' => in_brackets -= 1,
            ',' if in_brackets == 0 => {
                // Outside of brackets, treat comma as a delimiter
                let slice = &input[start..i].trim();
                if !slice.is_empty() {
                    arguments.push(*slice); // Dereference slice here
                }
                start = i + 1;
            }
            _ => {}
        }
    }

    // Push the final argument if it's not empty
    if start < input.len() {
        let slice = &input[start..].trim();
        if !slice.is_empty() {
            arguments.push(*slice); // Dereference slice here
        }
    }

    arguments
}

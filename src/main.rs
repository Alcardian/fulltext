use arboard::Clipboard;

fn main() {
    // Create a new, mutable clipboard instance.
    let mut clipboard = Clipboard::new().expect("Failed to create clipboard");
    
    // Retrieve the current clipboard text.
    let text = clipboard.get_text().expect("Failed to get clipboard text");
	
	// Convert the text.
    let converted_text = convert_text(&text);
	
	// Copy the converted text back into the clipboard.
    clipboard.set_text(converted_text).expect("Failed to set clipboard text");
}

fn convert_text(input: &str) -> String {
    let mut result = String::new();

    for line in input.lines() {
        // Use only trailing trim so we keep leading whitespace for detection.
        let line = line.trim_end();

        // Check if the line is completely blank after full trim.
        if line.trim().is_empty() {
            continue;
        }

        // If the line starts with spaces (indicating an indented line)
        if line.starts_with(' ') && line.contains('*') {
            if let Some(idx) = line.find('*') {
                result.push(' ');
                result.push_str(line[idx + 1..].trim());
                continue;
            }
        }

        // Now check for bullet points at the start (ignoring leading whitespace)
        let trimmed = line.trim_start();
        if trimmed.starts_with("* ") {
            if !result.is_empty() {
                result.push('\n');
            }
            result.push_str(trimmed.trim_start_matches("* ").trim());
        } else if trimmed == "*" {
            result.push('\n');
        }
    }

    result
}



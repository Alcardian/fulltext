use arboard::Clipboard;

fn main() {
    println!("Hello, world!");
	
	// Match: expression handles both successful and error outcomes from calling read_clipboard()
	match read_clipboard() {
        Ok(text) => {
            println!("Clipboard text: {}", text);
            println!("Converted text:\n{}", convert_text(&text));
        }
        Err(e) => eprintln!("Failed to read clipboard: {}", e),
    }
}

fn read_clipboard() -> Result<String, Box<dyn std::error::Error>> {
    // Create a new, mutable clipboard instance.
    // The '?' operator returns an error if creation fails.
    let mut clipboard = Clipboard::new()?;
	
    // Retrieve and return the clipboard text.
    Ok(clipboard.get_text()?)
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
                let content = line[idx + 1..].trim();
                result.push(' ');
                result.push_str(content);
                continue;
            }
        }

        // Now check for bullet points at the start (ignoring leading whitespace)
        let trimmed = line.trim_start();
        if trimmed.starts_with("* ") {
            if !result.is_empty() {
                result.push('\n');
            }
            let content = trimmed.trim_start_matches("* ").trim();
            result.push_str(content);
        } else if trimmed == "*" {
            result.push('\n');
        }
    }

    result
}



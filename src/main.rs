use arboard::Clipboard;

fn main() {
    println!("Hello, world!");
	
	// Match: expression handles both successful and error outcomes from calling read_clipboard()
	match read_clipboard() {
        Ok(text) => println!("Clipboard text: {}", text),
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

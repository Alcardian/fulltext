use arboard::Clipboard;

fn main() {
  // Create a new, mutable clipboard instance.
  let mut clipboard = Clipboard::new().expect("Failed to create clipboard");

  let text = clipboard.get_text().expect("Failed to get clipboard text");
  let converted_text = convert_text(&text);

  // TODO, new function

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

#[cfg(test)]
mod tests {
  use super::*;

  // Happy flow test: mixed bullet points, continuation lines, and blank lines.
  #[test]
  fn test_convert_text_happy_flow() {
    let input = "\
* 1st line.
  * Still 1st line. *italic text*,
  * and some more text.
* Here's the 2nd line. **BOLD**

* Here's the 3rd line.
*
* 4th line is empty, so this is the 5th line.";
    let expected = "\
1st line. Still 1st line. *italic text*, and some more text.
Here's the 2nd line. **BOLD**
Here's the 3rd line.

4th line is empty, so this is the 5th line.";
    let result = convert_text(input);
    assert_eq!(result, expected);
  }

  // Test continuation line: a bullet point followed by an indented line.
  #[test]
  fn test_continuation_line() {
    let input = "* First bullet\n  * continuation line";
    let expected = "First bullet continuation line";
    let result = convert_text(input);
    assert_eq!(result, expected);
  }

  // Test a line that contains only a "*" which should produce a newline.
  #[test]
  fn test_line_with_only_star() {
    let input = "*";
    let expected = "\n";
    let result = convert_text(input);
    assert_eq!(result, expected);
  }

  // Test that blank lines are ignored.
  #[test]
  fn test_blank_lines_ignored() {
    let input = "* Bullet one\n\n* Bullet two\n\n* Bullet three ";
    let expected = "Bullet one\nBullet two\nBullet three";
    let result = convert_text(input);
    assert_eq!(result, expected);
  }

  // Test that lines without a bullet or an indented bullet are ignored.
  #[test]
  fn test_normal_text_ignored() {
    let input = "Normal line\nAnother normal line";
    let expected = "";  // Since these lines don't match any condition, they are omitted.
    let result = convert_text(input);
    assert_eq!(result, expected);
  }

  // Test that extra spaces after the bullet marker are trimmed.
  #[test]
  fn test_extra_spaces_in_bullet() {
    let input = "*   Extra spaces text  ";
    let expected = "Extra spaces text";
    let result = convert_text(input);
    assert_eq!(result, expected);
  }

	#[test]
	fn test_line_without_star_is_ignored() {
		let input = "* Valid 1\nInvalid\n* Valid \n  * 2 ";
		let expected = "Valid 1\nValid 2";
		let result = convert_text(input);
		assert_eq!(result, expected);
	}
}



use arboard::Clipboard;
use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
  // Create a new, mutable clipboard instance.
  let mut clipboard = Clipboard::new().expect("Failed to create clipboard");
  let text = clipboard.get_text().expect("Failed to get clipboard text");
  let converted_text = convert_text(&text);
  let html = convert_to_html(&converted_text);
  clipboard.set_text(&converted_text).expect("Failed to set clipboard text");

  // Set the HTML based on OS
  #[cfg(target_os = "linux")]
  {
    // Requires xclip installed
    let mut p = Command::new("xclip")
    .args(&["-selection", "clipboard", "-t", "text/html"])
    .stdin(Stdio::piped())
    .spawn()
    .expect("xclip spawn failed: ensure xclip is installed");
    let stdin = p.stdin.as_mut().expect("Failed to open xclip stdin");
    stdin.write_all(html.as_bytes()).expect("Failed to write to xclip stdin");
    stdin.flush().expect("Failed to flush xclip stdin");
    let status = p.wait().expect("xclip process failed to complete");
    if !status.success() {
      panic!("xclip exited with error: {}", status);
    }
  }
  #[cfg(target_os = "macos")]
  {
    // macOS pbcopy supports -Prefer html
    let mut p = Command::new("pbcopy")
    .args(&["-Prefer", "html"])
    .stdin(Stdio::piped())
    .spawn()
    .expect("pbcopy spawn failed");
    p.stdin.as_mut().unwrap().write_all(html.as_bytes()).unwrap();
    p.wait().unwrap();
  }
  #[cfg(target_os = "windows")]
  {
    // Use the clipboard-win crate
    use clipboard_win::{formats, Clipboard};
    let _ = Clipboard::new().and_then(|clip| {
      formats::Html::set(html.as_bytes())
    }).expect("Failed to set Windows HTML clipboard");
  }
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

fn convert_to_html(input: &str) -> String {
  let mut res = String::new();
  let mut chars = input.chars().peekable();
  while let Some(char) = chars.next() {
    if char == '*' {
      // Check for bold (**text**)
      if chars.peek() == Some(&'*') {
        chars.next(); // consume the second *
        let mut bold_text = String::new();
        let mut found_closing = false;
        while let Some(next_char) = chars.next() {
          if next_char == '*' && chars.peek() == Some(&'*') {
            chars.next(); // consume the second closing *
            found_closing = true;
            break;
          }
          bold_text.push(next_char);
        }
        if found_closing {
          res.push_str(&format!("<strong>{}</strong>", html_escape(&bold_text)));
        } else {
          // If no closing **, treat as literal text
          res.push_str("**");
          res.push_str(&html_escape(&bold_text));
        }
      } else {
        // Check for italic (*text*)
        let mut italic_text = String::new();
        let mut found_closing = false;
        while let Some(next_char) = chars.next() {
          if next_char == '*' {
            found_closing = true;
            break;
          }
          italic_text.push(next_char);
        }
        if found_closing && !italic_text.is_empty() {
          res.push_str(&format!("<em>{}</em>", html_escape(&italic_text)));
        } else {
          // If no closing * or empty content, treat as literal
          res.push('*');
          res.push_str(&html_escape(&italic_text));
        }
      }
    } else if char == '\n' {
      res.push_str("<br>");
    } else {
      res.push_str(&html_escape(&char.to_string()));
    }
  }

  // Wrap the content in a CF_HTML-like structure for compatibility with LibreOffice
  let html_body = format!("<div>{}</div>", res);
  format!(
    "
<html><body>\r\n\
{body}\r\n\
</body></html>",
body = html_body
  )
}

fn html_escape(text: &str) -> String {
  text.replace('&', "&amp;")
  .replace('<', "&lt;")
  .replace('>', "&gt;")
  .replace('"', "&quot;")
  .replace('\'', "&#39;")
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

  // Tests for HTML conversion
  #[test]
  fn test_convert_to_html_basic() {
    let input = "Hello world";
    let expected = "\n<html><body>\r\n<div>Hello world</div>\r\n</body></html>";
    let result = convert_to_html(input);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_convert_to_html_with_bold() {
    let input = "This is **bold** text";
    let expected = "\n<html><body>\r\n<div>This is <strong>bold</strong> text</div>\r\n</body></html>";
    let result = convert_to_html(input);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_convert_to_html_with_italic() {
    let input = "This is *italic* text";
    let expected = "\n<html><body>\r\n<div>This is <em>italic</em> text</div>\r\n</body></html>";
    let result = convert_to_html(input);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_convert_to_html_with_mixed_formatting() {
    let input = "This has **bold** and *italic* text";
    let expected = "\n<html><body>\r\n<div>This has <strong>bold</strong> and <em>italic</em> text</div>\r\n</body></html>";
    let result = convert_to_html(input);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_convert_to_html_with_newlines() {
    let input = "Line 1\nLine 2\nLine 3";
    let expected = "\n<html><body>\r\n<div>Line 1<br>Line 2<br>Line 3</div>\r\n</body></html>";
    let result = convert_to_html(input);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_convert_to_html_html_escaping() {
    let input = "Text with <tags> & \"quotes\"";
    let expected = "\n<html><body>\r\n<div>Text with &lt;tags&gt; &amp; &quot;quotes&quot;</div>\r\n</body></html>";
    let result = convert_to_html(input);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_convert_to_html_incomplete_formatting() {
    // TODO - This test will fail until convert_to_html(..) logic won't grab a '*' if it directly is followed by another '** UNLESS there are 3 '*' in a row
    let input = "This has *incomplete italic and **incomplete bold";
    let expected = "\n<html><body>\r\n<div>This has *incomplete italic and **incomplete bold</div>\r\n</body></html>";
    let result = convert_to_html(input);
    assert_eq!(result, expected);
  }

  #[test]
  fn test_convert_to_html_empty_formatting() {
    let input = "Empty ** and * markers";
    let expected = "\n<html><body>\r\n<div>Empty ** and * markers</div>\r\n</body></html>";
    let result = convert_to_html(input);
    assert_eq!(result, expected);
  }

  // Integration test combining both functions
  #[test]
  fn test_full_pipeline() {
    let input = "* This is **bold** text\n* This is *italic* text";
    let converted = convert_text(input);
    let expected_converted = "This is **bold** text\nThis is *italic* text";
    assert_eq!(converted, expected_converted);

    let html = convert_to_html(&converted);
    let expected_html = "\n<html><body>\r\n<div>This is <strong>bold</strong> text<br>This is <em>italic</em> text</div>\r\n</body></html>";
    assert_eq!(html, expected_html);
  }

}

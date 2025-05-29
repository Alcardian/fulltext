# Fulltext - Rust
Originally a rewrite of a script I created to convert text for me from "Bulletpoint Format" to "Full Text".  
Now able to copy "bulletpoint format" and paste "full text" with styling included directly into Libre Office Writer.

# Running the Program
## Linux
* Requires xclip.
  * Can be installed with "sudo pacman -S xclip" on arch.

## Windows
* Only 1.0.0 version tested on Windows.

## MacOS
* Untested.

# Developing
## Development Build
To build during development, run:
```
cargo build
```

If it doesn't work, try:
```
cargo update
```

## Verify Unit Tests
Note: "test_convert_to_html_incomplete_formatting" will fail until program can handle incomplete italic and bold correctly. Should show 1 failed.
```
cargo test
```

## Release Build
Longer build time, but more optimized and smaller executable.
```
cargo run --release
```

# Manual Test Data
```
* 1st line.
    * Still 1st line. *italic text*,
    * and some more text.
* Here's the 2nd line. **BOLD**

* Here's the 3rd line.
*
* 4th line is empty, so this is the 5th line.
```

## Expected Result
```
1st line. Still 1st line. *italic text*, and some more text.
Here's the 2nd line. **BOLD**
Here's the 3rd line.

4th line is empty, so this is the 5th line.
```

# Motivation
1. To make it easier to make changes, real coding languages are easier for me than trying to do logic in scripts.
2. To learn Rust.

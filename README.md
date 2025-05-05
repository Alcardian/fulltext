# Fulltext - Rust
A rewrite of a script I created to convert text for me from "Bulletpoint Format" to "Full Text".

## Motivation
1. To make it easier to make changes, real coding languages are easier for me than trying to do logic in scripts.
2. To learn Rust.

## Notes
If you can't "cargo build", try "cargo update" first.

## Manual Test Data
* 1st line.
    * Still 1st line. *italic text*,
    * and some more text.
* Here's the 2nd line. **BOLD**

* Here's the 3rd line.
*
* 4th line is empty, so this is the 5th line.

### Expected Result
1st line. Still 1st line. *italic text*, and some more text.
Here's the 2nd line. **BOLD**
Here's the 3rd line.

4th line is empty, so this is the 5th line.

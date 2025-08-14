use std::{fmt::Arguments, io::{self, BufRead, Write}, str::FromStr};

/// A macro that:
/// - optionally prints a prompt (with `print!`).
/// - reads **one line** from stdin.
/// - returns `Err(InputError::Eof)` if EOF is encountered.
/// - returns `Err(InputError::Parse(e))` if the input cannot be parsed.
/// - returns `Err(InputError::Io(e))` if an IO error occurs.
///
/// # Usage:
/// ```no_run
/// // No prompt
/// let text: String = input!().unwrap();
///
/// // With prompt
/// let name: String = input!("Enter your name: ").unwrap();
///
/// // Formatted prompt
/// let user = "Alice";
/// let age: String = input!("Enter {}'s age: ", user).unwrap();
/// ```
#[macro_export]
macro_rules! input {
    () => {{
        $crate::read_input_from(
            &mut ::std::io::stdin().lock(),
            None,
            $crate::PrintStyle::Continue,
        )
    }};
    ($($arg:tt)*) => {{
        $crate::read_input_from(
            &mut ::std::io::stdin().lock(),
            Some(format_args!($($arg)*)),
            $crate::PrintStyle::Continue
        )
    }};
}

/// A macro that:
/// - prints the prompt on its own line (with `println!`),
/// - then reads one line,
/// - returns `Err(InputError::Eof)` if EOF is encountered.
/// - returns `Err(InputError::Parse(e))` if the input cannot be parsed.
/// - returns `Err(InputError::Io(e))` if an IO error occurs.
/// - otherwise parses into `String`.
///
/// # Usage:
/// ```no_run
/// let line: String = inputln!("What's your favorite color?").unwrap();
/// ```
#[macro_export]
macro_rules! inputln {
    () => {{
        $crate::read_input_from(
          &mut ::std::io::stdin().lock(), 
          None, 
          $crate::PrintStyle::NewLine
        )
    }};
    ($($arg:tt)*) => {{
        $crate::read_input_from(
          &mut ::std::io::stdin().lock(), 
          None, 
          $crate::PrintStyle::NewLine
        )
    }};
}

/// A single function that:
/// 1. Optionally prints a prompt (and flushes).
/// 2. Reads one line from the provided `BufRead`.
/// 3. Returns `Err(InputError::Eof)` if EOF is reached.
/// 4. Parses into type `T`, returning `Err(InputError::Parse)` on failure.
/// 5. Returns `Err(InputError::Io)` on I/O failure.
pub fn read_input_from<R, T>(
    reader: &mut R,
    prompt: Option<Arguments<'_>>,
    print_style: PrintStyle,
) -> Result<T, InputError<T::Err>>
where
    R: BufRead,
    T: FromStr,
    T::Err: std::fmt::Display + std::fmt::Debug,
{
    if let Some(prompt_args) = prompt {
        match print_style {
            PrintStyle::Continue => {
                // Use print! for no newline
                print!("{}", prompt_args);
            }
            PrintStyle::NewLine => {
                // Use println! for adding a newline
                println!("{}", prompt_args);
            }
        }
        // Always flush so the user sees the prompt immediately
        io::stdout().flush().map_err(InputError::Io)?;
    }

    let mut input = String::new();
    let bytes_read = reader.read_line(&mut input).map_err(InputError::Io)?;
    
    // If 0, that's EOF — return Eof error
    if bytes_read == 0 {
        return Err(InputError::Eof);
    }

    let trimmed = input.trim_end_matches(['\r', '\n'].as_ref());
    trimmed.parse::<T>().map_err(InputError::Parse)
}

/// A unified error type indicating either an I/O error, a parse error, or EOF.
#[derive(Debug)]
pub enum InputError<E> {
    /// An I/O error occurred (e.g., closed stdin).
    Io(io::Error),
    /// Failed to parse the input into the desired type.
    Parse(E),
    /// EOF encountered (read_line returned 0).
    Eof,
}

/// Defines how the prompt should be printed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrintStyle {
    /// Print the prompt without a trailing newline (uses `print!`).
    Continue,
    /// Print the prompt with a trailing newline (uses `println!`).
    NewLine,
}

impl<E: std::fmt::Display + std::fmt::Debug> std::fmt::Display for InputError<E> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputError::Io(e) => write!(f, "I/O error: {}", e),
            InputError::Parse(e) => write!(f, "Parse error: {}", e),
            InputError::Eof => write!(f, "EOF encountered"),
        }
    }
}

impl<E: std::fmt::Display + std::fmt::Debug> std::error::Error for InputError<E> {}
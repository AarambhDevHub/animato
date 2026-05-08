//! SVG `d` attribute parsing.

use crate::poly::PathCommand;
use alloc::vec::Vec;
use core::fmt;

/// Error returned by [`SvgPathParser::try_parse`].
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum SvgPathError {
    /// The input ended while a command still required more values.
    UnexpectedEnd {
        /// Byte offset where parsing stopped.
        at: usize,
    },
    /// A numeric value was expected.
    ExpectedNumber {
        /// Byte offset where the number was expected.
        at: usize,
    },
    /// A numeric value was present but could not be parsed as `f32`.
    InvalidNumber {
        /// Byte offset where the invalid number started.
        at: usize,
    },
    /// An SVG arc flag was expected.
    ExpectedFlag {
        /// Byte offset where the flag was expected.
        at: usize,
    },
    /// The input used a command that v0.4.0 does not support.
    UnsupportedCommand {
        /// Unsupported command character.
        command: char,
        /// Byte offset of the command.
        at: usize,
    },
    /// The path data started drawing before an initial moveto command.
    MissingMoveTo {
        /// Byte offset where drawing data appeared.
        at: usize,
    },
}

impl fmt::Display for SvgPathError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            SvgPathError::UnexpectedEnd { at } => write!(f, "unexpected end of SVG path at {at}"),
            SvgPathError::ExpectedNumber { at } => {
                write!(f, "expected SVG path number at {at}")
            }
            SvgPathError::InvalidNumber { at } => write!(f, "invalid SVG path number at {at}"),
            SvgPathError::ExpectedFlag { at } => write!(f, "expected SVG arc flag at {at}"),
            SvgPathError::UnsupportedCommand { command, at } => {
                write!(f, "unsupported SVG path command '{command}' at {at}")
            }
            SvgPathError::MissingMoveTo { at } => {
                write!(f, "SVG path must start with moveto before drawing at {at}")
            }
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for SvgPathError {}

/// Parser for SVG path `d` attributes.
#[derive(Clone, Copy, Debug, Default)]
pub struct SvgPathParser;

impl SvgPathParser {
    /// Parse an SVG `d` attribute.
    ///
    /// This permissive convenience method returns an empty vector on invalid
    /// input. Use [`try_parse`](Self::try_parse) when callers need details.
    pub fn parse(d: &str) -> Vec<PathCommand> {
        Self::try_parse(d).unwrap_or_default()
    }

    /// Parse an SVG `d` attribute with error reporting.
    pub fn try_parse(d: &str) -> Result<Vec<PathCommand>, SvgPathError> {
        Parser::new(d).parse()
    }
}

struct Parser<'a> {
    input: &'a str,
    pos: usize,
    current: [f32; 2],
    subpath_start: [f32; 2],
    has_current: bool,
    last_command: Option<u8>,
    commands: Vec<PathCommand>,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Self {
            input,
            pos: 0,
            current: [0.0, 0.0],
            subpath_start: [0.0, 0.0],
            has_current: false,
            last_command: None,
            commands: Vec::new(),
        }
    }

    fn parse(mut self) -> Result<Vec<PathCommand>, SvgPathError> {
        while !self.is_eof_after_separators() {
            let command = if let Some(byte) = self.peek_byte() {
                if byte.is_ascii_alphabetic() {
                    self.pos += 1;
                    self.last_command = Some(byte);
                    byte
                } else if let Some(command) = self.last_command {
                    command
                } else {
                    return Err(SvgPathError::MissingMoveTo { at: self.pos });
                }
            } else {
                break;
            };

            self.parse_command(command)?;
        }

        Ok(self.commands)
    }

    fn parse_command(&mut self, command: u8) -> Result<(), SvgPathError> {
        let relative = command.is_ascii_lowercase();
        match command.to_ascii_uppercase() {
            b'M' => self.parse_move(relative),
            b'L' => self.parse_line(relative),
            b'H' => self.parse_horizontal(relative),
            b'V' => self.parse_vertical(relative),
            b'C' => self.parse_cubic(relative),
            b'Q' => self.parse_quad(relative),
            b'A' => self.parse_arc(relative),
            b'Z' => {
                self.commands.push(PathCommand::ClosePath);
                self.current = self.subpath_start;
                self.has_current = true;
                self.last_command = None;
                Ok(())
            }
            _ => Err(SvgPathError::UnsupportedCommand {
                command: command as char,
                at: self.pos.saturating_sub(1),
            }),
        }
    }

    fn parse_move(&mut self, relative: bool) -> Result<(), SvgPathError> {
        let mut first = true;
        self.require_number()?;
        while self.has_number() {
            let point = self.parse_point(relative)?;
            if first {
                self.commands.push(PathCommand::MoveTo(point));
                self.subpath_start = point;
                first = false;
            } else {
                self.commands.push(PathCommand::LineTo(point));
            }
            self.current = point;
            self.has_current = true;
        }
        Ok(())
    }

    fn parse_line(&mut self, relative: bool) -> Result<(), SvgPathError> {
        self.ensure_current()?;
        self.require_number()?;
        while self.has_number() {
            let point = self.parse_point(relative)?;
            self.commands.push(PathCommand::LineTo(point));
            self.current = point;
        }
        Ok(())
    }

    fn parse_horizontal(&mut self, relative: bool) -> Result<(), SvgPathError> {
        self.ensure_current()?;
        self.require_number()?;
        while self.has_number() {
            let x = self.parse_number()?;
            let point = if relative {
                [self.current[0] + x, self.current[1]]
            } else {
                [x, self.current[1]]
            };
            self.commands.push(PathCommand::LineTo(point));
            self.current = point;
        }
        Ok(())
    }

    fn parse_vertical(&mut self, relative: bool) -> Result<(), SvgPathError> {
        self.ensure_current()?;
        self.require_number()?;
        while self.has_number() {
            let y = self.parse_number()?;
            let point = if relative {
                [self.current[0], self.current[1] + y]
            } else {
                [self.current[0], y]
            };
            self.commands.push(PathCommand::LineTo(point));
            self.current = point;
        }
        Ok(())
    }

    fn parse_cubic(&mut self, relative: bool) -> Result<(), SvgPathError> {
        self.ensure_current()?;
        self.require_number()?;
        while self.has_number() {
            let control1 = self.parse_point(relative)?;
            let control2 = self.parse_point(relative)?;
            let end = self.parse_point(relative)?;
            self.commands.push(PathCommand::CubicTo {
                control1,
                control2,
                end,
            });
            self.current = end;
        }
        Ok(())
    }

    fn parse_quad(&mut self, relative: bool) -> Result<(), SvgPathError> {
        self.ensure_current()?;
        self.require_number()?;
        while self.has_number() {
            let control = self.parse_point(relative)?;
            let end = self.parse_point(relative)?;
            self.commands.push(PathCommand::QuadTo { control, end });
            self.current = end;
        }
        Ok(())
    }

    fn parse_arc(&mut self, relative: bool) -> Result<(), SvgPathError> {
        self.ensure_current()?;
        self.require_number()?;
        while self.has_number() {
            let rx = self.parse_number()?;
            let ry = self.parse_number()?;
            let x_axis_rotation = self.parse_number()?;
            let large_arc = self.parse_flag()?;
            let sweep = self.parse_flag()?;
            let end = self.parse_point(relative)?;
            self.commands.push(PathCommand::ArcTo {
                radii: [rx, ry],
                x_axis_rotation,
                large_arc,
                sweep,
                end,
            });
            self.current = end;
        }
        Ok(())
    }

    fn parse_point(&mut self, relative: bool) -> Result<[f32; 2], SvgPathError> {
        let x = self.parse_number()?;
        let y = self.parse_number()?;
        if relative && self.has_current {
            Ok([self.current[0] + x, self.current[1] + y])
        } else {
            Ok([x, y])
        }
    }

    fn parse_number(&mut self) -> Result<f32, SvgPathError> {
        self.skip_separators();
        let start = self.pos;
        let bytes = self.input.as_bytes();

        if self.pos >= bytes.len() {
            return Err(SvgPathError::UnexpectedEnd { at: self.pos });
        }

        if matches!(bytes[self.pos], b'+' | b'-') {
            self.pos += 1;
        }

        let mut has_digit = false;
        while self.pos < bytes.len() && bytes[self.pos].is_ascii_digit() {
            self.pos += 1;
            has_digit = true;
        }

        if self.pos < bytes.len() && bytes[self.pos] == b'.' {
            self.pos += 1;
            while self.pos < bytes.len() && bytes[self.pos].is_ascii_digit() {
                self.pos += 1;
                has_digit = true;
            }
        }

        if !has_digit {
            return Err(SvgPathError::ExpectedNumber { at: start });
        }

        if self.pos < bytes.len() && matches!(bytes[self.pos], b'e' | b'E') {
            let exp_pos = self.pos;
            self.pos += 1;
            if self.pos < bytes.len() && matches!(bytes[self.pos], b'+' | b'-') {
                self.pos += 1;
            }
            let exp_start = self.pos;
            while self.pos < bytes.len() && bytes[self.pos].is_ascii_digit() {
                self.pos += 1;
            }
            if exp_start == self.pos {
                return Err(SvgPathError::InvalidNumber { at: exp_pos });
            }
        }

        self.input[start..self.pos]
            .parse::<f32>()
            .map_err(|_| SvgPathError::InvalidNumber { at: start })
    }

    fn parse_flag(&mut self) -> Result<bool, SvgPathError> {
        self.skip_separators();
        let at = self.pos;
        match self.peek_byte() {
            Some(b'0') => {
                self.pos += 1;
                Ok(false)
            }
            Some(b'1') => {
                self.pos += 1;
                Ok(true)
            }
            Some(_) => Err(SvgPathError::ExpectedFlag { at }),
            None => Err(SvgPathError::UnexpectedEnd { at }),
        }
    }

    fn require_number(&mut self) -> Result<(), SvgPathError> {
        if self.has_number() {
            Ok(())
        } else {
            Err(SvgPathError::ExpectedNumber { at: self.pos })
        }
    }

    fn ensure_current(&self) -> Result<(), SvgPathError> {
        if self.has_current {
            Ok(())
        } else {
            Err(SvgPathError::MissingMoveTo { at: self.pos })
        }
    }

    fn has_number(&self) -> bool {
        let mut pos = self.pos;
        let bytes = self.input.as_bytes();
        while pos < bytes.len() && is_separator(bytes[pos]) {
            pos += 1;
        }
        pos < bytes.len() && is_number_start(bytes[pos])
    }

    fn skip_separators(&mut self) {
        while let Some(byte) = self.peek_byte() {
            if is_separator(byte) {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn is_eof_after_separators(&mut self) -> bool {
        self.skip_separators();
        self.pos >= self.input.len()
    }

    fn peek_byte(&self) -> Option<u8> {
        self.input.as_bytes().get(self.pos).copied()
    }
}

fn is_separator(byte: u8) -> bool {
    byte == b',' || byte.is_ascii_whitespace()
}

fn is_number_start(byte: u8) -> bool {
    byte.is_ascii_digit() || matches!(byte, b'+' | b'-' | b'.')
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_basic_absolute_commands() {
        let commands = SvgPathParser::try_parse("M 0 0 L 10 0 H 20 V 30 Z").unwrap();
        assert_eq!(
            commands,
            alloc::vec![
                PathCommand::MoveTo([0.0, 0.0]),
                PathCommand::LineTo([10.0, 0.0]),
                PathCommand::LineTo([20.0, 0.0]),
                PathCommand::LineTo([20.0, 30.0]),
                PathCommand::ClosePath,
            ]
        );
    }

    #[test]
    fn parses_relative_commands() {
        let commands = SvgPathParser::try_parse("m10 10 l5 0 h5 v-5 q5 0 5 5").unwrap();
        assert_eq!(commands[0], PathCommand::MoveTo([10.0, 10.0]));
        assert_eq!(commands[1], PathCommand::LineTo([15.0, 10.0]));
        assert_eq!(commands[2], PathCommand::LineTo([20.0, 10.0]));
        assert_eq!(commands[3], PathCommand::LineTo([20.0, 5.0]));
        assert_eq!(
            commands[4],
            PathCommand::QuadTo {
                control: [25.0, 5.0],
                end: [25.0, 10.0],
            }
        );
    }

    #[test]
    fn parses_repeated_moveto_as_line_to() {
        let commands = SvgPathParser::try_parse("M0 0 10 10 20 0").unwrap();
        assert_eq!(commands.len(), 3);
        assert_eq!(commands[1], PathCommand::LineTo([10.0, 10.0]));
        assert_eq!(commands[2], PathCommand::LineTo([20.0, 0.0]));
    }

    #[test]
    fn parses_cubic_and_arc() {
        let commands = SvgPathParser::try_parse("M0 0 C10 0 10 20 20 20 A5 5 0 01 30 20").unwrap();
        assert_eq!(commands.len(), 3);
        assert_eq!(
            commands[2],
            PathCommand::ArcTo {
                radii: [5.0, 5.0],
                x_axis_rotation: 0.0,
                large_arc: false,
                sweep: true,
                end: [30.0, 20.0],
            }
        );
    }

    #[test]
    fn parses_adjacent_negative_numbers() {
        let commands = SvgPathParser::try_parse("M10-20L30-40").unwrap();
        assert_eq!(commands[0], PathCommand::MoveTo([10.0, -20.0]));
        assert_eq!(commands[1], PathCommand::LineTo([30.0, -40.0]));
    }

    #[test]
    fn unsupported_command_is_error() {
        let err = SvgPathParser::try_parse("M0 0 S 1 2 3 4").unwrap_err();
        assert!(matches!(err, SvgPathError::UnsupportedCommand { .. }));
        assert!(SvgPathParser::parse("M0 0 S 1 2 3 4").is_empty());
    }
}

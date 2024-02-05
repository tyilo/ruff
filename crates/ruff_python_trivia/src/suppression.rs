use crate::PythonWhitespace;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub enum SuppressionKind {
    /// A `fmt: off` or `yapf: disable` comment
    Off,
    /// A `fmt: on` or `yapf: enable` comment
    On,
    /// A `fmt: skip` comment
    Skip,
}

impl SuppressionKind {
    /// Given a `slice`
    pub fn from_slice(slice: &str) -> Option<Self> {
        // Match against `# fmt: on`, `# fmt: off`, `# yapf: disable`, and `# yapf: enable`, which
        // must be on their own lines.
        let trimmed = slice.strip_prefix('#').unwrap_or(slice).trim_whitespace();
        if let Some(command) = trimmed.strip_prefix("fmt:") {
            match command.trim_whitespace_start() {
                "off" => return Some(Self::Off),
                "on" => return Some(Self::On),
                "skip" => return Some(Self::Skip),
                _ => {}
            }
        } else if let Some(command) = trimmed.strip_prefix("yapf:") {
            match command.trim_whitespace_start() {
                "disable" => return Some(Self::Off),
                "enable" => return Some(Self::On),
                _ => {}
            }
        }

        // Search for `# fmt: skip` comments, which can be interspersed with other comments (e.g.,
        // `# fmt: skip # noqa: E501`).
        for segment in slice.split('#') {
            let trimmed = segment.trim_whitespace();
            if let Some(command) = trimmed.strip_prefix("fmt:") {
                if command.trim_whitespace_start() == "skip" {
                    return Some(SuppressionKind::Skip);
                }
            }
        }

        None
    }

    /// Returns true if this comment is a `fmt: off` or `yapf: disable` own line suppression comment.
    pub fn is_suppression_on(slice: &str, position: CommentLinePosition) -> bool {
        position.is_own_line() && matches!(Self::from_slice(slice), Some(Self::Off))
    }

    /// Returns true if this comment is a `fmt: on` or `yapf: enable` own line suppression comment.
    pub fn is_suppression_off(slice: &str, position: CommentLinePosition) -> bool {
        position.is_own_line() && matches!(Self::from_slice(slice), Some(Self::On))
    }
}
/// The position of a comment in the source text.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub enum CommentLinePosition {
    /// A comment that is on the same line as the preceding token and is separated by at least one line break from the following token.
    ///
    /// # Examples
    ///
    /// ## End of line
    ///
    /// ```python
    /// a; # comment
    /// b;
    /// ```
    ///
    /// `# comment` is an end of line comments because it is separated by at least one line break from the following token `b`.
    /// Comments that not only end, but also start on a new line are [`OwnLine`](CommentLinePosition::OwnLine) comments.
    EndOfLine,

    /// A Comment that is separated by at least one line break from the preceding token.
    ///
    /// # Examples
    ///
    /// ```python
    /// a;
    /// # comment
    /// b;
    /// ```
    ///
    /// `# comment` line comments because they are separated by one line break from the preceding token `a`.
    OwnLine,
}

impl CommentLinePosition {
    pub const fn is_own_line(self) -> bool {
        matches!(self, Self::OwnLine)
    }

    pub const fn is_end_of_line(self) -> bool {
        matches!(self, Self::EndOfLine)
    }
}

/*********************** GNU General Public License 3.0 ***********************\
|                                                                              |
|  Copyright (C) 2026 Kevin Matthes                                            |
|                                                                              |
|  This program is free software: you can redistribute it and/or modify        |
|  it under the terms of the GNU General Public License as published by        |
|  the Free Software Foundation, either version 3 of the License, or           |
|  (at your option) any later version.                                         |
|                                                                              |
|  This program is distributed in the hope that it will be useful,             |
|  but WITHOUT ANY WARRANTY; without even the implied warranty of              |
|  MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the               |
|  GNU General Public License for more details.                                |
|                                                                              |
|  You should have received a copy of the GNU General Public License           |
|  along with this program.  If not, see <https://www.gnu.org/licenses/>.      |
|                                                                              |
\******************************************************************************/

/// Details on a rule violation.
pub struct Finding {
    file: std::path::PathBuf,
    line: usize,
    message: String,
}

impl Finding {
    /// Retrieve the affected file.
    #[must_use]
    pub const fn file(&self) -> &std::path::PathBuf {
        &self.file
    }

    /// Retrieve the affected line.
    #[must_use]
    pub const fn line(&self) -> usize {
        self.line
    }

    /// Retrieve the explanation for this finding.
    #[must_use]
    pub const fn message(&self) -> &str {
        self.message.as_str()
    }

    /// Create a new instance.
    #[must_use]
    pub fn new(file: &str, line: usize, message: &str) -> Self {
        Self {
            file: file.into(),
            line,
            message: message.into(),
        }
    }
}

impl std::fmt::Display for Finding {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>,
    ) -> Result<(), std::fmt::Error> {
        write!(f, "{}:{}\t{}", self.file.display(), self.line, self.message)
    }
}

/******************************************************************************/

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

//! Tests for the struct collection.

mod finding {
    fn finding() -> coding_conventions::Finding {
        coding_conventions::Finding::new("path", 42, "message")
    }

    mod getters {
        #[test]
        fn file() {
            assert_eq!(
                crate::finding::finding().file(),
                &std::path::PathBuf::from("path")
            );
        }

        #[test]
        fn line() {
            assert_eq!(crate::finding::finding().line(), 42);
        }

        #[test]
        fn message() {
            assert_eq!(crate::finding::finding().message(), "message");
        }
    }

    mod traits {
        #[test]
        fn std_fmt_display() {
            assert_eq!(crate::finding::finding().into(), "path:42 message");
        }
    }
}

/******************************************************************************/

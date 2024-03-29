//! PGN data structures for storing PGN games.

mod pgn_vistor;
mod san_plus_wrapper;
pub use san_plus_wrapper::SanPlusWrapper;

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash)]
/// PGN headers struct that holds the headers of a PGN game.
/// Only stores the data required for PGN 'reduced export format'.
pub struct PgnHeaders {
    pub event: String,
    pub site: String,
    pub date: String,
    pub round: String,
    pub white: String,
    pub black: String,
    pub result: String,
}

impl PgnHeaders {
    /// Creates a new empty PgnHeaders struct
    pub fn new() -> PgnHeaders {
        PgnHeaders {
            event: String::new(),
            site: String::new(),
            date: String::new(),
            round: String::new(),
            white: String::new(),
            black: String::new(),
            result: String::new(),
        }
    }

    // Checks if the PgnHeaders struct is empty
    pub fn is_empty(&self) -> bool {
        self.event.is_empty()
            && self.site.is_empty()
            && self.date.is_empty()
            && self.round.is_empty()
            && self.white.is_empty()
            && self.black.is_empty()
            && self.result.is_empty()
    }
}

impl Default for PgnHeaders {
    /// Creates a new empty PgnHeaders struct
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize, Eq, PartialEq, Hash)]
/// PGN data struct that holds the headers and moves of a PGN game.
/// Only stores the data required for PGN 'reduced export format'.
///
/// A PGN game is in 'reduced export format' if abide by the following rules:
/// 1) There are no comments.
/// 2) Only the 7 mandatory tags are used (Event, Site, Date, Round, White, Black, Result).
/// 3) There are no recursive annotations.
/// 4) There are no numeric annotation glyphs.
///
/// # Example
///
/// ```
/// use cgn::pgn_data::PgnData;
/// use std::str::FromStr;
///
/// const PGN_STR_EXAMPLE: &str = r#"[Event "Titled Tuesday Blitz January 03 Early 2023"]
/// [Site ""]
/// [Date "2023.01.03"]
/// [Round "?"]
/// [White "Magnus Carlsen"]
/// [Black "Samvel Ter-Sahakyan"]
/// [Result "1-0"]
///
/// 1. a4 Nf6 2. d4 d5 3. Nf3 Bf5 4. Nh4 Be4 5. f3 Bg6 6. Nc3 c5 7. e4 cxd4 8. Nxg6
/// hxg6 9. Qxd4 Nc6 10. Qf2 d4 11. Nd1 e5 12. Bc4 Rc8 13. Qe2 Bb4+ 14. Kf1 Na5 15.
/// Bd3 O-O 16. Nf2 Qb6 17. h4 Nh5 18. Rh3 Qf6 19. g4 Nf4 20. Bxf4 Qxf4 21. h5 g5
/// 22. Rd1 a6 23. Kg2 Rc7 24. Rhh1 Rfc8 25. Nh3 Qf6 26. Ra1 Nc6 27. Rhc1 Bd6 28.
/// Qd2 Bb4 29. c3 Be7 30. Nf2 dxc3 31. bxc3 Nd8 32. Bb1 Ne6 33. Nh3 Bc5 34. Ba2 Rd8
/// 35. Qe2 Nf4+ 36. Nxf4 gxf4 37. Kh3 g6 38. Rd1 Rcd7 39. Rxd7 Rxd7 40. Rd1 Bf2 41.
/// Bxf7+ Kf8 42. Qxf2 Rxd1 43. Bxg6 Qd6 44. g5 Qd3 45. Qc5+ Qd6 46. Qc8+ Kg7 47.
/// Qxb7+ Kf8 48. Qf7# 1-0"#;
///
/// // Parse the PGN string into a PgnData struct (uses pgn_data::PgnVisitor under the hood)
/// let pgn_data = PgnData::from_str(PGN_STR_EXAMPLE).unwrap();
///
/// // Reading headers
/// assert_eq!("Titled Tuesday Blitz January 03 Early 2023", pgn_data.headers.event);
/// assert_eq!("Magnus Carlsen", pgn_data.headers.white);
///
/// // Reading moves
/// assert_eq!("a4", pgn_data.moves[0].0.to_string());
/// assert_eq!("Nf6", pgn_data.moves[1].0.to_string());
///
/// // PgnData struct can be converted back to a PGN string
/// // Note: The to_string() method will always return a 'reduced export format' PGN string,
/// // even if the original PGN string was not in 'reduced export format'.
/// assert_eq!(PGN_STR_EXAMPLE, pgn_data.to_string());
/// ```
pub struct PgnData {
    pub headers: PgnHeaders,
    pub moves: Vec<san_plus_wrapper::SanPlusWrapper>,
}

impl PgnData {
    /// Creates a new empty PgnData struct.
    pub fn new() -> PgnData {
        PgnData {
            headers: PgnHeaders::new(),
            moves: vec![],
        }
    }

    /// Clear headers from the PgnData struct.
    pub fn clear_headers(&mut self) {
        self.headers = PgnHeaders::new();
    }

    /// Checks if the PgnData struct is empty.
    pub fn is_empty(&self) -> bool {
        self.headers.is_empty() && self.moves.is_empty()
    }
}

impl Default for PgnData {
    /// Creates a new empty PgnData struct.
    fn default() -> Self {
        Self::new()
    }
}

impl std::str::FromStr for PgnData {
    type Err = std::io::Error;

    /// Parses a PGN string into a PgnData struct.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut visitor = pgn_vistor::PgnVisitor::new();
        pgn_reader::BufferedReader::new_cursor(&s)
            .read_game(&mut visitor)?
            .ok_or(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Failed to read PGN game from string",
            ))
    }
}

impl std::fmt::Display for PgnData {
    /// Formats the PgnData struct into a PGN string.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Create a string buffer and write the headers to it
        let mut s = String::new();
        s.push_str(&format!("[Event \"{}\"]\n", self.headers.event));
        s.push_str(&format!("[Site \"{}\"]\n", self.headers.site));
        s.push_str(&format!("[Date \"{}\"]\n", self.headers.date));
        s.push_str(&format!("[Round \"{}\"]\n", self.headers.round));
        s.push_str(&format!("[White \"{}\"]\n", self.headers.white));
        s.push_str(&format!("[Black \"{}\"]\n", self.headers.black));
        s.push_str(&format!("[Result \"{}\"]\n", self.headers.result));

        // Write the moves to the string buffer
        s.push('\n');
        for (i, san_plus) in self.moves.iter().enumerate() {
            if i % 2 == 0 {
                s.push_str(&format!("{}. ", i / 2 + 1));
            }
            s.push_str(&san_plus.0.to_string());
            s.push(' ');
        }

        // Write the result to the string buffer
        s.push_str(self.headers.result.as_str());

        //  Wrap the string buffer to 80 characters and write it to the formatter
        write!(f, "{}", textwrap::fill(&s, 80))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;

    /// Example PGN string.
    const PGN_STR_EXAMPLE: &str = r#"[Event "Titled Tuesday Blitz January 03 Early 2023"]
[Site ""]
[Date "2023.01.03"]
[Round "?"]
[White "Magnus Carlsen"]
[Black "Samvel Ter-Sahakyan"]
[Result "1-0"]

1. a4 Nf6 2. d4 d5 3. Nf3 Bf5 4. Nh4 Be4 5. f3 Bg6 6. Nc3 c5 7. e4 cxd4 8. Nxg6
hxg6 9. Qxd4 Nc6 10. Qf2 d4 11. Nd1 e5 12. Bc4 Rc8 13. Qe2 Bb4+ 14. Kf1 Na5 15.
Bd3 O-O 16. Nf2 Qb6 17. h4 Nh5 18. Rh3 Qf6 19. g4 Nf4 20. Bxf4 Qxf4 21. h5 g5
22. Rd1 a6 23. Kg2 Rc7 24. Rhh1 Rfc8 25. Nh3 Qf6 26. Ra1 Nc6 27. Rhc1 Bd6 28.
Qd2 Bb4 29. c3 Be7 30. Nf2 dxc3 31. bxc3 Nd8 32. Bb1 Ne6 33. Nh3 Bc5 34. Ba2 Rd8
35. Qe2 Nf4+ 36. Nxf4 gxf4 37. Kh3 g6 38. Rd1 Rcd7 39. Rxd7 Rxd7 40. Rd1 Bf2 41.
Bxf7+ Kf8 42. Qxf2 Rxd1 43. Bxg6 Qd6 44. g5 Qd3 45. Qc5+ Qd6 46. Qc8+ Kg7 47.
Qxb7+ Kf8 48. Qf7# 1-0"#;

    /// Example PGN string with additional header.
    const PGN_STR_EXAMPLE_EXTRA_HEADER: &str = r#"[Event "Titled Tuesday Blitz January 03 Early 2023"]
[Site ""]
[Date "2023.01.03"]
[Round "?"]
[White "Magnus Carlsen"]
[Black "Samvel Ter-Sahakyan"]
[Result "1-0"]
[Extra "FOOBAR"]

1. a4 Nf6 2. d4 d5 3. Nf3 Bf5 4. Nh4 Be4 5. f3 Bg6 6. Nc3 c5 7. e4 cxd4 8. Nxg6
hxg6 9. Qxd4 Nc6 10. Qf2 d4 11. Nd1 e5 12. Bc4 Rc8 13. Qe2 Bb4+ 14. Kf1 Na5 15.
Bd3 O-O 16. Nf2 Qb6 17. h4 Nh5 18. Rh3 Qf6 19. g4 Nf4 20. Bxf4 Qxf4 21. h5 g5
22. Rd1 a6 23. Kg2 Rc7 24. Rhh1 Rfc8 25. Nh3 Qf6 26. Ra1 Nc6 27. Rhc1 Bd6 28.
Qd2 Bb4 29. c3 Be7 30. Nf2 dxc3 31. bxc3 Nd8 32. Bb1 Ne6 33. Nh3 Bc5 34. Ba2 Rd8
35. Qe2 Nf4+ 36. Nxf4 gxf4 37. Kh3 g6 38. Rd1 Rcd7 39. Rxd7 Rxd7 40. Rd1 Bf2 41.
Bxf7+ Kf8 42. Qxf2 Rxd1 43. Bxg6 Qd6 44. g5 Qd3 45. Qc5+ Qd6 46. Qc8+ Kg7 47.
Qxb7+ Kf8 48. Qf7# 1-0"#;

    #[test]
    /// Tests if the PgnData struct can be parsed and then converted back to a string.
    fn parsed_eq_original() {
        let pgn_str = PGN_STR_EXAMPLE;
        let pgn_data = PgnData::from_str(pgn_str).unwrap();
        assert_eq!(pgn_str, pgn_data.to_string());
    }

    #[test]
    /// Tests if we can clear the headers from a PgnData struct.
    fn can_clear_headers() {
        let pgn_str = PGN_STR_EXAMPLE;
        let mut pgn_data = PgnData::from_str(pgn_str).unwrap();
        pgn_data.clear_headers();
        assert_eq!(pgn_data.headers.event, "");
        assert_eq!(pgn_data.headers.site, "");
        assert_eq!(pgn_data.headers.date, "");
        assert_eq!(pgn_data.headers.round, "");
        assert_eq!(pgn_data.headers.white, "");
        assert_eq!(pgn_data.headers.black, "");
        assert_eq!(pgn_data.headers.result, "");
    }

    #[test]
    /// Tests if additional headers are ignored when parsing a PGN string.
    fn ignores_additional_headers() {
        let pgn_str = PGN_STR_EXAMPLE_EXTRA_HEADER;
        let pgn_data = PgnData::from_str(pgn_str).unwrap();
        assert!(!pgn_data.to_string().contains("FOOBAR"));
    }

    #[test]
    /// Tests if the default PgnHeaders struct is empty.
    fn default_headers() {
        let pgn_data = PgnHeaders::default();
        assert_eq!(pgn_data.event, "");
        assert_eq!(pgn_data.site, "");
        assert_eq!(pgn_data.date, "");
        assert_eq!(pgn_data.round, "");
        assert_eq!(pgn_data.white, "");
        assert_eq!(pgn_data.black, "");
        assert_eq!(pgn_data.result, "");
    }

    #[test]
    /// Tests if the default PgnData struct is empty.
    fn default_pgn_data() {
        let pgn_data = PgnData::default();
        assert!(pgn_data.moves.is_empty());
        assert!(pgn_data.headers.is_empty());
    }
}

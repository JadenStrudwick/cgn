use crate::pgn_data::PgnData;
use wasm_bindgen::prelude::*;

/// This strategy uses the bincode crate to serialize the data and
/// then compresses it using the flate2 crate's ZlibEncoder at the
/// best compression level.

/// Compresses the PGN data using bincode and ZlibEncoder at the maximum compression level.
pub fn compress_pgn_data(pgn_data: &PgnData) -> Vec<u8> {
    // create a buffer to store the compressed data and a ZlibEncoder
    let mut compressed_data = Vec::new();
    let mut encoder =
        flate2::write::ZlibEncoder::new(&mut compressed_data, flate2::Compression::best());

    // serialize the data into the encoder and finish the compression
    bincode::serialize_into(&mut encoder, pgn_data).expect("Failed to serialize PGN data");
    encoder.finish().expect("Failed to compress PGN data");
    compressed_data
}

/// Decompresses the PGN data using bincode and ZlibDecoder.
pub fn decompress_pgn_data(compressed_data: &[u8]) -> PgnData {
    let mut decoder = flate2::read::ZlibDecoder::new(compressed_data);
    bincode::deserialize_from(&mut decoder).expect("Failed to deserialize PGN data")
}

/// Compresses the PGN string using bincode and ZlibEncoder at the maximum compression level.
/// # Arguments
/// * `pgn_str` - The PGN string.
/// # Returns
/// The compressed PGN data.
#[wasm_bindgen]
pub fn compress_pgn_str(pgn_str: &str) -> Vec<u8> {
    let pgn_data = PgnData::from_str(pgn_str);
    compress_pgn_data(&pgn_data)
}

/// Decompresses the PGN string using bincode and ZlibDecoder.
/// # Arguments
/// * `compressed_data` - The compressed PGN string.
/// # Returns
/// The decompressed PGN string.
#[wasm_bindgen]
pub fn decompress_pgn_str(compressed_data: &[u8]) -> String {
    let pgn_data = decompress_pgn_data(compressed_data);
    pgn_data.to_string()
}

#[cfg(test)]
mod tests {
    /// Example PGN string.
    pub const PGN_STR_EXAMPLE: &str = r#"[Event "Titled Tuesday Blitz January 03 Early 2023"]
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


    #[test]
    /// Test if the bincode Zlib compression is correct for PGN structs.
    fn bincode_zlib_pgn_data() {
        let pgn_str = PGN_STR_EXAMPLE;
        let pgn_data = super::PgnData::from_str(pgn_str);
        let compressed_data = super::compress_pgn_data(&pgn_data);
        let decompressed_data = super::decompress_pgn_data(&compressed_data);
        let decompressed_pgn_str = decompressed_data.to_string();
        assert_eq!(pgn_str, decompressed_pgn_str);
    }

    #[test]
    /// Test if the bincode Zlib compression is correct for PGN strings.
    fn bincode_zlib_pgn_str() {
        let pgn_str = PGN_STR_EXAMPLE;
        let compressed_data = super::compress_pgn_str(pgn_str);
        let decompressed_pgn_str = super::decompress_pgn_str(&compressed_data);
        assert_eq!(pgn_str, decompressed_pgn_str);
    } 
}

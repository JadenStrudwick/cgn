# CGN (Compressed Game Notation)

This library provides a set of compression algorithms for compressing PGN (Portable Game Notation) data. The library also provides a set of data structures for storing PGN data in a reduced format.

## Compression Algorithms (High to Low Compression Ratios --- Low to High Speed)
1) `opening-huffman` - A Huffman encoding algorithm that uses the huffman-encoding crate to compress the PGN data, but with an additional optimization for compressing common opening moves.
2) `dynamic-huffman` - A Huffman encoding algorithm that uses the huffman-encoding crate to compress the PGN data, but with a huffman tree that is updated dynamically as the data is compressed.
3) `huffman` - A Huffman encoding algorithm that uses a huffman-encoding crate to compress the PGN data.
4) `bincode` - A simple binary encoding algorithm that uses the bincode crate to serialize the PGN data into a binary format.

## Data Structures
* `PgnData` - A struct that holds the headers and moves of a PGN game. Only stores the data required for PGN 'reduced export format'.
* `PgnHeaders` - A struct that holds the headers of a PGN game. Only stores the data required for PGN 'reduced export format'.
* `SanPlusWrapper` - A wrapper struct that holds a `SanPlus` struct. This is used to implement the serialize and deserialize traits on the `SanPlus` struct.

## Example Usage

```rust
use cgn::pgn_data::PgnData;
use cgn::compression::dynamic_huffman::{compress_pgn_data, decompress_pgn_data, dynamic_huffman_compress_pgn_str, dynamic_huffman_decompress_pgn_str};
use bit_vec::BitVec;
use std::str::FromStr;

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

// PgnData Struct to BitVec Example
let pgn_data: PgnData = PgnData::from_str(PGN_STR_EXAMPLE).unwrap();
let compressed_bitvec: BitVec = compress_pgn_data(&pgn_data).unwrap();
let decompressed_pgn_data: PgnData = decompress_pgn_data(&compressed_bitvec).unwrap();
assert_eq!(pgn_data, decompressed_pgn_data);

// Pgn String to Bytes Example (functions are suitable for WASM compilation)
let compressed_bytes: Vec<u8> = dynamic_huffman_compress_pgn_str(PGN_STR_EXAMPLE);
let decompressed_pgn_str: String = dynamic_huffman_decompress_pgn_str(&compressed_bytes);
assert_eq!(PGN_STR_EXAMPLE, decompressed_pgn_str);
```
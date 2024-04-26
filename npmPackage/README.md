# CGN (Compressed Game Notation)

This library provides a set of compression algorithms for compressing PGN (Portable Game Notation) data.

## Compression Algorithms (High to Low Compression Ratios --- Low to High Speed)
1) `opening-huffman` - A Huffman encoding algorithm that uses the huffman-encoding crate to compress the PGN data, but with an additional optimization for compressing common opening moves.
2) `dynamic-huffman` - A Huffman encoding algorithm that uses the huffman-encoding crate to compress the PGN data, but with a huffman tree that is updated dynamically as the data is compressed.
3) `huffman` - A Huffman encoding algorithm that uses a huffman-encoding crate to compress the PGN data.
4) `bincode` - A simple binary encoding algorithm that uses the bincode crate to serialize the PGN data into a binary format.

/* tslint:disable */
/* eslint-disable */
/**
* Compresses a PGN string into a vector of bytes
* @param {string} pgn_str
* @returns {Uint8Array}
*/
export function opening_huffman_compress_pgn_str(pgn_str: string): Uint8Array;
/**
* Decompresses a vector of bytes into a PGN string
* @param {Uint8Array} compressed_data
* @returns {string}
*/
export function opening_huffman_decompress_pgn_str(compressed_data: Uint8Array): string;
/**
* Compresses a PGN string into a vector of bytes
* @param {string} pgn_str
* @returns {Uint8Array}
*/
export function dynamic_huffman_compress_pgn_str(pgn_str: string): Uint8Array;
/**
* Decompresses a vector of bytes into a PGN string
* @param {Uint8Array} compressed_data
* @returns {string}
*/
export function dynamic_huffman_decompress_pgn_str(compressed_data: Uint8Array): string;
/**
* Compresses a PGN string into a vector of bytes
* @param {string} pgn_str
* @returns {Uint8Array}
*/
export function bincode_compress_pgn_str(pgn_str: string): Uint8Array;
/**
* Decompresses a vector of bytes into a PGN string
* @param {Uint8Array} compressed_data
* @returns {string}
*/
export function bincode_decompress_pgn_str(compressed_data: Uint8Array): string;
/**
* Compresses a PGN string into a vector of bytes
* @param {string} pgn_str
* @returns {Uint8Array}
*/
export function huffman_compress_pgn_str(pgn_str: string): Uint8Array;
/**
* Decompresses a vector of bytes into a PGN string
* @param {Uint8Array} compressed_data
* @returns {string}
*/
export function huffman_decompress_pgn_str(compressed_data: Uint8Array): string;

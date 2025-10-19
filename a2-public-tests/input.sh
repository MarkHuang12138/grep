#!/bin/bash
echo "================================================"
echo "Test case 1: Display help information..." 
echo "------------------------------------------------"
cargo run -- -h
echo "================================================"
echo "Test case 2: Basic Search..."
echo "------------------------------------------------"
cargo run -- Utility tests/grep.md
echo "================================================"
echo "Test case 3: Basic Search with multiple files (a list of files)..."
echo "------------------------------------------------"
cargo run -- Utility tests/grep.md tests/recursive/grep.md
echo "================================================"
echo "Test case 4: Basic Search with multiple files (wildcard characters)..."
echo "------------------------------------------------"
cargo run -- Utility tests/*.md tests/recursive/*.md
echo "================================================"
echo "Test case 5: Case-insensitive search..."
echo "------------------------------------------------"
cargo run -- Utility tests/grep.md -i
echo "================================================"
echo "Test case 6: Print line numbers along with matching lines..."
echo "------------------------------------------------"
cargo run -- Utility tests/grep.md -n
echo "================================================"
echo "Test case 7: Invert match (print lines that do not match the pattern)..."
echo "------------------------------------------------"
cargo run -- Utility tests/grep.md -v
echo "================================================"
echo "Test case 8: Recursive search (search in subdirectories)..."
echo "------------------------------------------------"
cargo run -- Utility tests -r
echo "================================================"
echo "Test case 9: Print filenames along with matching lines..."
echo "------------------------------------------------"
cargo run -- Utility tests -r -f
echo "================================================"
echo "Test case 10: Produce colored output..."
echo "------------------------------------------------"
cargo run -- Utility tests -r -c -f
echo "================================================"
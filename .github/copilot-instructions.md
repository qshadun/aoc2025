# Advent of Code 2024 - AI Agent Instructions

## Project Overview
This is an Advent of Code 2024 solution repository with dual implementations: Python (Jupyter notebooks) and Rust. Each day's puzzle has parallel solutions in both languages.

## Repository Structure
- **Python solutions**: `day{N}.ipynb` - Jupyter notebooks at root level
- **Rust solutions**: `rust2024/src/bin/day{N}.rs` - Binary executables
- **Input files**: `input/day{N}.txt` (real), `input/day{N}_test.txt` (sample)
- **Shared utilities**: `rust2024/src/lib.rs` - Common grid/movement helpers

## Code Patterns

### Python Notebooks
Each notebook follows a consistent structure:
1. Parse input function reading from `input/day{N}.txt` or `input/day{N}_test.txt`
2. `part1(input_file)` function for first puzzle part
3. `part2(input_file)` function for second puzzle part
4. Test cells calling functions with test input
5. Solution cells calling functions with real input

**Example from day1.ipynb:**
```python
def part1(input_file):
    ls1, ls2 = [], []
    with open(input_file) as f:
        for line in f:
            n1, n2 = (re.split(r'\W+', line.rstrip()))
            ls1.append(int(n1))
            ls2.append(int(n2))
    ls1.sort()
    ls2.sort()
    return sum(abs(x-y) for x, y in zip(ls1, ls2))

part1('input/day1_test.txt')  # Test
part1('input/day1.txt')        # Real solution
```

### Rust Solutions
Each Rust binary follows this pattern:
1. `parse_input(input_file: &str)` - Returns parsed data structures
2. `part1(input_file: &str)` and `part2(input_file: &str)` - Solution functions
3. `main()` - Prints all four results (test/real for both parts)

**Example from day1.rs:**
```rust
fn main() {
    println!("ans for part1 test: {}", part1("../input/day1_test.txt"));
    println!("ans for part1: {}", part1("../input/day1.txt"));
    println!("ans for part2 test: {}", part2("../input/day1_test.txt"));
    println!("ans for part2: {}", part2("../input/day1.txt"));
}
```

**Note**: Rust binaries use relative paths `../input/` since they run from `rust2024/target/debug/`.

### Shared Rust Utilities (`lib.rs`)
- `read_grid(input_file: &str) -> Vec<Vec<char>>` - Parse character grids
- `print_grid(grid: &[Vec<char>])` - Debug output for grids
- `Move` enum - Direction handling with `do_move()`, `turn()`, `reverse_move()` methods
- `DELTAS: [[i32; 2]; 4]` - Standard directional offsets `[[-1,0], [1,0], [0,-1], [0,1]]`

### Common Parsing Patterns
- **Python**: Use `re.split(r'\W+', line.rstrip())` or `line.rstrip().split(' ')` for space-separated values
- **Rust**: Use `regex::Regex` with `find_iter()` for flexible number extraction
- **Grid parsing**: Convert lines to 2D character/digit arrays (see day10 examples)

## Running Solutions

### Python
Execute cells in Jupyter notebooks sequentially. No special setup required.

### Rust
```bash
cd rust2024
cargo run --bin day1    # Run specific day
cargo run --bin day10   # Day 10, etc.
```

Dependencies: `itertools = "0.14.0"`, `regex = "1.11.1"` (see Cargo.toml)

## When Creating New Solutions
1. **Python**: Copy pattern from `day1.ipynb` - parse function, part1, part2, test cells
2. **Rust**: Copy pattern from `rust2024/src/bin/day1.rs` - main with 4 println statements
3. **Input files**: Always create both `input/day{N}_test.txt` and `input/day{N}.txt`
4. **Grid problems**: Import utilities from `lib.rs` (`DELTAS`, `Move`, `read_grid`)
5. **Test first**: Always validate with test input before running real input

## Common Algorithms
- **BFS/DFS**: See `day10.ipynb` and `day10.rs` for pathfinding with visited sets
- **Memoization**: Python uses `@cache` decorator (day10), Rust uses manual `memo` Vec
- **Grid navigation**: Use `DELTAS` array for 4-directional movement with bounds checking

## Debug Tips
- Rust: Use `print_grid()` from lib.rs for visual debugging
- Python: Print intermediate states in notebook cells
- Both: Test inputs are small - validate logic there first

# aoc
Unified Advent of Code Repository

## Daily workflow (2026)

Run everything from `2026/`. The session cookie from adventofcode.com goes into `~/.config/aocd/token`.

```sh
sh setup_day.sh                        # today's template + input, `sh setup_day.sh 5` for a specific day
cargo test --bin day05                 # examples from the boilerplate! block
cargo run --release --bin day05        # answers and single-run timings for inputs/day05
cargo run --release --bin day05 big    # same for inputs/day05.big (any suffix works)
cargo bench --bin day05                # precise timings on the real input, asserts bench1/bench2
```

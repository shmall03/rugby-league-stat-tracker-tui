# Rugby League Stat Tracker TUI

A terminal-based (TUI) rugby league match statistics tracker built with Rust. Record scores, tackles, errors, set completion, discipline, and more — all from the comfort of your terminal.

## Features

- **Score tracking** — Tries (4pts), conversions (2pts), penalty goals (2pts), drop goals (1pt)
- **Live stats** — Tackles, errors (knock-ons), six-again calls, penalties awarded
- **Set completion** — Track completed vs attempted sets with percentage
- **Discipline** — Yellow and red cards per player
- **Match phases** — First Half → Halftime → Second Half → Full Time
- **Team toggle** — Switch active team with `Space`; stats apply to the active team
- **Undo** — Revert the last action with `u`
- **Export** — Save match data to JSON (auto-save on quit, manual save with `o`)
- **Color-coded UI** — Each team gets its own color; active team highlighted

## Installation

```bash
# Build the release binary
cargo build --release

# Run it
cargo run --release
```

## Usage

Enter team names at startup, then use the keyboard to track the match:

| Key | Action |
|-----|--------|
| `Space` | Toggle active team |
| `t` | Tackle (against defending team) |
| `r` | Try — enter minute then scorer name |
| `c` | Conversion — enter minute then kicker name |
| `g` | Penalty goal — enter minute then kicker name |
| `d` | Drop goal — enter minute then scorer name |
| `n` | Set NOT completed (attacking team) |
| `m` | Set completed (attacking team) |
| `x` | Six again (defending team) |
| `e` | Error / knock-on (attacking team) |
| `p` | Penalty awarded (against defending team) |
| `y` | Yellow card — enter minute then player name |
| `R` | Red card — enter minute then player name |
| `s` | Advance match phase |
| `u` | Undo last action |
| `o` | Save match to JSON |
| `q` | Quit (auto-saves if teams have data) |

## Export

Matches are saved as `match_{team_a}_v_{team_b}_{timestamp}.json` in the current directory. The JSON captures the full match state including all events, scores, and per-team stats.

## Project Structure

```
src/
├── main.rs      — Entry point and event loop
├── app.rs       — Application state and key handling
├── models.rs    — Data models (teams, phases, events, scoring)
├── ui.rs        — Terminal UI rendering (ratatui)
├── input.rs     — Text input component
└── export.rs    — JSON export
```

## License

MIT License

Copyright (c) 2026

Permission is hereby granted, free of charge, to any person obtaining a copy
of this software and associated documentation files (the "Software"), to deal
in the Software without restriction, including without limitation the rights
to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
copies of the Software, and to permit persons to whom the Software is
furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all
copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
SOFTWARE.

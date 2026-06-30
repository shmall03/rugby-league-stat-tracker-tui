# Rugby League Stat Tracker TUI

A terminal-based (TUI) rugby league match statistics tracker built with Rust. Record scores, tackles, errors, set completion, discipline, and more. Any suggestions are welcome.

## Features

- **Score tracking** — Tries (4pts), conversions (2pts), penalty goals (2pts), drop goals (1pt)
- **Live stats** — Tackles, errors (knock-ons), six-again calls, penalties awarded
- **Set completion** — Track completed vs attempted sets with percentage
- **Discipline** — Yellow and red cards per player
- **Match phases** — First Half → Halftime → Second Half → Full Time
- **Team toggle** — Switch active team with `Space`; stats apply to the active team
- **Undo** — Revert the last action with `u`
- **Export** — Save match data to JSON (auto-save on quit, manual save with `o`)
- **Colour-coded UI** — Each team gets its own colour; active team highlighted

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
| `Space` | Toggle team currently in possession |
| `t` | Tackle (against attacking team) |
| `r` | Try — enter minute then scorer name |
| `c` | Conversion — enter minute then kicker name |
| `g` | Penalty goal — enter minute then kicker name |
| `d` | Drop goal — enter minute then kicker name |
| `n` | Set NOT completed (by attacking team) |
| `m` | Set completed (by attacking team) |
| `x` | Six again (for attacking team) |
| `e` | Error / knock-on (by attacking team) |
| `p` | Penalty awarded (against defending team) |
| `y` | Yellow card — enter minute then player name |
| `R` `(SHIFT + r)` | Red card — enter minute then player name |
| `s` | Advance match phase (First Half -> Half-time -> Second Half -> Match Finished)* |
| `u` | Undo last action |
| `o` | Save match to JSON |
| `q` | Quit (auto-saves if teams have data) |

(*) Matches that go into extra time and/or golden point are not currently supported but can be tracked by simply continuing the second half.

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

Licensed under the [MIT License](LICENSE).

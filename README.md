# 🏉 Rugby League Stat Tracker TUI

[![Rust](https://img.shields.io/badge/Rust-1.85+-de5842?logo=rust&logoColor=white)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue)](LICENSE)
[![ratatui](https://img.shields.io/badge/built%20with-ratatui-FFD700)](https://github.com/ratatui-org/ratatui)

A terminal-based rugby league match statistics tracker built with Rust. Record scores, tackles, errors, set completion, discipline, possession, and more. 🏟️

## ✨ Features

- ⏱️ **Live match clock** — Start/pause with `Space`, displayed in the header
- 🎯 **Score tracking** — Tries 4pts, conversions 2pts, penalty goals 2pts, drop goals 1pt
- 📊 **Live stats** — Tackles, errors (knock-ons), six-again calls, penalties awarded
- 🔄 **Possession tracking** — Live % per team, toggle with `i`, visual indicator
- ✅ **Set completion** — Completed vs attempted sets with percentage
- 🟡🔴 **Discipline** — Yellow and red cards per player
- 🏁 **Match phases** — Auto-advance at 40′ and 80′; ties prompt End/Golden Point
- 🕐 **Auto-minute** — Event minute auto-populated from the clock
- 🔀 **Team toggle** — Switch active team with `Tab`
- ↩️ **Undo** — Revert the last action with `u`
- 💾 **Export** — Save match data to JSON (auto-save on quit, manual save with `o`)
- 🎨 **Colour-coded UI** — Each team gets its own colour; active team highlighted
- ⚖️ **Tie resolution** — End in a tie or play Golden Point extra time; first score wins

![Example screenshot](https://raw.githubusercontent.com/shmall03/rugby-league-stat-tracker-tui/master/example.png)

## 📦 Installation

```bash
# Build the release binary
cargo build --release

# Run it
cargo run --release
```

## 🎮 Usage

Enter team names at startup, then use the keyboard to track the match:

| Key | Action |
|-----|--------|
| `Space` | Start / pause match clock |
| `Tab` | Switch active team |
| `t` | Tackle (defending team) |
| `r` | Try — enter scorer name |
| `c` | Conversion — enter kicker name |
| `g` | Penalty goal — enter kicker name |
| `d` | Drop goal — enter scorer name |
| `n` | Set NOT completed |
| `m` | Set completed |
| `x` | Six again |
| `e` | Error / knock-on |
| `p` | Penalty awarded |
| `i` | Toggle possession |
| `y` | Yellow card — enter player name |
| `R` | Red card — enter player name |
| `s` | Advances stage of the game: First Half → Halftime → Second Half → Full Time / Golden Point ET |
| `u` | Undo last action |
| `o` | Save match to JSON |
| `E` | End match in a tie (when prompted at full time) |
| `G` | Golden Point extra time (when prompted) |
| `q` | Quit (auto-saves if teams have data) |

All event minute fields are auto-populated from the match clock — no manual entry required.

## 📤 Export

Matches are saved as `match_{team_a}_v_{team_b}_{timestamp}.json` in the current directory. The JSON captures the full match state including all events, scores, per-team stats, and possession data.

## 📁 Project Structure

```
src/
├── main.rs      — Entry point and event loop
├── app.rs       — Application state and key handling
├── models.rs    — Data models (teams, phases, events, scoring)
├── ui.rs        — Terminal UI rendering (ratatui)
├── input.rs     — Text input component
└── export.rs    — JSON export
```

## ⚖️ License

Licensed under the [MIT License](LICENSE).

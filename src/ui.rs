use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span, Text},
    widgets::{Block, Borders, Paragraph, Wrap},
};

use crate::app::*;
use crate::models::*;

fn team_color(team: Team) -> Color {
    match team {
        Team::A => Color::Green,
        Team::B => Color::Cyan,
    }
}

fn truncate(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max.saturating_sub(1)])
    } else {
        s.to_string()
    }
}

fn render_header(f: &mut Frame, area: Rect, app: &App) {
    let mins = app.state.elapsed_secs / 60;
    let secs = app.state.elapsed_secs % 60;
    let clock_indicator = if app.state.clock_running { "▶" } else { "⏸" };
    let clock_color = if app.state.clock_running { Color::Green } else { Color::Red };

    let text = Text::from(vec![
        Line::from(vec![Span::raw("")]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                truncate(&app.state.team_a, 16),
                Style::default().fg(team_color(Team::A)).add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                format!("{}", app.state.score(Team::A)),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::styled(" - ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}", app.state.score(Team::B)),
                Style::default().add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                truncate(&app.state.team_b, 16),
                Style::default().fg(team_color(Team::B)).add_modifier(Modifier::BOLD),
            ),
            Span::raw("   "),
            Span::styled(app.state.phase.label(), Style::default().fg(Color::DarkGray)),
            Span::raw("  "),
            Span::styled(
                format!("{} {:02}:{:02}", clock_indicator, mins, secs),
                Style::default().fg(clock_color).add_modifier(Modifier::BOLD),
            ),
        ]),
    ]);

    let paragraph = Paragraph::new(text)
        .style(Style::default())
        .alignment(Alignment::Center);
    f.render_widget(paragraph, area);
}

fn render_team_panel(f: &mut Frame, area: Rect, app: &App, team: Team) {
    let mut lines: Vec<Line> = Vec::new();

    let is_active = team == app.active_team;

    lines.push(Line::from(""));

    let poss = app.state.possession_pct(team);
    let poss_color = if poss >= 55.0 {
        team_color(team)
    } else if poss >= 45.0 {
        Color::Yellow
    } else {
        Color::Red
    };
    lines.push(Line::from(vec![
        Span::raw("Possession:         "),
        Span::styled(
            format!("{:.0}%", poss),
            Style::default()
                .fg(poss_color)
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    let tkl = app.state.tackles(team);
    lines.push(Line::from(vec![
        Span::raw("Tackles:            "),
        Span::styled(
            tkl.to_string(),
            Style::default()
                .fg(team_color(team))
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    let errs = app.state.errors(team);
    lines.push(Line::from(vec![
        Span::raw("Errors:             "),
        Span::styled(
            errs.to_string(),
            Style::default()
                .fg(team_color(team))
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    let sc = app.state.sets_completed(team);
    let sa = app.state.sets_attempted(team);
    let scp = app.state.sets_completion_pct(team);
    lines.push(Line::from(vec![
        Span::raw("Set Completion:     "),
        Span::styled(
            format!("{:.0}%", scp),
            Style::default()
                .fg(if scp >= 75.0 { Color::Green } else if scp >= 50.0 { Color::Yellow } else { Color::Red })
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(" ({}/{})", sc, sa)),
    ]));

    let pen_against = app.state.penalties_against(team);
    lines.push(Line::from(vec![
        Span::raw("Penalties Against:  "),
        Span::styled(
            pen_against.to_string(),
            Style::default()
                .fg(team_color(team))
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    let six = app.state.six_agains(team);
    lines.push(Line::from(vec![
        Span::raw("Six Again:          "),
        Span::styled(
            six.to_string(),
            Style::default()
                .fg(team_color(team))
                .add_modifier(Modifier::BOLD),
        ),
    ]));

    let pen_goals = app.state.penalty_goals(team);
    let pg_count = pen_goals.len();
    let pg_points = pg_count * 2;
    lines.push(Line::from(vec![
        Span::raw("Penalty Goals:      "),
        Span::styled(
            pg_count.to_string(),
            Style::default()
                .fg(team_color(team))
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(" ({}pts)", pg_points)),
    ]));

    let tries = app.state.tries(team);
    if !tries.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "── TRIES ──",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::DIM),
        )));
        for t in tries.iter().rev().take(6) {
            lines.push(Line::from(vec![
                Span::styled(format!("{:02}' ", t.minute), Style::default().fg(Color::DarkGray)),
                Span::raw(t.player.clone()),
            ]));
        }
        if tries.len() > 6 {
            lines.push(Line::from(Span::styled(
                format!("  ... and {} more", tries.len() - 6),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let pgs: Vec<&PenaltyGoalEvent> = app.state.penalty_goals(team);
    if !pgs.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "── PENALTY GOALS ──",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::DIM),
        )));
        for p in pgs.iter().rev().take(4) {
            lines.push(Line::from(vec![
                Span::styled(format!("{:02}' ", p.minute), Style::default().fg(Color::DarkGray)),
                Span::styled(p.player.clone(), Style::default().fg(team_color(team))),
                Span::raw(" "),
                Span::styled("●", Style::default().fg(Color::Green)),
            ]));
        }
        if pgs.len() > 4 {
            lines.push(Line::from(Span::styled(
                format!("  ... and {} more", pgs.len() - 4),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let convs = app.state.conversions(team);
    if !convs.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "── CONVERSIONS ──",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::DIM),
        )));
        for c in convs.iter().rev().take(4) {
            let sym = if c.successful { "✓" } else { "✗" };
            let sym_color = if c.successful { Color::Green } else { Color::Red };
            lines.push(Line::from(vec![
                Span::styled(format!("{:02}' ", c.minute), Style::default().fg(Color::DarkGray)),
                Span::styled(c.kicker.clone(), Style::default().fg(team_color(team))),
                Span::raw(" "),
                Span::styled(sym, Style::default().fg(sym_color)),
            ]));
        }
        if convs.len() > 4 {
            lines.push(Line::from(Span::styled(
                format!("  ... and {} more", convs.len() - 4),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let dgs = app.state.drop_goals(team);
    if !dgs.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "── DROP GOALS ──",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::DIM),
        )));
        for d in dgs.iter().rev().take(4) {
            lines.push(Line::from(vec![
                Span::styled(format!("{:02}' ", d.minute), Style::default().fg(Color::DarkGray)),
                Span::styled(d.player.clone(), Style::default().fg(team_color(team))),
                Span::raw(" "),
                Span::styled("⊡", Style::default().fg(Color::Green)),
            ]));
        }
        if dgs.len() > 4 {
            lines.push(Line::from(Span::styled(
                format!("  ... and {} more", dgs.len() - 4),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    let cards = app.state.cards(team);
    if !cards.is_empty() {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "── CARDS ──",
            Style::default()
                .fg(Color::Blue)
                .add_modifier(Modifier::DIM),
        )));
        for c in cards.iter().rev().take(6) {
            let (clabel, ccolor) = match c.card_type {
                CardType::Yellow => ("YC", Color::Yellow),
                CardType::Red => ("RC", Color::Red),
            };
            lines.push(Line::from(vec![
                Span::styled(format!("{:02}' ", c.minute), Style::default().fg(Color::DarkGray)),
                Span::styled(
                    format!("{} ", clabel),
                    Style::default()
                        .fg(ccolor)
                        .add_modifier(Modifier::BOLD),
                ),
                Span::raw(c.player.clone()),
            ]));
        }
        if cards.len() > 6 {
            lines.push(Line::from(Span::styled(
                format!("  ... and {} more", cards.len() - 6),
                Style::default().fg(Color::DarkGray),
            )));
        }
    }

    if app.input_target == InputTarget::None && app.message.is_none() {
        let phase_color = if app.state.phase == Phase::FullTime {
            Color::Red
        } else {
            Color::DarkGray
        };
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("Phase: {}", app.state.phase.label()),
            Style::default().fg(phase_color),
        )));
    }

    let title_prefix = if is_active { "▸ " } else { "  " };
    let title_color = if is_active { Color::Yellow } else { Color::DarkGray };

    let title_name = if is_active && app.state.in_possession {
        format!("{}  ● IN POSSESSION ", truncate(app.state.team_name(team), 14))
    } else {
        truncate(app.state.team_name(team), 18)
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" {} {} ", title_prefix, title_name))
        .title_style(Style::default().fg(title_color).add_modifier(Modifier::BOLD))
        .border_style(Style::default().fg(if is_active { Color::Yellow } else { Color::DarkGray }));

    let text = Text::from(lines);
    let paragraph = Paragraph::new(text).block(block).wrap(Wrap { trim: false });
    f.render_widget(paragraph, area);
}

fn render_footer(f: &mut Frame, area: Rect, app: &App) {
    if app.input.active {
        let text = Text::from(Line::from(format!(
            "{} {}",
            app.input.prompt, app.input.buffer
        )));
        let paragraph = Paragraph::new(text)
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD));
        f.render_widget(paragraph, area);
    } else if let Some(ref msg) = app.message {
        let text = Text::from(Line::from(Span::styled(msg, Style::default().fg(Color::Yellow))));
        let paragraph = Paragraph::new(text);
        f.render_widget(paragraph, area);
    } else {
        let text = Text::from(vec![
            Line::from(vec![
                Span::styled("SPACE", Style::default().fg(Color::Yellow)),
                Span::raw(": Clock    "),
                Span::styled("TAB", Style::default().fg(Color::Yellow)),
                Span::raw(": Team    "),
                Span::styled("t", Style::default().fg(Color::Yellow)),
                Span::raw(": Tackle    "),
                Span::styled("r", Style::default().fg(Color::Yellow)),
                Span::raw(": Try    "),
                Span::styled("c", Style::default().fg(Color::Yellow)),
                Span::raw(": Conversion    "),
                Span::styled("g", Style::default().fg(Color::Yellow)),
                Span::raw(": Penalty Goal    "),
                Span::styled("d", Style::default().fg(Color::Yellow)),
                Span::raw(": Drop Goal    "),
                Span::styled("n", Style::default().fg(Color::Yellow)),
                Span::raw(": Incomplete Set    "),
                Span::styled("m", Style::default().fg(Color::Yellow)),
                Span::raw(": Complete Set"),
            ]),
            Line::from(vec![
                Span::styled("x", Style::default().fg(Color::Yellow)),
                Span::raw(": Six Again    "),
                Span::styled("e", Style::default().fg(Color::Yellow)),
                Span::raw(": Error    "),
                Span::styled("p", Style::default().fg(Color::Yellow)),
                Span::raw(": Penalty    "),
                Span::styled("i", Style::default().fg(Color::Yellow)),
                Span::raw(": Possess    "),
                Span::styled("y", Style::default().fg(Color::Yellow)),
                Span::raw(": Yellow Card    "),
                Span::styled("R", Style::default().fg(Color::Yellow)),
                Span::raw(": Red Card    "),
                Span::styled("s", Style::default().fg(Color::Yellow)),
                Span::raw(": Phase    "),
                Span::styled("u", Style::default().fg(Color::Yellow)),
                Span::raw(": Undo    "),
                Span::styled("o", Style::default().fg(Color::Yellow)),
                Span::raw(": Save    "),
                Span::styled("q", Style::default().fg(Color::Yellow)),
                Span::raw(": Quit"),
            ]),
        ]);
        let paragraph = Paragraph::new(text).style(Style::default().fg(Color::DarkGray));
        f.render_widget(paragraph, area);
    }
}

pub fn render(f: &mut Frame, app: &App) {
    let size = f.area();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
            Constraint::Length(3),
        ])
        .split(size);

    render_header(f, chunks[0], app);

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    render_team_panel(f, main_chunks[0], app, Team::A);
    render_team_panel(f, main_chunks[1], app, Team::B);

    render_footer(f, chunks[2], app);
}

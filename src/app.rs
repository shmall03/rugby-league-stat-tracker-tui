use std::time::Instant;

use crossterm::event::{KeyCode, KeyEvent};

use crate::input::TextInput;
use crate::models::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputTarget {
    None,
    SetupTeamA,
    SetupTeamB,
    TryScorer,
    ConversionKicker,
    PenaltyGoalKicker,
    DropGoalScorer,
    YellowCardPlayer,
    RedCardPlayer,
}

pub struct App {
    pub state: MatchState,
    pub undo_stack: Vec<MatchState>,
    pub input: TextInput,
    pub input_target: InputTarget,
    pub active_team: Team,
    pub message: Option<String>,
    pub message_ticks: u32,
    pub should_quit: bool,
    clock_baseline: u64,
    clock_resumed_at: Option<Instant>,
    last_possession_tick: Option<Instant>,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            state: MatchState::new(String::new(), String::new()),
            undo_stack: Vec::new(),
            input: TextInput::new(),
            input_target: InputTarget::SetupTeamA,
            active_team: Team::A,
            message: None,
            message_ticks: 0,
            should_quit: false,
            clock_baseline: 0,
            clock_resumed_at: None,
            last_possession_tick: None,
        };
        app.input.start("Enter Team A name: ");
        app
    }

    fn save_state(&mut self) {
        self.undo_stack.push(self.state.clone());
    }

    fn set_message(&mut self, msg: String) {
        self.message = Some(msg);
        self.message_ticks = 0;
    }

    fn flush_possession(&mut self) {
        if let Some(last) = self.last_possession_tick {
            let delta = Instant::now().duration_since(last).as_secs_f64();
            *self.state.possession_secs_mut(self.active_team) += delta;
        }
    }

    pub fn tick(&mut self, _delta: f64) {
        if self.message.is_some() {
            self.message_ticks += 1;
            if self.message_ticks > 25 {
                self.message = None;
            }
        }

        if self.state.clock_running {
            if let Some(resumed) = self.clock_resumed_at {
                self.state.elapsed_secs = self.clock_baseline + resumed.elapsed().as_secs();
            }

            let now = Instant::now();
            if let Some(last) = self.last_possession_tick {
                if self.state.in_possession {
                    let delta = now.duration_since(last).as_secs_f64();
                    *self.state.possession_secs_mut(self.active_team) += delta;
                }
            }
            self.last_possession_tick = Some(now);
        }
    }

    pub fn handle_input_target(&mut self, target: InputTarget, submitted: bool, value: Option<String>) {
        let minute = self.state.active_minute();
        match target {
            InputTarget::SetupTeamA => {
                if submitted {
                    if let Some(name) = value {
                        if !name.trim().is_empty() {
                            self.state.team_a = name.trim().to_string();
                            self.input_target = InputTarget::SetupTeamB;
                            self.input.start("Enter Team B name: ");
                            return;
                        }
                    }
                }
                self.input.start("Enter Team A name: ");
            }
            InputTarget::SetupTeamB => {
                if submitted {
                    if let Some(name) = value {
                        if !name.trim().is_empty() {
                            self.state.team_b = name.trim().to_string();
                            self.input_target = InputTarget::None;
                            self.set_message("Match ready! Press Space for clock, Tab to switch team.".to_string());
                            return;
                        }
                    }
                }
                self.input.start("Enter Team B name: ");
            }
            InputTarget::TryScorer => {
                if submitted {
                    if let Some(player) = value {
                        if !player.trim().is_empty() {
                            let team = self.active_team;
                            let try_event = TryEvent {
                                team,
                                player: player.trim().to_string(),
                                minute,
                            };
                            self.save_state();
                            self.state.events.push(Event::Try(try_event));
                            self.set_message(format!("Try scored by {}! (+4 pts)", player.trim()));
                        }
                    }
                }
            }
            InputTarget::ConversionKicker => {
                if submitted {
                    if let Some(kicker) = value {
                        if !kicker.trim().is_empty() {
                            let team = self.active_team;
                            self.save_state();
                            let event = ConversionEvent {
                                team,
                                kicker: kicker.trim().to_string(),
                                minute,
                                successful: true,
                            };
                            self.state.events.push(Event::Conversion(event));
                            self.set_message(format!("Conversion by {}! (+2 pts)", kicker.trim()));
                        }
                    }
                }
            }
            InputTarget::PenaltyGoalKicker => {
                if submitted {
                    if let Some(kicker) = value {
                        if !kicker.trim().is_empty() {
                            let team = self.active_team;
                            self.save_state();
                            let event = PenaltyGoalEvent {
                                team,
                                player: kicker.trim().to_string(),
                                minute,
                            };
                            self.state.events.push(Event::PenaltyGoal(event));
                            self.set_message(format!("Penalty goal by {}! (+2 pts)", kicker.trim()));
                        }
                    }
                }
            }
            InputTarget::DropGoalScorer => {
                if submitted {
                    if let Some(player) = value {
                        if !player.trim().is_empty() {
                            let team = self.active_team;
                            self.save_state();
                            let event = DropGoalEvent {
                                team,
                                player: player.trim().to_string(),
                                minute,
                            };
                            self.state.events.push(Event::DropGoal(event));
                            self.set_message(format!("Drop goal by {}! (+1 pt)", player.trim()));
                        }
                    }
                }
            }
            InputTarget::YellowCardPlayer => {
                if submitted {
                    if let Some(player) = value {
                        if !player.trim().is_empty() {
                            let team = self.active_team;
                            let card_event = CardEvent {
                                team,
                                player: player.trim().to_string(),
                                minute,
                                card_type: CardType::Yellow,
                            };
                            self.save_state();
                            self.state.events.push(Event::Card(card_event));
                            self.set_message(format!("Yellow card: {} ({})", player.trim(), self.state.team_name(team)));
                        }
                    }
                }
            }
            InputTarget::RedCardPlayer => {
                if submitted {
                    if let Some(player) = value {
                        if !player.trim().is_empty() {
                            let team = self.active_team;
                            let card_event = CardEvent {
                                team,
                                player: player.trim().to_string(),
                                minute,
                                card_type: CardType::Red,
                            };
                            self.save_state();
                            self.state.events.push(Event::Card(card_event));
                            self.set_message(format!("Red card: {} ({})", player.trim(), self.state.team_name(team)));
                        }
                    }
                }
            }
            InputTarget::None => {}
        }
        if !matches!(target, InputTarget::SetupTeamA | InputTarget::SetupTeamB) {
            self.input_target = InputTarget::None;
        }
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if self.input.active {
            let result = self.input.handle_key(key);
            if !self.input.active {
                let submitted = result.is_some();
                let target = self.input_target;
                self.handle_input_target(target, submitted, result);
            }
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.should_quit = true;
            }
            KeyCode::Char(' ') | KeyCode::Tab if self.input_target != InputTarget::None => {}
            KeyCode::Tab => {
                self.flush_possession();
                self.active_team = self.active_team.other();
                self.last_possession_tick = Some(Instant::now());
                self.set_message(format!("Active team: {}", self.state.team_name(self.active_team)));
            }
            KeyCode::Char(' ') => {
                if self.state.clock_running {
                    self.flush_possession();
                    self.state.clock_running = false;
                    self.clock_resumed_at = None;
                    self.last_possession_tick = None;
                    self.set_message("Clock paused".to_string());
                } else {
                    self.clock_baseline = self.state.elapsed_secs;
                    self.clock_resumed_at = Some(Instant::now());
                    self.state.clock_running = true;
                    self.last_possession_tick = Some(Instant::now());
                    self.set_message("Clock running".to_string());
                }
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                let defending = self.active_team.other();
                let minute = self.state.active_minute();
                self.save_state();
                self.state.events.push(Event::Tackle(TackleEvent { team: defending, minute }));
                self.set_message(format!("Tackle: {}", self.state.team_name(defending)));
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                let minute = self.state.active_minute();
                self.save_state();
                let event = ErrorEvent { team: self.active_team, minute };
                self.state.events.push(Event::Error(event));
                self.set_message(format!("Error (knock-on): {}", self.state.team_name(self.active_team)));
            }
            KeyCode::Char('x') | KeyCode::Char('X') => {
                let defending = self.active_team.other();
                let minute = self.state.active_minute();
                self.save_state();
                let event = SixAgainEvent { team: defending, minute };
                self.state.events.push(Event::SixAgain(event));
                self.set_message(format!("Six again: {}", self.state.team_name(defending)));
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                self.save_state();
                match self.active_team {
                    Team::A => self.state.sets_attempted_a += 1,
                    Team::B => self.state.sets_attempted_b += 1,
                }
                self.set_message(format!("Set NOT completed: {}", self.state.team_name(self.active_team)));
            }
            KeyCode::Char('m') | KeyCode::Char('M') => {
                self.save_state();
                match self.active_team {
                    Team::A => {
                        self.state.sets_attempted_a += 1;
                        self.state.sets_completed_a += 1;
                    }
                    Team::B => {
                        self.state.sets_attempted_b += 1;
                        self.state.sets_completed_b += 1;
                    }
                }
                self.set_message(format!("Set completed: {}", self.state.team_name(self.active_team)));
            }
            KeyCode::Char('r') => {
                self.input_target = InputTarget::TryScorer;
                self.input.start("Try scorer name: ");
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if self.state.recent_try(self.active_team).is_some() {
                    self.input_target = InputTarget::ConversionKicker;
                    self.input.start("Conversion kicker name: ");
                } else {
                    self.set_message("No try to convert".to_string());
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.input_target = InputTarget::DropGoalScorer;
                self.input.start("Drop goal scorer name: ");
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                let defending = self.active_team.other();
                let minute = self.state.active_minute();
                self.save_state();
                let event = PenaltyAwardedEvent { team: defending, minute };
                self.state.events.push(Event::PenaltyAwarded(event));
                self.set_message(format!("Penalty awarded against: {}", self.state.team_name(defending)));
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                self.input_target = InputTarget::PenaltyGoalKicker;
                self.input.start("Penalty goal kicker name: ");
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                if self.state.clock_running {
                    self.state.in_possession = !self.state.in_possession;
                    if self.state.in_possession {
                        self.last_possession_tick = Some(Instant::now());
                        self.set_message("In possession: yes".to_string());
                    } else {
                        self.flush_possession();
                        self.set_message("In possession: no".to_string());
                    }
                } else {
                    self.set_message("Start the clock first".to_string());
                }
            }
            KeyCode::Char('y') => {
                self.input_target = InputTarget::YellowCardPlayer;
                self.input.start("Yellow card - player name: ");
            }
            KeyCode::Char('R') => {
                self.input_target = InputTarget::RedCardPlayer;
                self.input.start("Red card - player name: ");
            }
            KeyCode::Char('s') | KeyCode::Char('S') => {
                self.save_state();
                self.state.advance_phase();
                self.set_message(format!("Phase: {}", self.state.phase.label()));
            }
            KeyCode::Char('u') | KeyCode::Char('U') => {
                if let Some(previous) = self.undo_stack.pop() {
                    self.state = previous;
                    self.set_message("Undone last action".to_string());
                } else {
                    self.set_message("Nothing to undo".to_string());
                }
            }
            KeyCode::Char('o') | KeyCode::Char('O') => {
                match crate::export::save_to_json(&self.state) {
                    Ok(path) => self.set_message(format!("Saved: {}", path)),
                    Err(e) => self.set_message(format!("Save error: {}", e)),
                }
            }
            _ => {}
        }
    }
}

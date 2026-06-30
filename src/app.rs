use crossterm::event::{KeyCode, KeyEvent};

use crate::input::TextInput;
use crate::models::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputTarget {
    None,
    SetupTeamA,
    SetupTeamB,
    TryMinute,
    TryScorer,
    ConversionMinute,
    ConversionKicker,
    PenaltyGoalMinute,
    PenaltyGoalKicker,
    DropGoalMinute,
    DropGoalScorer,
    YellowCardMinute,
    YellowCardPlayer,
    RedCardMinute,
    RedCardPlayer,
}

pub struct App {
    pub state: MatchState,
    pub undo_stack: Vec<MatchState>,
    pub input: TextInput,
    pub input_target: InputTarget,
    pub pending_minute: Option<u32>,
    pub active_team: Team,
    pub message: Option<String>,
    pub message_ticks: u32,
    pub should_quit: bool,
}

impl App {
    pub fn new() -> Self {
        let mut app = App {
            state: MatchState::new(String::new(), String::new()),
            undo_stack: Vec::new(),
            input: TextInput::new(),
            input_target: InputTarget::SetupTeamA,
            pending_minute: None,
            active_team: Team::A,
            message: None,
            message_ticks: 0,
            should_quit: false,
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

    pub fn tick(&mut self, _delta: f64) {
        if self.message.is_some() {
            self.message_ticks += 1;
            if self.message_ticks > 25 {
                self.message = None;
            }
        }
    }

    pub fn handle_input_target(&mut self, target: InputTarget, submitted: bool, value: Option<String>) {
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
                            self.set_message("Match ready! Press Space to switch active team.".to_string());
                            return;
                        }
                    }
                }
                self.input.start("Enter Team B name: ");
            }
            InputTarget::TryMinute => {
                if submitted {
                    if let Some(m) = value {
                        if let Ok(minute) = m.trim().parse::<u32>() {
                            self.pending_minute = Some(minute);
                            self.input_target = InputTarget::TryScorer;
                            self.input.start("Try scorer name: ");
                            return;
                        }
                    }
                }
                self.input.start("Minute? ");
            }
            InputTarget::TryScorer => {
                if submitted {
                    if let Some(player) = value {
                        if !player.trim().is_empty() {
                            let team = self.active_team;
                            let minute = self.pending_minute.take().unwrap_or(0);
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
            InputTarget::ConversionMinute => {
                if submitted {
                    if let Some(m) = value {
                        if let Ok(minute) = m.trim().parse::<u32>() {
                            self.pending_minute = Some(minute);
                            self.input_target = InputTarget::ConversionKicker;
                            self.input.start("Conversion kicker name: ");
                            return;
                        }
                    }
                }
                self.input.start("Minute? ");
            }
            InputTarget::ConversionKicker => {
                if submitted {
                    if let Some(kicker) = value {
                        if !kicker.trim().is_empty() {
                            let team = self.active_team;
                            let minute = self.pending_minute.take().unwrap_or(0);
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
            InputTarget::PenaltyGoalMinute => {
                if submitted {
                    if let Some(m) = value {
                        if let Ok(minute) = m.trim().parse::<u32>() {
                            self.pending_minute = Some(minute);
                            self.input_target = InputTarget::PenaltyGoalKicker;
                            self.input.start("Penalty goal kicker name: ");
                            return;
                        }
                    }
                }
                self.input.start("Minute? ");
            }
            InputTarget::PenaltyGoalKicker => {
                if submitted {
                    if let Some(kicker) = value {
                        if !kicker.trim().is_empty() {
                            let team = self.active_team;
                            let minute = self.pending_minute.take().unwrap_or(0);
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
            InputTarget::DropGoalMinute => {
                if submitted {
                    if let Some(m) = value {
                        if let Ok(minute) = m.trim().parse::<u32>() {
                            self.pending_minute = Some(minute);
                            self.input_target = InputTarget::DropGoalScorer;
                            self.input.start("Drop goal scorer name: ");
                            return;
                        }
                    }
                }
                self.input.start("Minute? ");
            }
            InputTarget::DropGoalScorer => {
                if submitted {
                    if let Some(player) = value {
                        if !player.trim().is_empty() {
                            let team = self.active_team;
                            let minute = self.pending_minute.take().unwrap_or(0);
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
            InputTarget::YellowCardMinute => {
                if submitted {
                    if let Some(m) = value {
                        if let Ok(minute) = m.trim().parse::<u32>() {
                            self.pending_minute = Some(minute);
                            self.input_target = InputTarget::YellowCardPlayer;
                            self.input.start("Yellow card - player name: ");
                            return;
                        }
                    }
                }
                self.input.start("Minute? ");
            }
            InputTarget::YellowCardPlayer => {
                if submitted {
                    if let Some(player) = value {
                        if !player.trim().is_empty() {
                            let team = self.active_team;
                            let minute = self.pending_minute.take().unwrap_or(0);
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
            InputTarget::RedCardMinute => {
                if submitted {
                    if let Some(m) = value {
                        if let Ok(minute) = m.trim().parse::<u32>() {
                            self.pending_minute = Some(minute);
                            self.input_target = InputTarget::RedCardPlayer;
                            self.input.start("Red card - player name: ");
                            return;
                        }
                    }
                }
                self.input.start("Minute? ");
            }
            InputTarget::RedCardPlayer => {
                if submitted {
                    if let Some(player) = value {
                        if !player.trim().is_empty() {
                            let team = self.active_team;
                            let minute = self.pending_minute.take().unwrap_or(0);
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
        if key.code == KeyCode::Tab && self.input.active && !matches!(self.input_target, InputTarget::SetupTeamA | InputTarget::SetupTeamB) {
            self.set_message("No clock to pause — press Space to switch team.".to_string());
            return;
        }

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
                self.set_message("No clock to pause — press Space to switch team.".to_string());
            }
            KeyCode::Char(' ') => {
                self.active_team = self.active_team.other();
                self.set_message(format!("Active team: {}", self.state.team_name(self.active_team)));
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                let defending = self.active_team.other();
                self.save_state();
                match defending {
                    Team::A => self.state.tackles_a += 1,
                    Team::B => self.state.tackles_b += 1,
                }
                self.set_message(format!("Tackle: {}", self.state.team_name(defending)));
            }
            KeyCode::Char('e') | KeyCode::Char('E') => {
                self.save_state();
                let event = ErrorEvent { team: self.active_team, minute: 0 };
                self.state.events.push(Event::Error(event));
                self.set_message(format!("Error (knock-on): {}", self.state.team_name(self.active_team)));
            }
            KeyCode::Char('x') | KeyCode::Char('X') => {
                let defending = self.active_team.other();
                self.save_state();
                let event = SixAgainEvent { team: defending, minute: 0 };
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
                self.input_target = InputTarget::TryMinute;
                self.input.start("Minute? ");
            }
            KeyCode::Char('c') | KeyCode::Char('C') => {
                if self.state.recent_try(self.active_team).is_some() {
                    self.input_target = InputTarget::ConversionMinute;
                    self.input.start("Minute? ");
                } else {
                    self.set_message("No try to convert".to_string());
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                self.input_target = InputTarget::DropGoalMinute;
                self.input.start("Minute? ");
            }
            KeyCode::Char('p') | KeyCode::Char('P') => {
                let defending = self.active_team.other();
                self.save_state();
                let event = PenaltyAwardedEvent { team: defending, minute: 0 };
                self.state.events.push(Event::PenaltyAwarded(event));
                self.set_message(format!("Penalty awarded against: {}", self.state.team_name(defending)));
            }
            KeyCode::Char('g') | KeyCode::Char('G') => {
                self.input_target = InputTarget::PenaltyGoalMinute;
                self.input.start("Minute? ");
            }
            KeyCode::Char('y') => {
                self.input_target = InputTarget::YellowCardMinute;
                self.input.start("Minute? ");
            }
            KeyCode::Char('R') => {
                self.input_target = InputTarget::RedCardMinute;
                self.input.start("Minute? ");
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

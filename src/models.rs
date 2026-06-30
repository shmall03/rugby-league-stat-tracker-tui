use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Team {
    A,
    B,
}

impl Team {
    pub fn other(&self) -> Team {
        match self {
            Team::A => Team::B,
            Team::B => Team::A,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    FirstHalf,
    Halftime,
    SecondHalf,
    FullTime,
}

impl Phase {
    pub fn label(&self) -> &str {
        match self {
            Phase::FirstHalf => "First Half",
            Phase::Halftime => "Halftime",
            Phase::SecondHalf => "Second Half",
            Phase::FullTime => "Full Time",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TryEvent {
    pub team: Team,
    pub player: String,
    pub minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversionEvent {
    pub team: Team,
    pub kicker: String,
    pub minute: u32,
    pub successful: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PenaltyAwardedEvent {
    pub team: Team,
    pub minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PenaltyGoalEvent {
    pub team: Team,
    pub player: String,
    pub minute: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CardType {
    Yellow,
    Red,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CardEvent {
    pub team: Team,
    pub player: String,
    pub minute: u32,
    pub card_type: CardType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SixAgainEvent {
    pub team: Team,
    pub minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorEvent {
    pub team: Team,
    pub minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DropGoalEvent {
    pub team: Team,
    pub player: String,
    pub minute: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Event {
    Try(TryEvent),
    Conversion(ConversionEvent),
    PenaltyAwarded(PenaltyAwardedEvent),
    PenaltyGoal(PenaltyGoalEvent),
    Card(CardEvent),
    SixAgain(SixAgainEvent),
    Error(ErrorEvent),
    DropGoal(DropGoalEvent),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MatchState {
    pub team_a: String,
    pub team_b: String,
    pub phase: Phase,
    pub tackles_a: u32,
    pub tackles_b: u32,
    pub sets_completed_a: u32,
    pub sets_completed_b: u32,
    pub sets_attempted_a: u32,
    pub sets_attempted_b: u32,
    pub events: Vec<Event>,
}

impl MatchState {
    pub fn new(team_a: String, team_b: String) -> Self {
        MatchState {
            team_a,
            team_b,
            phase: Phase::FirstHalf,
            tackles_a: 0,
            tackles_b: 0,
            sets_completed_a: 0,
            sets_completed_b: 0,
            sets_attempted_a: 0,
            sets_attempted_b: 0,
            events: Vec::new(),
        }
    }

    fn team_score(&self, team: Team) -> u32 {
        let mut score = 0;
        for event in &self.events {
            match event {
                Event::Try(t) if t.team == team => score += 4,
                Event::Conversion(c) if c.team == team && c.successful => score += 2,
                Event::PenaltyGoal(p) if p.team == team => score += 2,
                Event::DropGoal(d) if d.team == team => score += 1,
                _ => {}
            }
        }
        score
    }

    pub fn score_a(&self) -> u32 {
        self.team_score(Team::A)
    }

    pub fn score_b(&self) -> u32 {
        self.team_score(Team::B)
    }

    pub fn team_name(&self, team: Team) -> &str {
        match team {
            Team::A => &self.team_a,
            Team::B => &self.team_b,
        }
    }

    pub fn score(&self, team: Team) -> u32 {
        match team {
            Team::A => self.score_a(),
            Team::B => self.score_b(),
        }
    }

    pub fn tackles(&self, team: Team) -> u32 {
        match team {
            Team::A => self.tackles_a,
            Team::B => self.tackles_b,
        }
    }

    pub fn sets_completed(&self, team: Team) -> u32 {
        match team {
            Team::A => self.sets_completed_a,
            Team::B => self.sets_completed_b,
        }
    }

    pub fn sets_attempted(&self, team: Team) -> u32 {
        match team {
            Team::A => self.sets_attempted_a,
            Team::B => self.sets_attempted_b,
        }
    }

    pub fn sets_completion_pct(&self, team: Team) -> f64 {
        let attempts = self.sets_attempted(team);
        if attempts == 0 {
            0.0
        } else {
            self.sets_completed(team) as f64 / attempts as f64 * 100.0
        }
    }

    pub fn tries(&self, team: Team) -> Vec<&TryEvent> {
        self.events
            .iter()
            .filter_map(|e| match e {
                Event::Try(t) if t.team == team => Some(t),
                _ => None,
            })
            .collect()
    }

    pub fn conversions(&self, team: Team) -> Vec<&ConversionEvent> {
        self.events
            .iter()
            .filter_map(|e| match e {
                Event::Conversion(c) if c.team == team => Some(c),
                _ => None,
            })
            .collect()
    }

    pub fn penalties_against(&self, team: Team) -> u32 {
        self.events
            .iter()
            .filter(|e| matches!(e, Event::PenaltyAwarded(p) if p.team == team))
            .count() as u32
    }

    pub fn penalty_goals(&self, team: Team) -> Vec<&PenaltyGoalEvent> {
        self.events
            .iter()
            .filter_map(|e| match e {
                Event::PenaltyGoal(p) if p.team == team => Some(p),
                _ => None,
            })
            .collect()
    }

    pub fn cards(&self, team: Team) -> Vec<&CardEvent> {
        self.events
            .iter()
            .filter_map(|e| match e {
                Event::Card(c) if c.team == team => Some(c),
                _ => None,
            })
            .collect()
    }

    pub fn six_agains(&self, team: Team) -> u32 {
        self.events
            .iter()
            .filter(|e| matches!(e, Event::SixAgain(s) if s.team == team))
            .count() as u32
    }

    pub fn errors(&self, team: Team) -> u32 {
        self.events
            .iter()
            .filter(|e| matches!(e, Event::Error(e_) if e_.team == team))
            .count() as u32
    }

    pub fn drop_goals(&self, team: Team) -> Vec<&DropGoalEvent> {
        self.events
            .iter()
            .filter_map(|e| match e {
                Event::DropGoal(d) if d.team == team => Some(d),
                _ => None,
            })
            .collect()
    }

    pub fn recent_try(&self, team: Team) -> Option<usize> {
        self.events
            .iter()
            .enumerate()
            .rev()
            .find_map(|(i, e)| match e {
                Event::Try(t) if t.team == team => Some(i),
                _ => None,
            })
    }

    pub fn advance_phase(&mut self) {
        self.phase = match self.phase {
            Phase::FirstHalf => Phase::Halftime,
            Phase::Halftime => Phase::SecondHalf,
            Phase::SecondHalf => Phase::FullTime,
            Phase::FullTime => Phase::FullTime,
        };
    }
}

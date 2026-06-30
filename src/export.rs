use std::fs;
use std::path::PathBuf;

use chrono::Local;

use crate::models::MatchState;

pub fn save_to_json(state: &MatchState) -> Result<String, String> {
    let ts = Local::now().format("%Y%m%d_%H%M%S");
    let safe_a = sanitize(&state.team_a);
    let safe_b = sanitize(&state.team_b);
    let filename = format!("match_{}_v_{}_{}.json", safe_a, safe_b, ts);
    let path = PathBuf::from(&filename);

    let json = serde_json::to_string_pretty(state).map_err(|e| e.to_string())?;
    fs::write(&path, &json).map_err(|e| e.to_string())?;

    Ok(filename)
}

fn sanitize(name: &str) -> String {
    name.chars()
        .map(|c| if c.is_alphanumeric() || c == '-' || c == '_' { c } else { '_' })
        .collect()
}

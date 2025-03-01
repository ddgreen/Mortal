use riichi::agent::{Agent, SimpleRuleAgent};
use riichi::mjai::{Event, EventExt};
use riichi::state::PlayerState;

use anyhow::Result;
use std::io::{self, BufRead};
use std::process;
use serde_json as json;

fn main() -> Result<()> {
    // Create a new SimpleRuleAgent for player 0
    let mut agent = SimpleRuleAgent::new(0);
    
    // Create a new PlayerState for player 0
    let mut state = PlayerState::new(0);
    
    // Process mjai events from stdin
    let stdin = io::stdin();
    let mut log = Vec::new();
    
    for line in stdin.lock().lines() {
        let line = line?;
        if line.is_empty() {
            continue;
        }
        
        // Parse the mjai event
        let event: Event = match json::from_str(&line) {
            Ok(ev) => ev,
            Err(e) => {
                eprintln!("Failed to parse event: {}", e);
                continue;
            }
        };
        
        // Create an EventExt from the Event
        let event_ext = EventExt::no_meta(event.clone());
        
        // Update the log
        log.push(event_ext.clone());
        
        // Update the state
        let cans = match state.update(&event) {
            Ok(cans) => cans,
            Err(e) => {
                eprintln!("Failed to update state: {}", e);
                continue;
            }
        };
        
        // Check if the agent can act
        if cans.can_act() && event.actor().map_or(false, |a| a == agent.player_id) {
            // Get the agent's reaction
            let reaction = match agent.react(&log, &state, None) {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("Agent failed to react: {}", e);
                    continue;
                }
            };
            
            // Print the agent's reaction
            println!("{}", json::to_string(&reaction.event)?);
        }
        
        // Handle game and kyoku boundaries
        match event {
            Event::StartGame { .. } => {
                agent.start_game()?;
            },
            Event::EndKyoku => {
                agent.end_kyoku()?;
            },
            Event::EndGame => {
                break;
            },
            _ => {}
        }
    }
    
    Ok(())
}

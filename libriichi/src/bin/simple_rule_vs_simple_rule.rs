use riichi::agent::{BatchAgent, SimpleRuleAgent};
use riichi::arena::game::{BatchGame, Index};

use anyhow::Result;
use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::time::Instant;
use serde_json as json;

fn main() -> Result<()> {
    // Output directory for game logs
    let output_dir = PathBuf::from("game_logs");
    std::fs::create_dir_all(&output_dir)?;
    
    println!("Starting self-play with SimpleRuleAgent");
    println!("Will play 2 games and save logs to {:?}", output_dir);
    
    let start_time = Instant::now();
    
    // Create SimpleRuleAgent instances
    let mut agents = [
        Box::new(SimpleRuleAgent::new_batched(&[0, 1, 2, 3])?) as Box<dyn BatchAgent>,
        Box::new(SimpleRuleAgent::new_batched(&[3, 2, 1, 0])?) as Box<dyn BatchAgent>,
    ];
    
    // Create game indexes - same as in the tsumogiri test
    let indexes = &[
        [
            Index { agent_idx: 0, player_id_idx: 0 },
            Index { agent_idx: 0, player_id_idx: 1 },
            Index { agent_idx: 1, player_id_idx: 1 },
            Index { agent_idx: 1, player_id_idx: 0 },
        ],
        [
            Index { agent_idx: 1, player_id_idx: 3 },
            Index { agent_idx: 1, player_id_idx: 2 },
            Index { agent_idx: 0, player_id_idx: 2 },
            Index { agent_idx: 0, player_id_idx: 3 },
        ],
    ];
    
    // Run the games
    let game = BatchGame::tenhou_hanchan(false);
    let results = game.run(&mut agents, indexes, &[(1009, 0), (1021, 0)])?;
    
    // Get current timestamp for unique filenames
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)?
        .as_secs();
    
    // Save game logs
    for (i, result) in results.iter().enumerate() {
        let game_log_path = output_dir.join(format!("game_{}_{}.json", timestamp, i));
        let mut file = File::create(game_log_path)?;
        
        // Write each event in the game log to the file
        for kyoku_log in &result.game_log {
            for event in kyoku_log {
                writeln!(file, "{}", json::to_string(&event.event)?)?;
            }
        }
    }
    
    let elapsed = start_time.elapsed();
    println!("Completed {} games in {:.2?}", results.len(), elapsed);
    println!("Average time per game: {:.2?}", elapsed / results.len() as u32);
    
    // Print some statistics
    println!("\nGame Results:");
    println!("{:<5} {:<15} {:<10}", "Game", "Player", "Score");
    println!("{:-<30}", "");
    
    for (i, result) in results.iter().enumerate() {
        for (j, (name, score)) in result.names.iter().zip(result.scores.iter()).enumerate() {
            println!("{:<5} {:<15} {:<10}", i, format!("{}. {}", j, name), score);
        }
        println!("{:-<30}", "");
    }
    
    Ok(())
}

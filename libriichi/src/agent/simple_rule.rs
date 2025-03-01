use super::{Agent, BatchifiedAgent, InvisibleState};
use crate::mjai::{Event, EventExt};
use crate::state::PlayerState;
use crate::tile::Tile;

use anyhow::{Context, Result};

/// `SimpleRuleAgent` implements a basic rule-based strategy:
/// 1. Aims to reach tenpai and declare riichi as soon as possible
/// 2. Plays defensively when other players declare riichi
pub struct SimpleRuleAgent {
    pub player_id: u8,
    /// Tracks riichi status of all players
    pub others_riichi: [bool; 4],
}

impl SimpleRuleAgent {
    pub fn new(player_id: u8) -> Self {
        Self {
            player_id,
            others_riichi: [false; 4],
        }
    }

    pub fn new_batched(player_ids: &[u8]) -> Result<BatchifiedAgent<Self>> {
        BatchifiedAgent::new(|id| Ok(Self::new(id)), player_ids)
    }

    /// Checks if a tile is considered safe when other players have declared riichi
    fn is_safe_tile(&self, _state: &PlayerState, tile: Tile) -> bool {
        // For simplicity, we only consider honor tiles as safe
        tile.is_jihai()
    }

    /// Selects a tile to discard based on the current strategy
    fn select_discard_tile(&self, state: &PlayerState) -> Result<Tile> {
        let candidates = state.discard_candidates_aka();
        
        // Check if any other player has declared riichi
        let any_riichi = self.others_riichi.iter().any(|&r| r);
        
        if any_riichi {
            // Defensive strategy: prioritize safe tiles
            for (i, &can_discard) in candidates.iter().enumerate() {
                if !can_discard {
                    continue;
                }
                
                let tile = Tile::try_from(i as u8)?;
                if self.is_safe_tile(state, tile) {
                    return Ok(tile);
                }
            }
        }
        
        // If no safe tile found or no riichi declared, discard any valid tile
        
        // Otherwise, just discard any valid tile
        for (i, &can_discard) in candidates.iter().enumerate() {
            if can_discard {
                return Ok(Tile::try_from(i as u8)?);
            }
        }
        
        // Fallback to tsumogiri if nothing else works
        state.last_self_tsumo().context("No valid discard found")
    }

    /// Updates the riichi status of other players
    fn update_riichi_status(&mut self, log: &[EventExt]) {
        if let Some(event) = log.last() {
            match event.event {
                Event::ReachAccepted { actor } => {
                    if actor != self.player_id {
                        self.others_riichi[actor as usize] = true;
                    }
                },
                _ => {}
            }
        }
    }
}

impl Agent for SimpleRuleAgent {
    fn name(&self) -> String {
        "simple_rule_agent".to_owned()
    }

    fn start_game(&mut self) -> Result<()> {
        // Reset riichi status at the start of a game
        self.others_riichi = [false; 4];
        Ok(())
    }

    fn end_kyoku(&mut self) -> Result<()> {
        // Reset riichi status at the end of a round
        self.others_riichi = [false; 4];
        Ok(())
    }

    fn react(
        &mut self,
        log: &[EventExt],
        state: &PlayerState,
        _: Option<InvisibleState>,
    ) -> Result<EventExt> {
        // Update riichi status based on the log
        self.update_riichi_status(log);
        
        let cans = state.last_cans();
        
        // Handle different action possibilities in order of priority
        
        // 1. Agari (Ron or Tsumo) - always take it
        if cans.can_agari() {
            if state.rule_based_agari() {
                if cans.can_tsumo_agari {
                    return Ok(EventExt::no_meta(Event::Hora {
                        actor: self.player_id,
                        target: self.player_id,
                        deltas: None,
                        ura_markers: None,
                    }));
                } else if cans.can_ron_agari {
                    return Ok(EventExt::no_meta(Event::Hora {
                        actor: self.player_id,
                        target: cans.target_actor,
                        deltas: None,
                        ura_markers: None,
                    }));
                }
            }
        }
        
        // 2. Riichi - declare if at tenpai and not already in riichi
        if cans.can_riichi && !state.self_riichi_declared() {
            return Ok(EventExt::no_meta(Event::Reach {
                actor: self.player_id,
            }));
        }
        
        // 3. Discard a tile
        if cans.can_discard {
            let tile = self.select_discard_tile(state)?;
            let tsumogiri = state.last_self_tsumo().map_or(false, |t| t == tile);
            
            return Ok(EventExt::no_meta(Event::Dahai {
                actor: self.player_id,
                pai: tile,
                tsumogiri,
            }));
        }
        
        // 4. Ryukyoku - consider if available
        if cans.can_ryukyoku && state.rule_based_ryukyoku() {
            return Ok(EventExt::no_meta(Event::Ryukyoku {
                deltas: None,
            }));
        }
        
        // 5. For simplicity, we don't handle ankan/kakan in this basic agent
        
        // 7. For simplicity, we generally don't call chi/pon/daiminkan
        // This agent focuses on riichi strategy
        
        // Default: pass (do nothing)
        Ok(EventExt::no_meta(Event::None))
    }
}

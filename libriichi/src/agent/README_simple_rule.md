# SimpleRuleAgent

`SimpleRuleAgent` is a basic rule-based mahjong AI agent that implements a simple strategy:

1. Aims to reach tenpai and declare riichi as soon as possible
2. Plays defensively when other players declare riichi by selecting safe tiles

## Strategy Details

### Offensive Strategy
- Always tries to reduce shanten (improve the hand)
- Declares riichi immediately when reaching tenpai
- Always takes agari (ron/tsumo) when available

### Defensive Strategy
- When other players declare riichi, prioritizes discarding safe tiles:
  - Honor tiles (jihai)
  - Tiles that riichi players have already discarded

### Calling Strategy
- Generally avoids calling chi/pon to maintain the ability to riichi
- May perform ankan/kakan if it doesn't break tenpai

## Usage

### As a Standalone Agent

```rust
use riichi::agent::{Agent, SimpleRuleAgent};
use riichi::state::PlayerState;

// Create a new agent for player 0
let mut agent = SimpleRuleAgent::new(0);

// Create a player state
let mut state = PlayerState::new(0);

// Update state with game events...

// Get the agent's reaction
let reaction = agent.react(&log, &state, None)?;
```

### For Batch Processing

```rust
use riichi::agent::{BatchAgent, SimpleRuleAgent};

// Create a batched agent for multiple players
let player_ids = [0, 1, 2, 3];
let mut batched_agent = SimpleRuleAgent::new_batched(&player_ids)?;

// Use the batched agent for processing multiple games simultaneously
```

### Command Line Testing

The repository includes a simple command-line tool for testing the agent:

```bash
# Build the test binary
cargo build --bin simple_rule_agent_test

# Run with mjai format input
cat game_log.json | ./target/debug/simple_rule_agent_test
```

## Purpose

This agent is designed to serve as a baseline for reinforcement learning. It implements a simple but effective strategy that can be used to generate training data for more sophisticated learning-based agents.

The agent's behavior is deterministic and follows clear rules, making it suitable as a baseline for comparison with more advanced agents. It prioritizes:

1. Reaching tenpai quickly and declaring riichi
2. Playing defensively when other players declare riichi
3. Taking advantage of winning opportunities

## Integration with Mortal

To use this agent for self-play training with Mortal:

1. Create games between multiple instances of SimpleRuleAgent
2. Record the game logs in mjai format
3. Use these logs as initial training data for Mortal's reinforcement learning

This approach provides a solid foundation of basic gameplay before moving to more sophisticated strategies through reinforcement learning.

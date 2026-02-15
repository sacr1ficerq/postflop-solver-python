# OxiPostflop Python Bindings Coverage

This document provides a checklist of all methods and properties that have Python bindings implemented in OxiPostflop.

## Structs Accessible from Python

### DonkSizeOptions

Bet size options for the donk bets.

- [x] `__new__(donk_sizes: str)` - Create DonkSizeOptions from string
- [x] `__repr__()` - Representation
- [x] `__str__()` - String representation

### BetSizeOptions

Bet size options for the first bets and raises.

- [x] `__new__(bet_sizes: str, raise_sizes: str)` - Create BetSizeOptions from strings
- [x] `__repr__()` - Representation
- [x] `__str__()` - String representation

### BoardState

An enum representing the board state.

- [x] `__new__(initial_state: str)` - Create BoardState from string
- [x] `__repr__()` - Representation
- [x] `__str__()` - String representation

### CardConfig

A struct containing the card configuration.

- [x] `__new__(oop_range, ip_range, flop, turn=None, river=None)` - Create CardConfig
- [x] `__repr__()` - Representation
- [x] `__str__()` - String representation

### TreeConfig

A struct containing the game tree configuration.

- [x] `__new__(...)` - Create TreeConfig with all parameters
- [x] `__repr__()` - Representation
- [x] `__str__()` - String representation

### Action

Available actions of the postflop game.

- [x] `__new__(action: str)` - Create Action from string
- [x] `__repr__()` - Representation
- [x] `__str__()` - String representation

### PostFlopGame

A struct representing a postflop game.

**Constructor:**
- [x] `__new__(card_config, tree_config)` - Create PostFlopGame

**Navigation:**
- [x] `play(action: int)` - Play an action by index
- [x] `back_to_root()` - Move back to the root node
- [x] `history() -> list[int]` - Get action history from root
- [x] `apply_history(history: list[int])` - Apply a history of actions

**Node Info:**
- [x] `is_terminal_node() -> bool` - Check if current node is terminal
- [x] `is_chance_node() -> bool` - Check if current node is a chance node
- [x] `current_player() -> int` - Get current player (0=OOP, 1=IP)
- [x] `current_board() -> list[int]` - Get current board cards
- [x] `possible_cards() -> int` - Get possible cards as bit mask
- [x] `available_actions() -> list[Action]` - Get available actions

**Data Access:**
- [x] `private_cards(player: int) -> list[tuple[int, int]]` - Get private cards
- [x] `weights(player: int) -> list[float]` - Get raw weights
- [x] `normalized_weights(player: int) -> list[float]` - Get normalized weights
- [x] `equity(player: int) -> list[float]` - Get equity values
- [x] `expected_values(player: int) -> list[float]` - Get expected values
- [x] `expected_values_detail(player: int) -> list[float]` - Get detailed EV
- [x] `strategy() -> list[float]` - Get current strategy
- [x] `total_bet_amount() -> list[int]` - Get total bet amounts [OOP, IP]

**Configuration:**
- [x] `card_config() -> CardConfig` - Get card configuration
- [x] `tree_config() -> TreeConfig` - Get tree configuration
- [x] `memory_usage() -> tuple[int, int]` - Get memory usage
- [x] `is_memory_allocated() -> bool or None` - Check if memory allocated
- [x] `added_lines() -> list[list[Action]]` - Get added lines
- [x] `removed_lines() -> list[list[Action]]` - Get removed lines

**Computation:**
- [x] `cache_normalized_weights()` - Cache normalized weights
- [x] `allocate_memory(enable_compression: bool)` - Allocate memory
- [x] `lock_current_strategy(strategy: list[float])` - Lock current strategy
- [x] `unlock_current_strategy()` - Unlock current strategy
- [x] `current_locking_strategy() -> list[float] or None` - Get locking strategy

## Module-Level Functions

- [x] `holes_to_strings(holes) -> list[str]` - Convert card IDs to strings
- [x] `card_from_str(card: str) -> int` - Parse card string to ID
- [x] `flop_from_str(flop: str) -> list[int]` - Parse flop string to IDs
- [x] `card_to_string(card: int) -> str` - Convert card ID to string
- [x] `hole_to_string(hole) -> str` - Convert hole to string
- [x] `compute_average(values, weights) -> float` - Weighted average
- [x] `solve(game, max_iterations, target_exploitability, verbose) -> float` - Solve game
- [x] `compute_exploitability(game) -> float` - Compute exploitability
- [x] `compute_current_ev(game) -> list[float, float]` - Compute current EV
- [x] `compute_mes_ev(game) -> list[float, float]` - Compute MES EV
- [x] `finalize(game)` - Finalize solving process

## Not Implemented

The following are NOT implemented:

**PostFlopGame:**
- `set_bunching_effect()` - Requires BunchingData from postflop-solver
- `reset_bunching_effect()` - Requires BunchingData
- `remove_lines()` - Advanced feature
- `memory_usage_bunching()` - Advanced feature

**Other Modules:**
- `Range` class - Would need separate Python binding
- `BunchingData` class - Would need separate Python binding
- File I/O functions - Would require additional work
- Serialization (bincode) - Would require additional work

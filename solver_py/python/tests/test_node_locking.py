"""
Tests for node locking functionality adapted from postflop-solver examples.

Node locking allows fixing the strategy at certain nodes while solving for the rest of the tree.
This is useful for:
- Exploring how different opponent strategies affect optimal play
- Implementing mixed strategies with specific frequencies
- Analyzing game-theoretic properties
"""

import pytest
import numpy as np
from oxipostflop import (
    BoardState,
    BetSizeOptions,
    CardConfig,
    TreeConfig,
    PostFlopGame,
    solve,
)


class TestNodeLocking:
    """Tests for normal and partial node locking scenarios."""

    def test_normal_node_locking_first_scenario(self):
        """
        Test normal node locking with IP folding 25% and calling 75%.

        When IP calls 75% of the time, OOP should:
        - QQ: always check (can't value own QQ getting called by worse)
        - AA: always all-in (can value own AA getting called by worse)
        """
        card_config = CardConfig(
            oop_range="AsAh,QsQh",
            ip_range="KsKh",
            flop="2s3h4d",
            turn="6c",
            river="7c",
        )

        tree_config = TreeConfig(
            initial_state=BoardState("River"),
            starting_pot=20,
            effective_stack=10,
            flop_bet_sizes_oop=BetSizeOptions("a", ""),
            flop_bet_sizes_ip=BetSizeOptions("a", ""),
            turn_bet_sizes_oop=BetSizeOptions("a", ""),
            turn_bet_sizes_ip=BetSizeOptions("a", ""),
            river_bet_sizes_oop=BetSizeOptions("a", ""),
            river_bet_sizes_ip=BetSizeOptions("a", ""),
        )

        game = PostFlopGame(card_config, tree_config)
        # Node locking must be performed after allocating memory and before solving
        game.allocate_memory(False)

        # OOP goes all-in (action 1 is the all-in bet)
        game.play(1)
        # Lock IP's strategy: 25% fold, 75% call
        game.lock_current_strategy([0.25, 0.75])
        game.back_to_root()

        solve(game, 1000, 0.001, False)
        game.cache_normalized_weights()

        # Check OOP's strategy: [QQ check, AA check, QQ bet, AA bet]
        strategy_oop = game.strategy()
        assert abs(strategy_oop[0] - 1.0) < 1e-3  # QQ always check
        assert abs(strategy_oop[1] - 0.0) < 1e-3  # AA never check
        assert abs(strategy_oop[2] - 0.0) < 1e-3  # QQ never all-in
        assert abs(strategy_oop[3] - 1.0) < 1e-3  # AA always all-in

    def test_normal_node_locking_second_scenario(self):
        """
        Test normal node locking with IP folding 50% and calling 50%.

        When IP calls 50% of the time, OOP should:
        - QQ: always bet (mixed strategy becomes pure bet)
        - AA: always bet (always wants to get called by worse)
        """
        card_config = CardConfig(
            oop_range="AsAh,QsQh",
            ip_range="KsKh",
            flop="2s3h4d",
            turn="6c",
            river="7c",
        )

        tree_config = TreeConfig(
            initial_state=BoardState("River"),
            starting_pot=20,
            effective_stack=10,
            flop_bet_sizes_oop=BetSizeOptions("a", ""),
            flop_bet_sizes_ip=BetSizeOptions("a", ""),
            turn_bet_sizes_oop=BetSizeOptions("a", ""),
            turn_bet_sizes_ip=BetSizeOptions("a", ""),
            river_bet_sizes_oop=BetSizeOptions("a", ""),
            river_bet_sizes_ip=BetSizeOptions("a", ""),
        )

        game = PostFlopGame(card_config, tree_config)
        game.allocate_memory(False)

        # OOP goes all-in (action 1 is the all-in bet)
        game.play(1)
        # Lock IP's strategy: 50% fold, 50% call
        game.lock_current_strategy([0.5, 0.5])
        game.back_to_root()

        solve(game, 1000, 0.001, False)
        game.cache_normalized_weights()

        # Check OOP's strategy: [QQ check, AA check, QQ bet, AA bet]
        strategy_oop = game.strategy()
        assert abs(strategy_oop[0] - 0.0) < 1e-3  # QQ never check
        assert abs(strategy_oop[1] - 0.0) < 1e-3  # AA never check
        assert abs(strategy_oop[2] - 1.0) < 1e-3  # QQ always bet
        assert abs(strategy_oop[3] - 1.0) < 1e-3  # AA always bet

    def test_partial_node_locking(self):
        """
        Test partial node locking where only JJ is locked.

        Only JJ is locked to 80% check / 20% bet, while AA and QQ
        remain free to optimize their strategies.
        """
        card_config = CardConfig(
            oop_range="AsAh,QsQh,JsJh",
            ip_range="KsKh",
            flop="2s3h4d",
            turn="6c",
            river="7c",
        )

        tree_config = TreeConfig(
            initial_state=BoardState("River"),
            starting_pot=10,
            effective_stack=10,
            flop_bet_sizes_oop=BetSizeOptions("a", ""),
            flop_bet_sizes_ip=BetSizeOptions("a", ""),
            turn_bet_sizes_oop=BetSizeOptions("a", ""),
            turn_bet_sizes_ip=BetSizeOptions("a", ""),
            river_bet_sizes_oop=BetSizeOptions("a", ""),
            river_bet_sizes_ip=BetSizeOptions("a", ""),
        )

        game = PostFlopGame(card_config, tree_config)
        game.allocate_memory(False)

        # Lock OOP's strategy: only JJ is locked and the rest is not
        # Strategy array: [JJ check, QQ check, AA check, JJ bet, QQ bet, AA bet]
        # JJ: 80% check, 20% all-in
        game.lock_current_strategy([0.8, 0.0, 0.0, 0.2, 0.0, 0.0])

        solve(game, 1000, 0.001, False)
        game.cache_normalized_weights()

        # Check OOP's strategy
        # Strategy array is [JJ check, QQ check, AA check, JJ bet, QQ bet, AA bet]
        strategy_oop = game.strategy()
        assert abs(strategy_oop[0] - 0.8) < 1e-3  # JJ check 80% (locked)
        assert abs(strategy_oop[1] - 0.7) < 1e-3  # QQ check 70%
        assert abs(strategy_oop[2] - 0.0) < 1e-3  # AA never check
        assert abs(strategy_oop[3] - 0.2) < 1e-3  # JJ bet 20% (locked)
        assert abs(strategy_oop[4] - 0.3) < 1e-3  # QQ bet 30%
        assert abs(strategy_oop[5] - 1.0) < 1e-3  # AA always bet


class TestNodeLockingMethods:
    """Tests for node locking API methods."""

    def test_lock_and_unlock_strategy(self):
        """
        Test locking and unlocking a strategy.

        - lock_current_strategy should set the strategy at current node
        - current_locking_strategy should return the locked strategy
        - unlock_current_strategy should remove the lock
        """
        card_config = CardConfig(
            oop_range="AA,KK",
            ip_range="QQ",
            flop="2s3h4d",
            turn="6c",
            river="7c",
        )

        tree_config = TreeConfig(
            initial_state=BoardState("River"),
            starting_pot=20,
            effective_stack=10,
            flop_bet_sizes_oop=BetSizeOptions("a", ""),
            flop_bet_sizes_ip=BetSizeOptions("a", ""),
            turn_bet_sizes_oop=BetSizeOptions("a", ""),
            turn_bet_sizes_ip=BetSizeOptions("a", ""),
            river_bet_sizes_oop=BetSizeOptions("a", ""),
            river_bet_sizes_ip=BetSizeOptions("a", ""),
        )

        game = PostFlopGame(card_config, tree_config)
        game.allocate_memory(False)

        # Lock the strategy at current node
        game.lock_current_strategy([0.5, 0.5])
        locking_strategy = game.current_locking_strategy()
        assert locking_strategy is not None
        assert len(locking_strategy) == 2

        # Unlock the strategy
        game.unlock_current_strategy()
        assert game.current_locking_strategy() is None

    def test_play_after_locking(self):
        """
        Test navigating the game tree after locking.

        - play(action) moves to a child node
        - back_to_root returns to the root node
        - current_player returns the current player (0=OOP, 1=IP)
        """
        card_config = CardConfig(
            oop_range="AA,KK",
            ip_range="QQ",
            flop="2s3h4d",
            turn="6c",
            river="7c",
        )

        tree_config = TreeConfig(
            initial_state=BoardState("River"),
            starting_pot=20,
            effective_stack=10,
            flop_bet_sizes_oop=BetSizeOptions("a", ""),
            flop_bet_sizes_ip=BetSizeOptions("a", ""),
            turn_bet_sizes_oop=BetSizeOptions("a", ""),
            turn_bet_sizes_ip=BetSizeOptions("a", ""),
            river_bet_sizes_oop=BetSizeOptions("a", ""),
            river_bet_sizes_ip=BetSizeOptions("a", ""),
        )

        game = PostFlopGame(card_config, tree_config)
        game.allocate_memory(False)

        # At root, OOP (player 0) acts first
        assert game.current_player() == 0

        # Play action 1 (all-in)
        game.play(1)
        assert len(game.available_actions()) > 0

        # Go back to root
        game.back_to_root()
        assert game.current_player() == 0


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

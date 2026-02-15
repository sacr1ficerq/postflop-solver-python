"""
Node locking examples adapted from postflop-solver.

Node locking allows fixing the strategy at certain nodes while solving for the rest of the tree.
This is useful for:
- Exploring how different opponent strategies affect optimal play
- Implementing mixed strategies with specific frequencies
- Analyzing game-theoretic properties
"""

from oxipostflop import (
    BoardState,
    BetSizeOptions,
    CardConfig,
    TreeConfig,
    PostFlopGame,
    solve,
)


def normal_node_locking():
    """
    Demonstrates normal node locking where the opponent's strategy is locked
    at a specific node, and we analyze how OOP responds.

    Scenario:
    - OOP range: AsAh, QsQh (AA, QQ)
    - IP range: KsKh (KK)
    - Board: 2s3h4d 6c 7c (River)
    - OOP is all-in (no more betting)
    - We lock IP's strategy at the call/fold decision and see how OOP should play
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

    # OOP all-in (action 1 is all-in bet)
    game.play(1)
    # Lock IP's strategy: 25% fold, 75% call
    game.lock_current_strategy([0.25, 0.75])
    game.back_to_root()

    solve(game, 1000, 0.001, False)
    game.cache_normalized_weights()

    # Check OOP's strategy
    # Strategy array is [QQ check, AA check, QQ bet, AA bet]
    strategy_oop = game.strategy()
    assert abs(strategy_oop[0] - 1.0) < 1e-3  # QQ always check
    assert abs(strategy_oop[1] - 0.0) < 1e-3  # AA never check
    assert abs(strategy_oop[2] - 0.0) < 1e-3  # QQ never all-in
    assert abs(strategy_oop[3] - 1.0) < 1e-3  # AA always all-in

    # Now try with IP calling 50% of the time
    game.allocate_memory(False)
    game.play(1)
    # Lock IP's strategy: 50% fold, 50% call
    game.lock_current_strategy([0.5, 0.5])
    game.back_to_root()

    solve(game, 1000, 0.001, False)
    game.cache_normalized_weights()

    # Check OOP's strategy
    strategy_oop = game.strategy()
    assert abs(strategy_oop[0] - 0.0) < 1e-3  # QQ never check
    assert abs(strategy_oop[1] - 0.0) < 1e-3  # AA never check
    assert abs(strategy_oop[2] - 1.0) < 1e-3  # QQ always bet
    assert abs(strategy_oop[3] - 1.0) < 1e-3  # AA always bet


def partial_node_locking():
    """
    Demonstrates partial node locking where only specific hands are locked
    while others remain free to optimize.

    Scenario:
    - OOP range: AsAh, QsQh, JsJh (AA, QQ, JJ)
    - IP range: KsKh (KK)
    - Board: 2s3h4d 6c 7c (River)
    - Only JJ is locked (80% check, 20% all-in), AA and QQ are free to optimize
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


if __name__ == "__main__":
    normal_node_locking()
    partial_node_locking()
    print("All node locking tests passed!")

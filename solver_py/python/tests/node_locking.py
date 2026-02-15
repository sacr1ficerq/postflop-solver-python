from oxipostflop import (
    BoardState,
    BetSizeOptions,
    CardConfig,
    TreeConfig,
    PostFlopGame,
    solve,
)


def normal_node_locking():
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

    game.play(1)
    game.lock_current_strategy([0.25, 0.75])
    game.back_to_root()

    solve(game, 1000, 0.001, False)
    game.cache_normalized_weights()

    strategy_oop = game.strategy()
    assert abs(strategy_oop[0] - 1.0) < 1e-3
    assert abs(strategy_oop[1] - 0.0) < 1e-3
    assert abs(strategy_oop[2] - 0.0) < 1e-3
    assert abs(strategy_oop[3] - 1.0) < 1e-3

    game.allocate_memory(False)
    game.play(1)
    game.lock_current_strategy([0.5, 0.5])
    game.back_to_root()

    solve(game, 1000, 0.001, False)
    game.cache_normalized_weights()

    strategy_oop = game.strategy()
    assert abs(strategy_oop[0] - 0.0) < 1e-3
    assert abs(strategy_oop[1] - 0.0) < 1e-3
    assert abs(strategy_oop[2] - 1.0) < 1e-3
    assert abs(strategy_oop[3] - 1.0) < 1e-3


def partial_node_locking():
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

    game.lock_current_strategy([0.8, 0.0, 0.0, 0.2, 0.0, 0.0])

    solve(game, 1000, 0.001, False)
    game.cache_normalized_weights()

    strategy_oop = game.strategy()
    assert abs(strategy_oop[0] - 0.8) < 1e-3
    assert abs(strategy_oop[1] - 0.7) < 1e-3
    assert abs(strategy_oop[2] - 0.0) < 1e-3
    assert abs(strategy_oop[3] - 0.2) < 1e-3
    assert abs(strategy_oop[4] - 0.3) < 1e-3
    assert abs(strategy_oop[5] - 1.0) < 1e-3


if __name__ == "__main__":
    normal_node_locking()
    partial_node_locking()
    print("All node locking tests passed!")

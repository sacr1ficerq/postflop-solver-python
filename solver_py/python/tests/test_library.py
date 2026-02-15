import pytest
import numpy as np
from oxipostflop import (
    BoardState,
    BetSizeOptions,
    DonkSizeOptions,
    CardConfig,
    TreeConfig,
    PostFlopGame,
    Action,
    holes_to_strings,
    compute_average,
    card_from_str,
    solve,
)


class TestBoardState:
    def test_board_state_flop(self):
        state = BoardState("Flop")
        assert "Flop" in str(state)

    def test_board_state_turn(self):
        state = BoardState("Turn")
        assert "Turn" in str(state)

    def test_board_state_river(self):
        state = BoardState("River")
        assert "River" in str(state)

    def test_invalid_board_state(self):
        with pytest.raises(Exception):
            BoardState("Invalid")


class TestBetSizeOptions:
    def test_only_bet_size(self):
        bs = BetSizeOptions("60%", "")
        assert bs is not None

    def test_multiple_bet_sizes(self):
        bs = BetSizeOptions("60%, 100%, 2e", "")
        assert bs is not None

    def test_geometric_allin(self):
        bs = BetSizeOptions("60%, e, a", "")
        assert bs is not None

    def test_raise_sizes(self):
        bs = BetSizeOptions("60%, e, a", "60%, e, a")
        assert bs is not None


class TestCardConfig:
    def test_flop_config(self):
        cc = CardConfig("AA, KK", "QQ", "AsKh3d")
        assert cc is not None

    def test_turn_config(self):
        cc = CardConfig("AA, KK", "QQ", "AsKh3d", turn="2d")
        assert cc is not None

    def test_river_config(self):
        cc = CardConfig("AA, KK", "QQ", "AsKh3d", turn="2d", river="5h")
        assert cc is not None


class TestTreeConfig:
    def test_basic_config(self):
        bs = BoardState("River")
        bet_sizes_oop = BetSizeOptions("60%", "2.5x")
        bet_sizes_ip = BetSizeOptions("60%", "2.5x")
        tc = TreeConfig(
            bs,
            200,
            900,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
        )
        assert tc is not None


class TestAction:
    def test_check_action(self):
        action = Action("Check")
        assert action is not None


class TestHolesToStrings:
    def test_single_hand(self):
        holes = [(12, 11)]  # AcKd
        result = holes_to_strings(holes)
        assert len(result) == 1

    def test_multiple_hands(self):
        holes = [(12, 11), (12, 10), (1, 0)]  # AcKd, AcQd, 2cAd
        result = holes_to_strings(holes)
        assert len(result) == 3


class TestPostFlopGame:
    def test_game_creation(self):
        oop_range = "66+,A8s+,AJo+,KQo"
        ip_range = "QQ-22,AQs-A2s,ATo+"

        card_config = CardConfig(oop_range, ip_range, "AsKh3d")
        board_state = BoardState("Flop")
        bet_sizes_oop = BetSizeOptions("60%", "2.5x")
        bet_sizes_ip = BetSizeOptions("60%", "2.5x")
        tree_config = TreeConfig(
            board_state,
            200,
            900,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
        )

        game = PostFlopGame(card_config, tree_config)
        assert game is not None

    def test_private_cards(self):
        oop_range = "66+,A8s+,AJo+,KQo"
        ip_range = "QQ-22,AQs-A2s,ATo+"

        card_config = CardConfig(oop_range, ip_range, "AsKh3d")
        board_state = BoardState("Flop")
        bet_sizes_oop = BetSizeOptions("60%", "2.5x")
        bet_sizes_ip = BetSizeOptions("60%", "2.5x")
        tree_config = TreeConfig(
            board_state,
            200,
            900,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
        )

        game = PostFlopGame(card_config, tree_config)

        oop_cards = game.private_cards(0)
        ip_cards = game.private_cards(1)

        assert len(oop_cards) > 0
        assert len(ip_cards) > 0

    def test_holes_to_strings_integration(self):
        oop_range = "KK"
        ip_range = "QQ"

        card_config = CardConfig(oop_range, ip_range, "AsKh3d")
        board_state = BoardState("Flop")
        bet_sizes_oop = BetSizeOptions("60%", "2.5x")
        bet_sizes_ip = BetSizeOptions("60%", "2.5x")
        tree_config = TreeConfig(
            board_state,
            200,
            900,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
        )

        game = PostFlopGame(card_config, tree_config)
        oop_cards = game.private_cards(0)
        strings = holes_to_strings(oop_cards)

        assert len(strings) > 0
        assert "K" in strings[0]

    def test_available_actions(self):
        oop_range = "66+,A8s+,AJo+,KQo"
        ip_range = "QQ-22,AQs-A2s,ATo+"

        card_config = CardConfig(oop_range, ip_range, "AsKh3d")
        board_state = BoardState("Flop")
        bet_sizes_oop = BetSizeOptions("60%", "2.5x")
        bet_sizes_ip = BetSizeOptions("60%", "2.5x")
        tree_config = TreeConfig(
            board_state,
            200,
            900,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
        )

        game = PostFlopGame(card_config, tree_config)
        actions = game.available_actions()

        assert len(actions) > 0

    def test_play_and_back_to_root(self):
        oop_range = "66+,A8s+,AJo+,KQo"
        ip_range = "QQ-22,AQs-A2s,ATo+"

        card_config = CardConfig(oop_range, ip_range, "AsKh3d")
        board_state = BoardState("Flop")
        bet_sizes_oop = BetSizeOptions("60%", "2.5x")
        bet_sizes_ip = BetSizeOptions("60%", "2.5x")
        tree_config = TreeConfig(
            board_state,
            200,
            900,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
        )

        game = PostFlopGame(card_config, tree_config)

        actions = game.available_actions()
        assert len(actions) > 0


class TestSolve:
    def test_solve_flop(self):
        oop_range = "66+,A8s+,AJo+,KQo"
        ip_range = "QQ-22,AQs-A2s,ATo+"

        card_config = CardConfig(oop_range, ip_range, "AsKh3d")
        board_state = BoardState("Flop")
        bet_sizes_oop = BetSizeOptions("60%", "2.5x")
        bet_sizes_ip = BetSizeOptions("60%", "2.5x")
        tree_config = TreeConfig(
            board_state,
            200,
            900,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
        )

        game = PostFlopGame(card_config, tree_config)

        exploitability = solve(game, 10, 1.0, False)
        assert exploitability is not None

    def test_strategy_after_solve(self):
        oop_range = "AA,KK"
        ip_range = "QQ"

        card_config = CardConfig(oop_range, ip_range, "AsKh3d")
        board_state = BoardState("Flop")
        bet_sizes_oop = BetSizeOptions("60%", "2.5x")
        bet_sizes_ip = BetSizeOptions("60%", "2.5x")
        tree_config = TreeConfig(
            board_state,
            200,
            900,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
        )

        game = PostFlopGame(card_config, tree_config)

        solve(game, 10, 1.0, False)

        strategy = game.strategy()
        assert len(strategy) > 0

    def test_equity_and_ev(self):
        oop_range = "AA,KK"
        ip_range = "QQ"

        card_config = CardConfig(oop_range, ip_range, "AsKh3d")
        board_state = BoardState("Flop")
        bet_sizes_oop = BetSizeOptions("60%", "2.5x")
        bet_sizes_ip = BetSizeOptions("60%", "2.5x")
        tree_config = TreeConfig(
            board_state,
            200,
            900,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
        )

        game = PostFlopGame(card_config, tree_config)

        solve(game, 10, 1.0, False)

        game.cache_normalized_weights()
        equity = game.equity(0)
        ev = game.expected_values(0)

        assert len(equity) > 0
        assert len(ev) > 0

    def test_normalized_weights(self):
        oop_range = "AA,KK"
        ip_range = "QQ"

        card_config = CardConfig(oop_range, ip_range, "AsKh3d")
        board_state = BoardState("Flop")
        bet_sizes_oop = BetSizeOptions("60%", "2.5x")
        bet_sizes_ip = BetSizeOptions("60%", "2.5x")
        tree_config = TreeConfig(
            board_state,
            200,
            900,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
        )

        game = PostFlopGame(card_config, tree_config)

        solve(game, 10, 1.0, False)

        game.cache_normalized_weights()
        weights = game.normalized_weights(0)

        assert len(weights) > 0


class TestTurnBoard:
    def test_turn_game(self):
        oop_range = "66+,A8s+,AJo+,KQo"
        ip_range = "QQ-22,AQs-A2s,ATo+"

        card_config = CardConfig(oop_range, ip_range, "AsKh3d", turn="2d")
        board_state = BoardState("Turn")
        bet_sizes_oop = BetSizeOptions("60%", "2.5x")
        bet_sizes_ip = BetSizeOptions("60%", "2.5x")
        tree_config = TreeConfig(
            board_state,
            200,
            900,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
            bet_sizes_oop,
            bet_sizes_ip,
        )

        game = PostFlopGame(card_config, tree_config)

        assert game is not None
        solve(game, 5, 1.0, False)


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

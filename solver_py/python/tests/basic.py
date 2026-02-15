import pytest
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


oop_range = "66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs,96s+,85s+,75s+,65s,54s"
ip_range = (
    "QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+,T7s+,96s+,86s+,75s+,64s+,53s+"
)

card_config = CardConfig(oop_range, ip_range, "Td9d6h", turn="Qc")

board_state = BoardState("Turn")

flop_bet_sizes_oop = BetSizeOptions("60%, e, a", "2.5x")
flop_bet_sizes_ip = BetSizeOptions("50%, e, a", "2.5x")
turn_bet_sizes_oop = BetSizeOptions("70%, e, a", "2.5x")
turn_bet_sizes_ip = BetSizeOptions("80%, e, a", "2.5x")
river_bet_sizes_oop = BetSizeOptions("40%, e, a", "2.5x")
river_bet_sizes_ip = BetSizeOptions("90%, e, a", "2.5x")

river_donk_sizes = DonkSizeOptions("50%")

tree_config = TreeConfig(
    board_state,
    200,
    900,
    flop_bet_sizes_oop,
    flop_bet_sizes_ip,
    turn_bet_sizes_oop,
    turn_bet_sizes_ip,
    river_bet_sizes_oop,
    river_bet_sizes_ip,
    turn_donk_sizes=None,
    river_donk_sizes=river_donk_sizes,
    add_allin_threshold=1.5,
    force_allin_threshold=0.15,
    merging_threshold=0.1,
)

game = PostFlopGame(card_config, tree_config)

oop_cards = game.private_cards(0)
oop_cards_str = holes_to_strings(oop_cards)
assert oop_cards_str[0] == "5c4c"

mem_usage, mem_usage_compressed = game.memory_usage()
print(f"Memory usage without compression: {mem_usage / (1024**3):.2f} GB")
print(
    f"Memory usage with compression: {mem_usage_compressed / (1024**3):.2f} GB"
)

max_num_iterations = 10
target_exploitability = 200 * 0.005
exploitability = solve(game, max_num_iterations, target_exploitability, True)
print(f"Exploitability: {exploitability:.2f}")

game.cache_normalized_weights()
equity = game.equity(0)
ev = game.expected_values(0)
print(f"Equity of oop_hands[0]: {100.0 * equity[0]:.2f}%")
print(f"EV of oop_hands[0]: {ev[0]:.2f}")

weights = game.normalized_weights(0)
average_equity = compute_average(equity, weights)
average_ev = compute_average(ev, weights)
print(f"Average equity: {100.0 * average_equity:.2f}%")
print(f"Average EV: {average_ev:.2f}")

actions = game.available_actions()
print(f"Available actions (OOP): {actions}")
assert len(actions) > 0

game.play(1)

actions = game.available_actions()
print(f"Available actions (IP): {actions}")
assert len(actions) > 0

ip_cards = game.private_cards(1)
strategy = game.strategy()
print(f"IP cards count: {len(ip_cards)}")
print(f"Strategy length: {len(strategy)}")
assert len(ip_cards) == 250
assert len(strategy) == 750

ksjs_idx = holes_to_strings(ip_cards).index("KsJs")
assert strategy[ksjs_idx] == 0.0
assert (
    abs(
        strategy[ksjs_idx]
        + strategy[ksjs_idx + 250]
        + strategy[ksjs_idx + 500]
        - 1.0
    )
    < 1e-6
)

game.play(1)

assert game.is_chance_node() == True

card_7s = card_from_str("7s")
possible = game.possible_cards()
assert possible & (1 << card_7s) != 0

game.play(card_7s)

game.back_to_root()

print("All basic example tests passed!")

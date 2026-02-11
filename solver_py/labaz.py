from oxipostflop import *

board_state = BoardState("River")
bet_sizes = BetSizeOptions("60%", "2.5x")

card_config = CardConfig("TT+", "55+", "AsKh3d", "Ad", "2h")

tree_config = TreeConfig(board_state,
                         200,
                         900,
                         bet_sizes)

game = PostFlopGame(card_config, tree_config)

solve(game, 100, 0.1, True)

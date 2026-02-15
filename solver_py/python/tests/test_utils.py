import pytest
import numpy as np
import io
import sys
from oxipostflop.utils import (
    card_to_rank,
    get_matrix_indices,
    create_hands_matrix,
    get_average_weight_matrix,
    normalize_weights,
    print_matrix,
    matrix_to_string,
    RANK_CHARS,
)


class TestCardToRank:
    def test_card_to_rank_2(self):
        assert card_to_rank(0) == 0
        assert card_to_rank(1) == 0
        assert card_to_rank(2) == 0
        assert card_to_rank(3) == 0

    def test_card_to_rank_a(self):
        assert card_to_rank(48) == 12
        assert card_to_rank(49) == 12
        assert card_to_rank(50) == 12
        assert card_to_rank(51) == 12

    def test_card_to_rank_k(self):
        assert card_to_rank(44) == 11
        assert card_to_rank(45) == 11
        assert card_to_rank(46) == 11
        assert card_to_rank(47) == 11


class TestGetMatrixIndices:
    def test_pair(self):
        assert get_matrix_indices(0, 1) == (0, 0)
        assert get_matrix_indices(51, 50) == (12, 12)

    def test_suited_ak(self):
        assert get_matrix_indices(51, 47) == (12, 11)

    def test_offsuit_ak(self):
        assert get_matrix_indices(51, 44) == (12, 11)

    def test_ak_reversed(self):
        assert get_matrix_indices(44, 51) == (12, 11)

    def test_connector(self):
        assert get_matrix_indices(0, 7) == (1, 0)


class TestCreateHandsMatrix:
    def test_empty_input(self):
        result = create_hands_matrix([], [], [], [])
        assert len(result) == 13
        assert all(len(row) == 13 for row in result)

    def test_single_hand(self):
        private_cards = [(51, 47)]
        weights = [1.0]
        strategy = [0.5, 0.5]
        actions = ["Check", "Bet"]

        result = create_hands_matrix(private_cards, weights, strategy, actions)

        assert len(result[12][11]) == 1
        hand = result[12][11][0]
        assert hand["weight"] == 1.0
        assert hand["Check"] == 0.5
        assert hand["Bet"] == 0.5

    def test_multiple_hands_same_cell(self):
        private_cards = [(51, 47), (50, 46), (49, 45)]
        weights = [1.0, 0.8, 0.6]
        strategy = [0.5, 0.5, 0.3, 0.7, 0.4, 0.6]
        actions = ["Check", "Bet"]

        result = create_hands_matrix(private_cards, weights, strategy, actions)

        assert len(result[12][11]) == 3
        assert result[12][11][0]["weight"] == 1.0
        assert result[12][11][1]["weight"] == 0.8
        assert result[12][11][2]["weight"] == 0.6

    def test_hands_across_different_cells(self):
        private_cards = [(51, 47), (51, 40), (0, 0)]
        weights = [1.0, 0.5, 0.3]
        strategy = [0.5, 0.5, 0.3, 0.7, 0.4, 0.6]
        actions = ["Check", "Bet"]

        result = create_hands_matrix(private_cards, weights, strategy, actions)

        assert len(result[12][11]) == 1
        assert len(result[12][10]) == 1
        assert len(result[0][0]) == 1


class TestGetAverageWeightMatrix:
    def test_empty_matrix(self):
        hands_matrix = [[[] for _ in range(13)] for _ in range(13)]
        result = get_average_weight_matrix(hands_matrix)
        assert result.shape == (13, 13)
        assert np.all(result == 0)

    def test_single_hand(self):
        hands_matrix = [[[] for _ in range(13)] for _ in range(13)]
        hands_matrix[12][11] = [{"weight": 1.0}]

        result = get_average_weight_matrix(hands_matrix)

        assert result[12, 11] == 1.0

    def test_multiple_hands_same_cell(self):
        hands_matrix = [[[] for _ in range(13)] for _ in range(13)]
        hands_matrix[12][11] = [{"weight": 1.0}, {"weight": 0.5}]

        result = get_average_weight_matrix(hands_matrix)

        assert result[12, 11] == 0.75


class TestNormalizeWeights:
    def test_empty_input(self):
        result = normalize_weights([])
        assert result == []

    def test_single_weight(self):
        result = normalize_weights([1.0])
        assert result == [1.0]

    def test_multiple_weights(self):
        result = normalize_weights([1.0, 2.0, 0.5])
        assert result == [0.5, 1.0, 0.25]

    def test_all_zero(self):
        result = normalize_weights([0.0, 0.0, 0.0])
        assert result == [0.0, 0.0, 0.0]

    def test_unnormalized_input(self):
        result = normalize_weights([100.0, 200.0])
        assert result == [0.5, 1.0]


class TestMatrixToString:
    def test_empty_matrix(self):
        matrix = np.zeros((13, 13))
        result = matrix_to_string(matrix)
        lines = result.split("\n")
        assert len(lines) == 13
        expected_line = ". " * 12 + "."
        assert all(line == expected_line for line in lines)

    def test_matrix_with_values(self):
        matrix = np.zeros((13, 13))
        matrix[0, 0] = 1.0
        matrix[12, 12] = 0.5

        result = matrix_to_string(matrix)
        lines = result.split("\n")

        assert "1.000" in lines[0]
        assert "0.500" in lines[12]


class TestPrintMatrix:
    def test_print_matrix_with_values(self):
        matrix = np.zeros((13, 13))
        matrix[12, 11] = 0.5
        matrix[0, 0] = 1.0

        captured = io.StringIO()
        sys.stdout = captured
        print_matrix(matrix)
        sys.stdout = sys.__stdout__

        output = captured.getvalue()
        assert "A|" in output
        assert "2|" in output
        assert "0.500" in output
        assert "1.000" in output


class TestIntegration:
    def test_full_workflow(self):
        private_cards = [
            (51, 47),
            (50, 46),
            (49, 45),
            (44, 47),
            (0, 3),
            (0, 0),
        ]
        weights = [1.0, 0.8, 0.6, 0.9, 0.5, 0.3]
        strategy = [0.5] * (len(private_cards) * 2)
        actions = ["Check", "Bet"]

        hands_matrix = create_hands_matrix(private_cards, weights, strategy, actions)
        avg_weights = get_average_weight_matrix(hands_matrix)

        assert hands_matrix[12][11][0]["weight"] == 1.0
        assert avg_weights[12, 11] > 0


if __name__ == "__main__":
    pytest.main([__file__, "-v"])

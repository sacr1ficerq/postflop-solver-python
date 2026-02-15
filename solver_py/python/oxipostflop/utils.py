import numpy as np
from typing import List, Dict, Any, Tuple


RANK_CHARS = ["2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A"]


def card_to_rank(card: int) -> int:
    return card // 4


def get_matrix_indices(card1: int, card2: int) -> Tuple[int, int]:
    rank1 = card_to_rank(card1)
    rank2 = card_to_rank(card2)
    high_rank = max(rank1, rank2)
    low_rank = min(rank1, rank2)
    return (high_rank, low_rank)


def create_hands_matrix(
    private_cards: List[Tuple[int, int]],
    weights: List[float],
    strategy: List[float],
    actions: List[str],
) -> List[List[List[Dict[str, Any]]]]:
    matrix = [[[] for _ in range(13)] for _ in range(13)]

    num_actions = len(actions)

    for hand_idx, (cards, weight) in enumerate(zip(private_cards, weights)):
        card1, card2 = cards
        high_rank, low_rank = get_matrix_indices(card1, card2)

        hand_dict: Dict[str, Any] = {
            "weight": weight,
        }

        for action_idx, action_name in enumerate(actions):
            strategy_idx = hand_idx * num_actions + action_idx
            if strategy_idx < len(strategy):
                hand_dict[action_name] = strategy[strategy_idx]

        matrix[high_rank][low_rank].append(hand_dict)

    return matrix


def get_average_weight_matrix(
    hands_matrix: List[List[List[Dict[str, Any]]]],
) -> np.ndarray:
    result = np.zeros((13, 13), dtype=np.float64)

    for i in range(13):
        for j in range(13):
            cell = hands_matrix[i][j]
            if cell:
                weights = [hand["weight"] for hand in cell]
                result[i, j] = np.mean(weights)

    return result


def normalize_weights(weights: List[float]) -> List[float]:
    if not len(weights):
        return []
    max_weight = max(weights)
    if max_weight == 0:
        return [0.0] * len(weights)
    return [w / max_weight for w in weights]


def matrix_to_string(matrix: np.ndarray, precision: int = 3) -> str:
    lines = []
    rows, cols = matrix.shape
    for i in range(rows):
        row_vals = [
            f"{matrix[i, j]:.{precision}f}" if matrix[i, j] > 0 else "."
            for j in range(cols)
        ]
        lines.append(" ".join(row_vals))
    return "\n".join(lines)


def print_matrix(matrix: np.ndarray, precision: int = 3) -> None:
    rows, cols = matrix.shape
    header = "   " + precision * " " + (" " * (precision+1) + " ").join(RANK_CHARS)
    print(header)
    print("  " + "-" * (len(header) - 1))

    filler = {
        1: " . ",
        2: "  . ",
        3: "  .  ",
    }[precision]

    for i in range(rows):
        row_vals = [
            f"{matrix[i, j]:.{precision}f}" if matrix[i, j] > 0 else filler
            for j in range(cols)
        ]
        print(f"{RANK_CHARS[i]}| " + " ".join(row_vals))


if __name__ == "__main__":
    matrix = np.zeros((13, 13), dtype=np.float64)
    matrix[0, 0] = 0.5
    matrix[1, 4] = 0.5
    matrix[3, 5] = 0.5
    matrix[2, 4] = 0.5
    matrix[9, 1] = 0.5
    matrix[9, 2] = 0.5
    matrix[9, 3] = 0.5
    matrix[4, 12] = 0.5
    matrix[7, 10] = 0.5
    print_matrix(matrix, precision=3)

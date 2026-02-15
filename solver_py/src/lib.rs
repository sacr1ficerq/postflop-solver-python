#![allow(unsafe_op_in_unsafe_fn)]
use pyo3::prelude::*;

#[pymodule]
mod oxipostflop {
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;

    use ndarray::Array1;
    use numpy::IntoPyArray;
    use numpy::PyArray1;

    /// Bet size options for the first bets and raises.
    ///
    /// Multiple bet sizes can be specified using a comma-separated string.
    /// Each element must be a string ending in one of the following characters: %, x, c, r, e, a.
    ///
    /// - %: Percentage of the pot. (e.g., "70%")
    /// - x: Multiple of the previous bet. Valid for only raises. (e.g., "2.5x")
    /// - c: Constant value. Must be an integer. (e.g., "100c")
    /// - c + r: Constant value with raise cap (for FLHE). Both values must be integers.
    ///          Valid only for raises. (e.g., "20c3r")
    /// - e: Geometric size.
    ///   - e: Same as "3e" for the flop, "2e" for the turn, and "1e" (equivalent to "a") for the river.
    ///   - Xe: The geometric size with X streets remaining. X must be a positive integer. (e.g., "2e")
    ///   - XeY%: Same as Xe, but the maximum size is Y% of the pot. (e.g., "3e200%")
    ///   - If specified for raises, the number of previous raises is subtracted from X.
    /// - a: All-in. (e.g., "a")
    #[derive(Clone)]
    #[pyclass]
    pub struct DonkSizeOptions(postflop_solver::DonkSizeOptions);

    #[pymethods]
    impl DonkSizeOptions {
        #[new]
        fn new(donk_sizes: &str) -> PyResult<Self> {
            postflop_solver::DonkSizeOptions::try_from(donk_sizes)
                .map(Self)
                .map_err(|e| PyValueError::new_err(e))
        }

        fn __repr__(&self) -> String {
            format!("{:#?}", self.0)
        }

        fn __str__(&self) -> String {
            format!("{:#?}", self.0)
        }
    }

    /// Bet size options for the first bets and raises.
    ///
    /// Multiple bet sizes can be specified using a comma-separated string.
    /// Each element must be a string ending in one of the following characters: %, x, c, r, e, a.
    ///
    /// - %: Percentage of the pot. (e.g., "70%")
    /// - x: Multiple of the previous bet. Valid for only raises. (e.g., "2.5x")
    /// - c: Constant value. Must be an integer. (e.g., "100c")
    /// - c + r: Constant value with raise cap (for FLHE). Both values must be integers.
    ///          Valid only for raises. (e.g., "20c3r")
    /// - e: Geometric size.
    ///   - e: Same as "3e" for the flop, "2e" for the turn, and "1e" (equivalent to "a") for the river.
    ///   - Xe: The geometric size with X streets remaining. X must be a positive integer. (e.g., "2e")
    ///   - XeY%: Same as Xe, but the maximum size is Y% of the pot. (e.g., "3e200%")
    ///   - If specified for raises, the number of previous raises is subtracted from X.
    /// - a: All-in. (e.g., "a")
    #[derive(Clone)]
    #[pyclass]
    pub struct BetSizeOptions(postflop_solver::BetSizeOptions);

    #[pymethods]
    impl BetSizeOptions {
        #[new]
        fn new(bet_sizes: &str, raise_sizes: &str) -> PyResult<Self> {
            postflop_solver::BetSizeOptions::try_from((bet_sizes, raise_sizes))
                .map(Self)
                .map_err(|e| PyValueError::new_err(e))
        }

        fn __repr__(&self) -> String {
            format!("{:#?}", self.0)
        }

        fn __str__(&self) -> String {
            format!("{:#?}", self.0)
        }
    }

    /// An enum representing the board state.
    ///
    /// The board state determines which betting rounds are included in the game tree.
    #[pyclass]
    pub struct BoardState(postflop_solver::BoardState);

    fn get_board_state(str_state: &str) -> Result<postflop_solver::BoardState, String> {
        match str_state {
            "Flop" => Ok(postflop_solver::BoardState::Flop),
            "Turn" => Ok(postflop_solver::BoardState::Turn),
            "River" => Ok(postflop_solver::BoardState::River),
            _ => Err(format!("invalid board state: {}", str_state)),
        }
    }

    #[pymethods]
    impl BoardState {
        #[new]
        fn new(initial_state: &str) -> PyResult<Self> {
            get_board_state(initial_state)
                .map(Self)
                .map_err(|e| PyValueError::new_err(e))
        }

        fn __repr__(&self) -> String {
            format!("{:#?}", self.0)
        }

        fn __str__(&self) -> String {
            format!("{:#?}", self.0)
        }
    }

    /// A struct containing the card configuration.
    ///
    /// Parameters
    /// ----------
    /// oop_range : str
    ///     The range of the out-of-position (OOP) player as a string.
    /// ip_range : str
    ///     The range of the in-position (IP) player as a string.
    /// flop : str
    ///     The flop cards as a string (e.g., "AsKh3d").
    /// turn : str, optional
    ///     The turn card as a string (e.g., "2d").
    /// river : str, optional
    ///     The river card as a string (e.g., "5h").
    ///
    /// Examples
    /// --------
    /// >>> CardConfig("AA, KK", "QQ", "AsKh3d")
    /// >>> CardConfig("AA, KK", "QQ", "AsKh3d", turn="2d")
    /// >>> CardConfig("AA, KK", "QQ", "AsKh3d", turn="2d", river="5h")
    #[pyclass]
    pub struct CardConfig(postflop_solver::CardConfig);

    #[pymethods]
    impl CardConfig {
        #[new]
        #[pyo3(signature = (oop_range, ip_range, flop, turn=None, river=None))]
        fn new(
            oop_range: &str,
            ip_range: &str,
            flop: &str,
            turn: Option<&str>,
            river: Option<&str>,
        ) -> PyResult<Self> {
            let oop_range_instance = oop_range
                .parse::<postflop_solver::Range>()
                .map_err(|e| PyValueError::new_err(e))?;
            let ip_range_instance = ip_range
                .parse::<postflop_solver::Range>()
                .map_err(|e| PyValueError::new_err(e))?;

            let flop_instance =
                postflop_solver::flop_from_str(flop).map_err(|e| PyValueError::new_err(e))?;

            let turn_instance = turn.map_or(Ok(postflop_solver::NOT_DEALT), |turn| {
                postflop_solver::card_from_str(turn).map_err(|e| PyValueError::new_err(e))
            })?;

            let river_instance = river.map_or(Ok(postflop_solver::NOT_DEALT), |river| {
                postflop_solver::card_from_str(river).map_err(|e| PyValueError::new_err(e))
            })?;

            Ok(Self(postflop_solver::CardConfig {
                range: [oop_range_instance, ip_range_instance],
                flop: flop_instance,
                turn: turn_instance,
                river: river_instance,
            }))
        }

        fn __repr__(&self) -> String {
            format!("{:#?}", self.0)
        }

        fn __str__(&self) -> String {
            format!("{:#?}", self.0)
        }
    }

    /// A struct containing the game tree configuration.
    ///
    /// Parameters
    /// ----------
    /// initial_state : BoardState
    ///     The initial board state (Flop, Turn, or River).
    /// starting_pot : int
    ///     The starting pot size in chips.
    /// effective_stack : int
    ///     The effective stack size in chips.
    /// flop_bet_sizes_oop : BetSizeOptions
    ///     Bet size options for OOP on the flop.
    /// flop_bet_sizes_ip : BetSizeOptions
    ///     Bet size options for IP on the flop.
    /// turn_bet_sizes_oop : BetSizeOptions
    ///     Bet size options for OOP on the turn.
    /// turn_bet_sizes_ip : BetSizeOptions
    ///     Bet size options for IP on the turn.
    /// river_bet_sizes_oop : BetSizeOptions
    ///     Bet size options for OOP on the river.
    /// river_bet_sizes_ip : BetSizeOptions
    ///     Bet size options for IP on the river.
    /// turn_donk_sizes : DonkSizeOptions, optional
    ///     Donk size options for the turn.
    /// river_donk_sizes : DonkSizeOptions, optional
    ///     Donk size options for the river.
    /// add_allin_threshold : float, optional
    ///     The threshold for adding all-in as a bet size option. Default is 1.5.
    /// force_allin_threshold : float, optional
    ///     The threshold for forcing all-in as the only option. Default is 0.15.
    /// merging_threshold : float, optional
    ///     The threshold for merging bet sizes. Default is 0.1.
    ///
    /// Examples
    /// --------
    /// >>> bet_sizes = BetSizeOptions("60%", "2.5x")
    /// >>> donk_sizes = DonkSizeOptions("50%")
    /// >>> TreeConfig(BoardState("Flop"), 200, 900, bet_sizes, bet_sizes, bet_sizes, bet_sizes, bet_sizes, bet_sizes)
    #[pyclass]
    pub struct TreeConfig(postflop_solver::TreeConfig);

    #[pymethods]
    impl TreeConfig {
        #[new]
        #[pyo3(signature = (
            initial_state,
            starting_pot,
            effective_stack,
            flop_bet_sizes_oop,
            flop_bet_sizes_ip,
            turn_bet_sizes_oop,
            turn_bet_sizes_ip,
            river_bet_sizes_oop,
            river_bet_sizes_ip,
            turn_donk_sizes = None,
            river_donk_sizes = None,
            add_allin_threshold = 1.5,
            force_allin_threshold = 0.15,
            merging_threshold = 0.1
        ))]
        fn new(
            initial_state: &BoardState,
            starting_pot: usize,
            effective_stack: usize,
            flop_bet_sizes_oop: &BetSizeOptions,
            flop_bet_sizes_ip: &BetSizeOptions,
            turn_bet_sizes_oop: &BetSizeOptions,
            turn_bet_sizes_ip: &BetSizeOptions,
            river_bet_sizes_oop: &BetSizeOptions,
            river_bet_sizes_ip: &BetSizeOptions,
            turn_donk_sizes: Option<&DonkSizeOptions>,
            river_donk_sizes: Option<&DonkSizeOptions>,
            add_allin_threshold: f64,
            force_allin_threshold: f64,
            merging_threshold: f64,
        ) -> PyResult<Self> {
            Ok(Self(postflop_solver::TreeConfig {
                initial_state: initial_state.0,
                starting_pot: starting_pot as i32,
                effective_stack: effective_stack as i32,
                rake_rate: 0.0,
                rake_cap: 0.0,
                flop_bet_sizes: [flop_bet_sizes_oop.0.clone(), flop_bet_sizes_ip.0.clone()],
                turn_bet_sizes: [turn_bet_sizes_oop.0.clone(), turn_bet_sizes_ip.0.clone()],
                river_bet_sizes: [river_bet_sizes_oop.0.clone(), river_bet_sizes_ip.0.clone()],
                turn_donk_sizes: turn_donk_sizes.map(|d| d.0.clone()),
                river_donk_sizes: river_donk_sizes.map(|d| d.0.clone()),
                add_allin_threshold,
                force_allin_threshold,
                merging_threshold,
            }))
        }

        fn __repr__(&self) -> String {
            format!("{:#?}", self.0)
        }

        fn __str__(&self) -> String {
            format!("{:#?}", self.0)
        }
    }

    /// Available actions of the postflop game.
    #[pyclass]
    pub struct Action(postflop_solver::Action);

    #[pymethods]
    impl Action {
        #[new]
        fn new(action: &str) -> PyResult<Self> {
            get_action(action, 0)
                .map(Self)
                .map_err(|e| PyValueError::new_err(e))
        }

        fn __repr__(&self) -> String {
            format!("{:#?}", self.0)
        }

        fn __str__(&self) -> String {
            format!("{:#?}", self.0)
        }
    }

    fn get_action(str_action: &str, x: usize) -> Result<postflop_solver::Action, String> {
        match str_action {
            "Check" => Ok(postflop_solver::Action::Check),
            "Bet(x)" => Ok(postflop_solver::Action::Bet(x as i32)),
            "Raise(x)" => Ok(postflop_solver::Action::Raise(x as i32)),
            "AllIn(x)" => Ok(postflop_solver::Action::AllIn(x as i32)),
            _ => Err(format!("invalid action: {}", str_action)),
        }
    }

    /// Converts a list of hole cards to strings.
    ///
    /// Parameters
    /// ----------
    /// holes : list of tuple of int
    ///     List of (card_id1, card_id2) pairs representing hole cards.
    ///
    /// Returns
    /// -------
    /// list of str
    ///     List of string representations of the hole cards.
    ///
    /// Examples
    /// --------
    /// >>> holes_to_strings([(12, 11)])
    /// ['AcKd']
    #[pyfunction]
    fn holes_to_strings(holes: Vec<(u8, u8)>) -> PyResult<Vec<String>> {
        postflop_solver::holes_to_strings(holes.as_slice()).map_err(|e| PyValueError::new_err(e))
    }

    /// Computes the weighted average of values.
    ///
    /// Parameters
    /// ----------
    /// values : list of float
    ///     List of values.
    /// weights : list of float
    ///     List of weights.
    ///
    /// Returns
    /// -------
    /// float
    ///     The weighted average.
    ///
    /// Examples
    /// --------
    /// >>> compute_average([1.0, 2.0, 3.0], [1.0, 1.0, 1.0])
    /// 2.0
    #[pyfunction]
    fn compute_average(values: Vec<f32>, weights: Vec<f32>) -> f32 {
        postflop_solver::compute_average(values.as_slice(), weights.as_slice())
    }

    /// Converts a card string to a card ID.
    ///
    /// Parameters
    /// ----------
    /// card : str
    ///     Card string (e.g., "Ac", "Kh", "2d").
    ///
    /// Returns
    /// -------
    /// int
    ///     Card ID (0-51).
    #[pyfunction]
    fn card_from_str(card: &str) -> PyResult<u8> {
        postflop_solver::card_from_str(card)
            .map(|c| c as u8)
            .map_err(|e| PyValueError::new_err(e))
    }

    /// Converts a flop string to an array of card IDs.
    ///
    /// Parameters
    /// ----------
    /// flop : str
    ///     Flop string (e.g., "AsKh3d").
    ///
    /// Returns
    /// -------
    /// numpy.ndarray
    ///     Array of 3 card IDs.
    #[pyfunction]
    fn flop_from_str<'py>(py: Python<'py>, flop: &str) -> PyResult<Bound<'py, PyArray1<u8>>> {
        postflop_solver::flop_from_str(flop)
            .map(|arr| {
                let array = Array1::from_vec(arr.to_vec());
                array.into_pyarray_bound(py)
            })
            .map_err(|e| PyValueError::new_err(e))
    }

    /// Converts a card ID to a string.
    ///
    /// Parameters
    /// ----------
    /// card : int
    ///     Card ID (0-51).
    ///
    /// Returns
    /// -------
    /// str
    ///     Card string (e.g., "Ac").
    #[pyfunction]
    fn card_to_string(card: u8) -> PyResult<String> {
        postflop_solver::card_to_string(card).map_err(|e| PyValueError::new_err(e))
    }

    /// Converts a hole card to a string.
    ///
    /// Parameters
    /// ----------
    /// hole : tuple of int
    ///     (card_id1, card_id2) pair.
    ///
    /// Returns
    /// -------
    /// str
    ///     Hole card string (e.g., "AcKd").
    #[pyfunction]
    fn hole_to_string(hole: (u8, u8)) -> PyResult<String> {
        postflop_solver::hole_to_string(hole).map_err(|e| PyValueError::new_err(e))
    }

    /// The main postflop game class.
    ///
    /// This class represents a postflop poker game tree and provides methods
    /// for navigating the tree, solving the game, and querying strategies and values.
    ///
    /// Parameters
    /// ----------
    /// card_config : CardConfig
    ///     The card configuration containing player ranges and board cards.
    /// tree_config : TreeConfig
    ///     The game tree configuration.
    ///
    /// Examples
    /// --------
    /// >>> card_config = CardConfig("AA,KK", "QQ", "AsKh3d")
    /// >>> tree_config = TreeConfig(BoardState("Flop"), 200, 900, bet_sizes, bet_sizes, bet_sizes, bet_sizes, bet_sizes, bet_sizes)
    /// >>> game = PostFlopGame(card_config, tree_config)
    #[pyclass]
    pub struct PostFlopGame(postflop_solver::PostFlopGame);

    #[pymethods]
    impl PostFlopGame {
        /// Creates a new PostFlopGame with the specified configurations.
        ///
        /// Parameters
        /// ----------
        /// card_config : CardConfig
        ///   The card configuration containing player ranges and board cards.
        /// tree_config : TreeConfig
        ///   The game tree configuration.
        ///
        /// Returns
        /// -------
        /// PostFlopGame
        ///   The created game instance.
        #[new]
        fn new(card_config: &CardConfig, tree_config: &TreeConfig) -> PyResult<Self> {
            let action_tree = postflop_solver::ActionTree::new(tree_config.0.clone())
                .map_err(|e| PyValueError::new_err(e))?;
            postflop_solver::PostFlopGame::with_config(card_config.0.clone(), action_tree)
                .map(Self)
                .map_err(|e| PyValueError::new_err(e))
        }

        /// Moves the current node back to the root node.
        fn back_to_root(&mut self) {
            self.0.back_to_root();
        }

        /// Returns the history of actions from root to current node.
        ///
        /// The history is a list of action indices, i.e., the arguments of `play()`.
        /// If `usize.MAX` was passed to `play()`, it is replaced with the actual action index.
        ///
        /// Returns
        /// -------
        /// numpy.ndarray
        ///   Array of action indices.
        ///
        /// Examples
        /// --------
        /// >>> game.history()
        /// array([1, 0], dtype=uint64)
        fn history<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<usize>> {
            let vec = self.0.history().to_vec();
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }

        /// Applies a history of actions from the root node.
        ///
        /// This method first calls `back_to_root()` and then calls `play()` for each action
        /// in the history. The action of `usize.MAX` is allowed for chance nodes.
        ///
        /// Parameters
        /// ----------
        /// history : list of int
        ///   Action indices to apply.
        fn apply_history(&mut self, history: Vec<usize>) -> PyResult<()> {
            self.0.apply_history(&history);
            Ok(())
        }

        /// Returns whether the current node is terminal.
        ///
        /// Note that the turn/river node after the call action after the all-in action
        /// is considered terminal.
        ///
        /// Returns
        /// -------
        /// bool
        fn is_terminal_node(&self) -> bool {
            self.0.is_terminal_node()
        }

        /// Returns whether the current node is a chance node (i.e., turn/river node).
        ///
        /// Note that the terminal node is not considered a chance node.
        ///
        /// Returns
        /// -------
        /// bool
        fn is_chance_node(&self) -> bool {
            self.0.is_chance_node()
        }

        /// If the current node is a chance node, returns a list of cards that can be dealt.
        ///
        /// The returned value is a 64-bit integer.
        /// The i-th bit is set to 1 if the card of ID i can be dealt.
        /// If the current node is not a chance node, 0 is returned.
        ///
        /// Returns
        /// -------
        /// int
        ///   64-bit integer where i-th bit = 1 if card i can be dealt.
        fn possible_cards(&self) -> u64 {
            self.0.possible_cards()
        }

        /// Returns the current player (0=OOP, 1=IP).
        ///
        /// If the current node is a terminal node or a chance node, returns an undefined value.
        ///
        /// Returns
        /// -------
        /// int
        ///   0 for OOP, 1 for IP.
        fn current_player(&self) -> usize {
            self.0.current_player()
        }

        /// Returns the current board cards.
        ///
        /// The returned vector is of length 3, 4, or 5.
        /// The flop cards, the turn card, and the river card, if any, are stored in this order.
        ///
        /// Returns
        /// -------
        /// numpy.ndarray
        ///   Array of card IDs for flop (3), turn (4), river (5).
        ///
        /// Examples
        /// --------
        /// >>> game.current_board()
        /// array([12, 11, 5, 8, 6], dtype=uint8)
        fn current_board<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<u8>> {
            let vec = self.0.current_board();
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }

        /// Plays an action at the current node.
        ///
        /// - For player nodes: the `action` corresponds to the index into `available_actions()`.
        /// - For chance nodes: the `action` corresponds to the dealt card ID, or `usize.MAX` for auto-select.
        ///
        /// Parameters
        /// ----------
        /// action : int
        ///   For player nodes: index into available_actions().
        ///   For chance nodes: card ID to deal, or usize.MAX for auto-select.
        ///
        /// Raises
        /// ------
        /// Exception
        ///   If memory is not allocated is terminal or the current node.
        ///
        /// Examples
        /// --------
        /// >>> game.play(1)  # Play the second action
        fn play(&mut self, action: usize) {
            self.0.play(action);
        }

        /// Returns private cards for a player.
        ///
        /// Parameters
        /// ----------
        /// player : int
        ///   0 for OOP, 1 for IP.
        ///
        /// Returns
        /// -------
        /// list of tuple of int
        ///   List of (card_id1, card_id2) pairs.
        ///
        /// Examples
        /// --------
        /// >>> game.private_cards(0)
        /// [(12, 11), (12, 10), ...]
        fn private_cards(&self, player: usize) -> Vec<(u8, u8)> {
            Vec::from_iter(
                self.0
                    .private_cards(player)
                    .iter()
                    .map(|(a, b)| (*a as u8, *b as u8)),
            )
        }

        /// Returns estimated memory usage.
        ///
        /// Returns
        /// -------
        /// tuple of (int, int)
        ///   (uncompressed_bytes, compressed_bytes)
        ///
        /// Examples
        /// --------
        /// >>> game.memory_usage()
        /// (1073741824, 536870912)
        fn memory_usage(&self) -> (u64, u64) {
            self.0.memory_usage()
        }

        /// Returns the tree configuration.
        ///
        /// Returns
        /// -------
        /// TreeConfig
        fn tree_config(&self) -> TreeConfig {
            TreeConfig(self.0.tree_config().clone())
        }

        /// Returns available actions at current node.
        ///
        /// If the current node is a terminal, returns an empty list.
        /// If the current node is a turn/river node and not a terminal,
        /// isomorphic chances are grouped into one representative action.
        ///
        /// Returns
        /// -------
        /// list of Action
        ///
        /// Examples
        /// --------
        /// >>> game.available_actions()
        /// [Check, Bet(60)]
        fn available_actions(&self) -> Vec<Action> {
            self.0
                .available_actions()
                .iter()
                .map(|a| Action(*a))
                .collect()
        }

        /// Returns equity for each private hand.
        ///
        /// Must call `cache_normalized_weights()` first.
        ///
        /// Parameters
        /// ----------
        /// player : int
        ///   0 for OOP, 1 for IP.
        ///
        /// Returns
        /// -------
        /// numpy.ndarray
        ///   Array of float32 values.
        ///
        /// Examples
        /// --------
        /// >>> game.cache_normalized_weights()
        /// >>> game.equity(0)
        /// array([0.85, 0.75, ...], dtype=float32)
        fn equity<'py>(&self, py: Python<'py>, player: usize) -> Bound<'py, PyArray1<f32>> {
            let vec = self.0.equity(player);
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }

        /// Returns expected values for each private hand.
        ///
        /// Must call `cache_normalized_weights()` first and game must be solved.
        ///
        /// Parameters
        /// ----------
        /// player : int
        ///   0 for OOP, 1 for IP.
        ///
        /// Returns
        /// -------
        /// numpy.ndarray
        ///   Array of float32 values.
        ///
        /// Examples
        /// --------
        /// >>> game.cache_normalized_weights()
        /// >>> game.expected_values(0)
        /// array([0.5, 0.3, ...], dtype=float32)
        fn expected_values<'py>(
            &self,
            py: Python<'py>,
            player: usize,
        ) -> Bound<'py, PyArray1<f32>> {
            let vec = self.0.expected_values(player);
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }

        /// Returns detailed EV for each action and hand.
        ///
        /// Must call `cache_normalized_weights()` first and game must be solved.
        ///
        /// Parameters
        /// ----------
        /// player : int
        ///   0 for OOP, 1 for IP.
        ///
        /// Returns
        /// -------
        /// numpy.ndarray
        ///   Array of float32 values with length num_actions * num_hands.
        ///
        /// Examples
        /// --------
        /// >>> game.cache_normalized_weights()
        /// >>> game.expected_values_detail(0)
        /// array([0.2, 0.8, 0.5, 0.5, ...], dtype=float32)
        fn expected_values_detail<'py>(
            &self,
            py: Python<'py>,
            player: usize,
        ) -> Bound<'py, PyArray1<f32>> {
            let vec = self.0.expected_values_detail(player);
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }

        /// Returns normalized weights for each private hand.
        ///
        /// The "normalized weights" represent the actual number of combinations
        /// that the player is holding each hand.
        ///
        /// Must call `cache_normalized_weights()` first.
        ///
        /// Parameters
        /// ----------
        /// player : int
        ///   0 for OOP, 1 for IP.
        ///
        /// Returns
        /// -------
        /// numpy.ndarray
        ///   Array of float32 values.
        ///
        /// Examples
        /// --------
        /// >>> game.cache_normalized_weights()
        /// >>> game.normalized_weights(0)
        /// array([6., 6., ...], dtype=float32)
        fn normalized_weights<'py>(
            &self,
            py: Python<'py>,
            player: usize,
        ) -> Bound<'py, PyArray1<f32>> {
            let weights = self.0.normalized_weights(player);
            let vec = Vec::from_iter(weights.iter().map(|w| *w as f32));
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }

        /// Computes the normalized weights and caches them.
        ///
        /// After mutating the current node (e.g., by calling `play()`), this method
        /// must be called once before calling `normalized_weights()`, `equity()`,
        /// `expected_values()`, or `expected_values_detail()`.
        ///
        /// Time complexity:
        /// - (no bunching) O(#(OOP private hands) + #(IP private hands))
        /// - (bunching) O(#(OOP private hands) * #(IP private hands))
        fn cache_normalized_weights(&mut self) {
            self.0.cache_normalized_weights();
        }

        /// Returns strategy at current node.
        ///
        /// The return value is a vector of the length num_actions * num_private_hands.
        /// The probability of the i-th action with the j-th private hand is stored in the
        /// i * num_private_hands + j-th element.
        ///
        /// If a hand overlaps with the board, an undefined value is returned.
        ///
        /// Panics if the current node is a terminal node or a chance node.
        /// Also, panics if the memory is not yet allocated.
        ///
        /// Returns
        /// -------
        /// numpy.ndarray
        ///   Array of float32 values with length num_actions * num_hands.
        ///
        /// Examples
        /// --------
        /// >>> game.strategy()
        /// array([0.8, 0.2, 0.5, 0.5, ...], dtype=float32)
        fn strategy<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f32>> {
            let vec = self.0.strategy();
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }

        /// Returns the total bet amount for each player.
        ///
        /// Returns
        /// -------
        /// list of int
        ///   [oop_bet, ip_bet]
        ///
        /// Examples
        /// --------
        /// >>> game.total_bet_amount()
        /// [100, 200]
        fn total_bet_amount(&self) -> [i32; 2] {
            self.0.total_bet_amount()
        }

        /// Locks strategy at current node with custom frequencies.
        ///
        /// The strategy argument must be a slice of the length num_actions * num_private_hands.
        ///
        /// - A negative value is treated as a zero.
        /// - If the i * num_private_hands + j-th element of the strategy is positive for some i,
        ///   the j-th private hand will be locked. The probability for each action will be normalized
        ///   so that their sum is 1.0.
        /// - If the i * num_private_hands + j-th element of the strategy is not positive for all i,
        ///   the j-th private hand will not be locked. That is, the solver can adjust the
        ///   strategy of the j-th private hand.
        ///
        /// This method must be called after allocating memory and before solving the game.
        /// Panics if the memory is not yet allocated or the game is already solved.
        /// Also, panics if the current node is a terminal node or a chance node.
        ///
        /// Parameters
        /// ----------
        /// strategy : list of float
        ///   Array of float32 values with length = num_actions * num_hands.
        ///
        /// Examples
        /// --------
        /// >>> game.lock_current_strategy([0.8, 0.2, 0.5, 0.5])
        fn lock_current_strategy(&mut self, strategy: Vec<f32>) -> PyResult<()> {
            self.0.lock_current_strategy(&strategy);
            Ok(())
        }

        /// Unlocks strategy at current node.
        ///
        /// This method must be called after allocating memory and before solving the game.
        /// Panics if the memory is not yet allocated or the game is already solved.
        /// Also, panics if the current node is a terminal node or a chance node.
        fn unlock_current_strategy(&mut self) {
            self.0.unlock_current_strategy();
        }

        /// Returns current locking strategy if any.
        ///
        /// If the current node is not locked, None is returned.
        ///
        /// Otherwise, returns a reference to the vector of the length
        /// num_actions * num_private_hands.
        /// The probability of the i-th action with the j-th private hand is stored in the
        /// i * num_private_hands + j-th element.
        /// If the j-th private hand is not locked, returns -1.0 for all i.
        ///
        /// Returns
        /// -------
        /// numpy.ndarray or None
        ///   Array of float32 values or None.
        ///
        /// Examples
        /// --------
        /// >>> game.current_locking_strategy()
        /// array([0.8, 0.2, -1., -1.], dtype=float32)
        fn current_locking_strategy<'py>(
            &self,
            py: Python<'py>,
        ) -> Option<Bound<'py, PyArray1<f32>>> {
            self.0.current_locking_strategy().map(|vec| {
                let array = Array1::from_vec(vec);
                array.into_pyarray_bound(py)
            })
        }

        /// Returns whether memory has been allocated.
        ///
        /// Returns
        /// -------
        /// bool or None
        ///   True if allocated uncompressed, False if compressed, None if not allocated.
        fn is_memory_allocated(&self) -> Option<bool> {
            self.0.is_memory_allocated()
        }

        /// Returns the card configuration.
        ///
        /// Returns
        /// -------
        /// CardConfig
        fn card_config(&self) -> CardConfig {
            CardConfig(self.0.card_config().clone())
        }

        /// Returns added lines in the action tree.
        ///
        /// Returns
        /// -------
        /// list of list of Action
        fn added_lines(&self) -> Vec<Vec<Action>> {
            self.0
                .added_lines()
                .iter()
                .map(|line| line.iter().map(|a| Action(*a)).collect())
                .collect()
        }

        /// Returns removed lines from the action tree.
        ///
        /// Returns
        /// -------
        /// list of list of Action
        fn removed_lines(&self) -> Vec<Vec<Action>> {
            self.0
                .removed_lines()
                .iter()
                .map(|line| line.iter().map(|a| Action(*a)).collect())
                .collect()
        }

        /// Allocates memory for the game.
        ///
        /// Parameters
        /// ----------
        /// enable_compression : bool, default False
        ///   If True, uses 16-bit integers (saves ~50% memory).
        fn allocate_memory(&mut self, enable_compression: bool) {
            self.0.allocate_memory(enable_compression);
        }

        /// Returns raw weights for each private hand.
        ///
        /// If a hand overlaps with the board, returns 0.0.
        ///
        /// Parameters
        /// ----------
        /// player : int
        ///   0 for OOP, 1 for IP.
        ///
        /// Returns
        /// -------
        /// numpy.ndarray
        ///   Array of float32 values.
        ///
        /// Examples
        /// --------
        /// >>> game.weights(0)
        /// array([1., 1., ...], dtype=float32)
        fn weights<'py>(&self, py: Python<'py>, player: usize) -> Bound<'py, PyArray1<f32>> {
            let w = self.0.weights(player);
            let vec = Vec::from_iter(w.iter().map(|x| *x));
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }
    }

    /// Performs Discounted CFR algorithm until the given number of iterations or exploitability is satisfied.
    ///
    /// This method allocates memory, solves the game, and returns the exploitability of the obtained strategy.
    ///
    /// Parameters
    /// ----------
    /// game : PostFlopGame
    ///   The game instance to solve.
    /// max_num_iterations : int
    ///   Maximum number of iterations.
    /// target_exploitability : float
    ///   Target exploitability threshold. Solver stops when exploitability <= target.
    /// verbose : bool
    ///   Whether to print progress.
    ///
    /// Returns
    /// -------
    /// float
    ///   The exploitability of the obtained strategy.
    ///
    /// Examples
    /// --------
    /// >>> solve(game, 1000, 0.001, True)
    /// iteration: 0 / 1000 (exploitability = 1.2345e-01)
    /// iteration: 100 / 1000 (exploitability = 5.6789e-03)
    /// 0.0056789
    #[pyfunction]
    fn solve(
        game: &mut PostFlopGame,
        max_num_iterations: u32,
        target_exploitability: f32,
        verbose: bool,
    ) -> f32 {
        let game_instance = &mut game.0;

        let (mem_usage, mem_usage_compressed) = game_instance.memory_usage();
        println!(
            "Memory usage without compression (32-bit float): {:.2}GB",
            mem_usage as f64 / (1024.0 * 1024.0 * 1024.0)
        );
        println!(
            "Memory usage with compression (16-bit integer): {:.2}GB",
            mem_usage_compressed as f64 / (1024.0 * 1024.0 * 1024.0)
        );

        game_instance.allocate_memory(false);

        postflop_solver::solve(
            game_instance,
            max_num_iterations,
            target_exploitability,
            verbose,
        )
    }

    /// Computes the exploitability of the current strategy.
    ///
    /// Parameters
    /// ----------
    /// game : PostFlopGame
    ///   The game instance.
    ///
    /// Returns
    /// -------
    /// float
    ///   The exploitability value.
    ///
    /// Examples
    /// --------
    /// >>> compute_exploitability(game)
    /// 0.05
    #[pyfunction]
    fn compute_exploitability(game: &PostFlopGame) -> f32 {
        postflop_solver::compute_exploitability(&game.0)
    }

    /// Computes the expected values of the current strategy of each player.
    ///
    /// The bias, i.e., (starting pot) / 2, is already subtracted to increase the significant figures.
    /// This treatment makes the return value zero-sum when not raked.
    ///
    /// Parameters
    /// ----------
    /// game : PostFlopGame
    ///   The game instance.
    ///
    /// Returns
    /// -------
    /// list of float
    ///   [oop_ev, ip_ev]
    ///
    /// Examples
    /// --------
    /// >>> compute_current_ev(game)
    /// [0.5, -0.5]
    #[pyfunction]
    fn compute_current_ev(game: &PostFlopGame) -> [f32; 2] {
        postflop_solver::compute_current_ev(&game.0)
    }

    /// Computes the expected values of the MES (Maximally Exploitative Strategy) of each player.
    ///
    /// The bias, i.e., (starting pot) / 2, is already subtracted to increase the significant figures.
    /// Therefore, the average of the return value corresponds to the exploitability value if not raked.
    ///
    /// Parameters
    /// ----------
    /// game : PostFlopGame
    ///   The game instance.
    ///
    /// Returns
    /// -------
    /// list of float
    ///   [oop_mes_ev, ip_mes_ev]
    ///
    /// Examples
    /// --------
    /// >>> compute_mes_ev(game)
    /// [1.0, -1.0]
    #[pyfunction]
    fn compute_mes_ev(game: &PostFlopGame) -> [f32; 2] {
        postflop_solver::compute_mes_ev(&game.0)
    }

    /// Finalizes the solving process.
    ///
    /// Computes the expected values and saves them to the game tree.
    /// Must be called after solving if you want to query expected values.
    #[pyfunction]
    fn finalize(game: &mut PostFlopGame) {
        postflop_solver::finalize(&mut game.0)
    }

    #[pyfunction]
    fn get_array(py: Python<'_>) -> Bound<'_, PyArray1<i32>> {
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        let array = Array1::from_vec(vec);
        array.into_pyarray_bound(py)
    }
}

#![allow(unsafe_op_in_unsafe_fn)]
use pyo3::prelude::*;

#[pymodule]
mod oxipostflop {
    use pyo3::exceptions::PyValueError;
    use pyo3::prelude::*;

    use ndarray::Array1;
    use numpy::IntoPyArray;
    use numpy::PyArray1;

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

    fn get_action(str_action: &str, x: usize) -> Result<postflop_solver::Action, String> {
        match str_action {
            "Check" => Ok(postflop_solver::Action::Check),
            "Bet(x)" => Ok(postflop_solver::Action::Bet(x as i32)),
            "Raise(x)" => Ok(postflop_solver::Action::Raise(x as i32)),
            "AllIn(x)" => Ok(postflop_solver::Action::AllIn(x as i32)),
            _ => Err(format!("invalid action: {}", str_action)),
        }
    }

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

    #[pyfunction]
    fn holes_to_strings(holes: Vec<(u8, u8)>) -> PyResult<Vec<String>> {
        postflop_solver::holes_to_strings(holes.as_slice()).map_err(|e| PyValueError::new_err(e))
    }

    #[pyfunction]
    fn compute_average(values: Vec<f32>, weights: Vec<f32>) -> f32 {
        postflop_solver::compute_average(values.as_slice(), weights.as_slice())
    }

    #[pyfunction]
    fn card_from_str(card: &str) -> PyResult<u8> {
        postflop_solver::card_from_str(card)
            .map(|c| c as u8)
            .map_err(|e| PyValueError::new_err(e))
    }

    #[pyfunction]
    fn flop_from_str<'py>(py: Python<'py>, flop: &str) -> PyResult<Bound<'py, PyArray1<u8>>> {
        postflop_solver::flop_from_str(flop)
            .map(|arr| {
                let array = Array1::from_vec(arr.to_vec());
                array.into_pyarray_bound(py)
            })
            .map_err(|e| PyValueError::new_err(e))
    }

    #[pyfunction]
    fn card_to_string(card: u8) -> PyResult<String> {
        postflop_solver::card_to_string(card).map_err(|e| PyValueError::new_err(e))
    }

    #[pyfunction]
    fn hole_to_string(hole: (u8, u8)) -> PyResult<String> {
        postflop_solver::hole_to_string(hole).map_err(|e| PyValueError::new_err(e))
    }

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
        /// Returns
        /// -------
        /// numpy.ndarray
        ///   Array of action indices.
        fn history<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<usize>> {
            let vec = self.0.history().to_vec();
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }

        /// Applies a history of actions from the root node.
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
        /// Returns
        /// -------
        /// bool
        fn is_terminal_node(&self) -> bool {
            self.0.is_terminal_node()
        }

        /// Returns whether the current node is a chance node.
        ///
        /// Returns
        /// -------
        /// bool
        fn is_chance_node(&self) -> bool {
            self.0.is_chance_node()
        }

        /// Returns possible cards as a bit mask at a chance node.
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
        /// Returns
        /// -------
        /// int
        fn current_player(&self) -> usize {
            self.0.current_player()
        }

        /// Returns the current board cards.
        ///
        /// Returns
        /// -------
        /// numpy.ndarray
        ///   Array of card IDs for flop (3), turn (4), river (5).
        fn current_board<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<u8>> {
            let vec = self.0.current_board();
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }

        /// Plays an action at the current node.
        ///
        /// Parameters
        /// ----------
        /// action : int
        ///   For player nodes: index into available_actions().
        ///   For chance nodes: card ID to deal, or usize::MAX for auto-select.
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
        /// Returns
        /// -------
        /// list of Action
        fn available_actions(&self) -> Vec<Action> {
            self.0
                .available_actions()
                .iter()
                .map(|a| Action(*a))
                .collect()
        }

        /// Returns equity for each private hand.
        ///
        /// Must call cache_normalized_weights() first.
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
        fn equity<'py>(&self, py: Python<'py>, player: usize) -> Bound<'py, PyArray1<f32>> {
            let vec = self.0.equity(player);
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }

        /// Returns expected values for each private hand.
        ///
        /// Must call cache_normalized_weights() first and game must be solved.
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
        /// Must call cache_normalized_weights() first and game must be solved.
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
        /// Must call cache_normalized_weights() first.
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
        fn normalized_weights<'py>(
            &self,
            py: Python<'py>,
            player: usize,
        ) -> Bound<'py, PyArray1<f32>> {
            let weights = self.0.normalized_weights(player);
            let vec = Vec::from_iter(weights.iter().map(|w| *w as f32));  // TODO: can we make this
                                                                          // more efficient?
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }

        /// Caches normalized weights for equity/EV calculations.
        fn cache_normalized_weights(&mut self) {
            self.0.cache_normalized_weights();
        }

        /// Returns strategy at current node.
        ///
        /// Returns
        /// -------
        /// numpy.ndarray
        ///   Array of float32 values with length num_actions * num_hands.
        fn strategy<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<f32>> {
            let vec = self.0.strategy();
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }

        /// Returns total bet amount for each player.
        ///
        /// Returns
        /// -------
        /// list of int
        ///   [oop_bet, ip_bet]
        fn total_bet_amount(&self) -> [i32; 2] {
            self.0.total_bet_amount()
        }

        /// Locks strategy at current node with custom frequencies.
        ///
        /// Must call after allocate_memory() and before solve().
        ///
        /// Parameters
        /// ----------
        /// strategy : numpy.ndarray
        ///   Array of float32 values with length = num_actions * num_hands.
        fn lock_current_strategy(&mut self, strategy: Vec<f32>) -> PyResult<()> {
            self.0.lock_current_strategy(&strategy);
            Ok(())
        }

        /// Unlocks strategy at current node.
        fn unlock_current_strategy(&mut self) {
            self.0.unlock_current_strategy();
        }

        /// Returns current locking strategy if any.
        ///
        /// Returns
        /// -------
        /// numpy.ndarray or None
        ///   Array of float32 values or None.
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
        /// Parameters
        /// ----------
        /// player : int
        ///   0 for OOP, 1 for IP.
        ///
        /// Returns
        /// -------
        /// numpy.ndarray
        ///   Array of float32 values.
        fn weights<'py>(&self, py: Python<'py>, player: usize) -> Bound<'py, PyArray1<f32>> {
            let w = self.0.weights(player);
            let vec = Vec::from_iter(w.iter().map(|x| *x));
            let array = Array1::from_vec(vec);
            array.into_pyarray_bound(py)
        }
    }

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

    #[pyfunction]
    fn compute_exploitability(game: &PostFlopGame) -> f32 {
        postflop_solver::compute_exploitability(&game.0)
    }

    #[pyfunction]
    fn compute_current_ev(game: &PostFlopGame) -> [f32; 2] {
        postflop_solver::compute_current_ev(&game.0)
    }

    #[pyfunction]
    fn compute_mes_ev(game: &PostFlopGame) -> [f32; 2] {
        postflop_solver::compute_mes_ev(&game.0)
    }

    #[pyfunction]
    fn finalize(game: &mut PostFlopGame) {
        postflop_solver::finalize(&mut game.0);
    }

    #[pyfunction]
    fn get_array(py: Python<'_>) -> Bound<'_, PyArray1<i32>> {
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        let array = Array1::from_vec(vec);
        array.into_pyarray_bound(py)
    }
}

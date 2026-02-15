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
            let donk_sizes_instance = postflop_solver::DonkSizeOptions::try_from(donk_sizes)
                .map_err(|e| PyValueError::new_err(format!("something wrong my brother: {}", e)))?;
            Ok(Self(donk_sizes_instance))
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
            let bet_sizes_instance =
                postflop_solver::BetSizeOptions::try_from((bet_sizes, raise_sizes)).map_err(
                    |e| PyValueError::new_err(format!("something wrong my brother: {}", e)),
                )?;
            Ok(Self(bet_sizes_instance))
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
            let initial_state = get_board_state(initial_state)
                .map_err(|e| PyValueError::new_err(format!("something wrong my brother: {}", e)))?;
            Ok(Self(initial_state))
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
                .map_err(|e| PyValueError::new_err(format!("something wrong my brother: {}", e)))?;
            let ip_range_instance = ip_range
                .parse::<postflop_solver::Range>()
                .map_err(|e| PyValueError::new_err(format!("something wrong my brother: {}", e)))?;

            let flop_instance = postflop_solver::flop_from_str(flop)
                .map_err(|e| PyValueError::new_err(format!("something wrong my brother: {}", e)))?;

            let turn_instance = turn.map_or(Ok(postflop_solver::NOT_DEALT), |turn| {
                postflop_solver::card_from_str(turn).map_err(|e| {
                    PyValueError::new_err(format!("something wrong my brother: {}", e))
                })
            })?;

            let river_instance = river.map_or(Ok(postflop_solver::NOT_DEALT), |river| {
                postflop_solver::card_from_str(river).map_err(|e| {
                    PyValueError::new_err(format!("something wrong my brother: {}", e))
                })
            })?;

            let card_config_instance = postflop_solver::CardConfig {
                range: [oop_range_instance, ip_range_instance],
                flop: flop_instance,
                turn: turn_instance,
                river: river_instance,
            };
            Ok(Self(card_config_instance))
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
            let initial_state_instance = initial_state.0;
            let tree_config_instance = postflop_solver::TreeConfig {
                initial_state: initial_state_instance,
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
            };
            Ok(Self(tree_config_instance))
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
            let action_instance = get_action(action, 0)
                .map_err(|e| PyValueError::new_err(format!("something wrong my brother: {}", e)))?;
            Ok(Self(action_instance))
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
        let strings = postflop_solver::holes_to_strings(holes.as_slice())
            .map_err(|e| PyValueError::new_err(format!("something wrong my brother: {}", e)))?;
        Ok(strings)
    }

    #[pyfunction]
    fn compute_average(values: Vec<f32>, weights: Vec<f32>) -> f32 {
        postflop_solver::compute_average(values.as_slice(), weights.as_slice())
    }

    #[pyfunction]
    fn card_from_str(card: &str) -> PyResult<u8> {
        postflop_solver::card_from_str(card)
            .map(|c| c as u8)
            .map_err(|e| PyValueError::new_err(format!("something wrong my brother: {}", e)))
    }

    #[pyclass]
    pub struct PostFlopGame(postflop_solver::PostFlopGame);
    #[pymethods]
    impl PostFlopGame {
        #[new]
        fn new(card_config: &CardConfig, tree_config: &TreeConfig) -> PyResult<Self> {
            let card_config_instance = card_config.0.clone();
            let action_tree_instance = postflop_solver::ActionTree::new(tree_config.0.clone())
                .map_err(|e| PyValueError::new_err(format!("something wrong my brother: {}", e)))?;
            let game_instance = postflop_solver::PostFlopGame::with_config(
                card_config_instance,
                action_tree_instance,
            )
            .map_err(|e| PyValueError::new_err(format!("something wrong my brother: {}", e)))?;
            Ok(Self(game_instance))
        }

        fn play(&mut self, action: usize) {
            self.0.play(action);
        }

        fn back_to_root(&mut self) {
            self.0.back_to_root();
        }

        fn is_chance_node(&self) -> bool {
            self.0.is_chance_node()
        }

        // return a 64-bit integer representing the possible cards as a bit mask
        fn possible_cards(&self) -> u64 {
            self.0.possible_cards()
        }

        fn private_cards(&self, player: usize) -> Vec<(u8, u8)> {
            Vec::from_iter(
                self.0
                    .private_cards(player)
                    .iter()
                    .map(|(a, b)| (*a as u8, *b as u8)),
            )
        }

        fn memory_usage(&self) -> (u64, u64) {
            self.0.memory_usage()
        }

        fn tree_config(&self) -> TreeConfig {
            TreeConfig(self.0.tree_config().clone())
        }

        fn available_actions(&self) -> Vec<Action> {
            self.0
                .available_actions()
                .iter()
                .map(|a| Action(*a))
                .collect()
        }

        fn equity(&self, player: usize) -> Vec<f32> {
            self.0.equity(player)
        }

        fn expected_values(&self, player: usize) -> Vec<f32> {
            self.0.expected_values(player)
        }

        fn normalized_weights(&self, player: usize) -> Vec<f32> {
            let weights = self.0.normalized_weights(player);
            Vec::from_iter(weights.iter().map(|w| *w as f32))
        }

        fn cache_normalized_weights(&mut self) {
            self.0.cache_normalized_weights();
        }

        fn strategy(&self) -> Vec<f32> {
            self.0.strategy()
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

        // allocate memory without compression (use 32-bit float)
        game_instance.allocate_memory(false);

        let exploitability = postflop_solver::solve(
            game_instance,
            max_num_iterations,
            target_exploitability,
            verbose,
        );
        exploitability
    }

    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pyfunction]
    fn get_array(py: Python<'_>) -> Bound<'_, PyArray1<i32>> {
        let vec: Vec<i32> = vec![1, 2, 3, 4, 5];
        let array = Array1::from_vec(vec);
        array.into_pyarray_bound(py)
    }

    use postflop_solver::*;
    #[allow(dead_code)]
    fn mama() {
        // ranges of OOP and IP in string format
        // see the documentation of `Range` for more details about the format
        let oop_range = "66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs,96s+,85s+,75s+,65s,54s";
        let ip_range = "QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+,T7s+,96s+,86s+,75s+,64s+,53s+";

        let card_config = postflop_solver::CardConfig {
            range: [oop_range.parse().unwrap(), ip_range.parse().unwrap()],
            flop: flop_from_str("Td9d6h").unwrap(),
            turn: card_from_str("Qc").unwrap(),
            river: NOT_DEALT,
        };

        // bet sizes -> 60% of the pot, geometric size, and all-in
        // raise sizes -> 2.5x of the previous bet
        // see the documentation of `BetSizeOptions` for more details
        let bet_sizes = postflop_solver::BetSizeOptions::try_from(("60%, e, a", "2.5x")).unwrap();

        let tree_config = postflop_solver::TreeConfig {
            initial_state: postflop_solver::BoardState::Turn, // must match `card_config`
            starting_pot: 200,
            effective_stack: 900,
            rake_rate: 0.0,
            rake_cap: 0.0,
            flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()], // [OOP, IP]
            turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
            river_bet_sizes: [bet_sizes.clone(), bet_sizes],
            turn_donk_sizes: None, // use default bet sizes
            river_donk_sizes: Some(postflop_solver::DonkSizeOptions::try_from("50%").unwrap()),
            add_allin_threshold: 1.5, // add all-in if (maximum bet size) <= 1.5x pot
            force_allin_threshold: 0.15, // force all-in if (SPR after the opponent's call) <= 0.15
            merging_threshold: 0.1,
        };

        // build the game tree
        // `ActionTree` can be edited manually after construction
        let action_tree = ActionTree::new(tree_config).unwrap();
        let mut game =
            postflop_solver::PostFlopGame::with_config(card_config, action_tree).unwrap();

        // obtain the private hands
        let oop_cards = game.private_cards(0);
        let oop_cards_str = postflop_solver::holes_to_strings(oop_cards).unwrap();
        assert_eq!(
            &oop_cards_str[..10],
            &["5c4c", "Ac4c", "5d4d", "Ad4d", "5h4h", "Ah4h", "5s4s", "As4s", "6c5c", "7c5c"]
        );

        // check memory usage
        let (mem_usage, mem_usage_compressed) = game.memory_usage();
        println!(
            "Memory usage without compression (32-bit float): {:.2}GB",
            mem_usage as f64 / (1024.0 * 1024.0 * 1024.0)
        );
        println!(
            "Memory usage with compression (16-bit integer): {:.2}GB",
            mem_usage_compressed as f64 / (1024.0 * 1024.0 * 1024.0)
        );

        // allocate memory without compression (use 32-bit float)
        game.allocate_memory(false);

        // allocate memory with compression (use 16-bit integer)
        // game.allocate_memory(true);

        // solve the game
        let max_num_iterations = 1000;
        let target_exploitability = game.tree_config().starting_pot as f32 * 0.005; // 0.5% of the pot
        let exploitability =
            postflop_solver::solve(&mut game, max_num_iterations, target_exploitability, true);
        println!("Exploitability: {:.2}", exploitability);

        // get equity and EV of a specific hand
        game.cache_normalized_weights();
        let equity = game.equity(0); // `0` means OOP player
        let ev = game.expected_values(0);
        println!("Equity of oop_hands[0]: {:.2}%", 100.0 * equity[0]);
        println!("EV of oop_hands[0]: {:.2}", ev[0]);

        // get equity and EV of whole hand
        let weights = game.normalized_weights(0);
        let average_equity = compute_average(equity.to_vec(), weights.to_vec());
        let average_ev = compute_average(ev.to_vec(), weights.to_vec());
        println!("Average equity: {:.2}%", 100.0 * average_equity);
        println!("Average EV: {:.2}", average_ev);

        // get available actions (OOP)
        let actions = game.available_actions();
        assert_eq!(
            format!("{:?}", actions),
            "[Check, Bet(120), Bet(216), AllIn(900)]"
        );

        // play `Bet(120)`
        game.play(1);

        // get available actions (IP)
        let actions = game.available_actions();
        assert_eq!(format!("{:?}", actions), "[Fold, Call, Raise(300)]");

        // confirm that IP does not fold the nut straight
        let ip_cards = game.private_cards(1);
        let strategy = game.strategy();
        assert_eq!(ip_cards.len(), 250);
        assert_eq!(strategy.len(), 750);

        let ksjs = postflop_solver::holes_to_strings(ip_cards)
            .unwrap()
            .iter()
            .position(|s| s == "KsJs")
            .unwrap();

        // strategy[index] => Fold
        // strategy[index + ip_cards.len()] => Call
        // strategy[index + 2 * ip_cards.len()] => Raise(300)
        assert_eq!(strategy[ksjs], 0.0);
        assert!((strategy[ksjs] + strategy[ksjs + 250] + strategy[ksjs + 500] - 1.0).abs() < 1e-6);

        // play `Call`
        game.play(1);

        // confirm that the current node is a chance node (i.e., river node)
        assert!(game.is_chance_node());

        // confirm that "7s" can be dealt
        let card_7s = card_from_str("7s").unwrap();
        assert!(game.possible_cards() & (1 << card_7s) != 0);

        // deal "7s"
        game.play(card_7s as usize);

        // back to the root node
        game.back_to_root();
    }
}

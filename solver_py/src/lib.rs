use pyo3::prelude::*;

#[pymodule]
mod oxipostflop {
    use pyo3::prelude::*;

    #[pyfunction]
    fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
        Ok((a + b).to_string())
    }

    #[pyfunction]
    fn say_labaz() {
        println!("labaz zamaz");
    }

    #[pyfunction]
    fn greet(name: String) {
        println!("hellow nigger {}", name);
    }

    use solver_rs::*;
    fn mama() {
        // ranges of OOP and IP in string format
        // see the documentation of `Range` for more details about the format
        let oop_range = "66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs,96s+,85s+,75s+,65s,54s";
        let ip_range = "QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+,T7s+,96s+,86s+,75s+,64s+,53s+";

        let card_config = CardConfig {
            range: [oop_range.parse().unwrap(), ip_range.parse().unwrap()],
            flop: flop_from_str("Td9d6h").unwrap(),
            turn: card_from_str("Qc").unwrap(),
            river: NOT_DEALT,
        };

        // bet sizes -> 60% of the pot, geometric size, and all-in
        // raise sizes -> 2.5x of the previous bet
        // see the documentation of `BetSizeOptions` for more details
        let bet_sizes = BetSizeOptions::try_from(("60%, e, a", "2.5x")).unwrap();

        let tree_config = TreeConfig {
            initial_state: BoardState::Turn, // must match `card_config`
            starting_pot: 200,
            effective_stack: 900,
            rake_rate: 0.0,
            rake_cap: 0.0,
            flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()], // [OOP, IP]
            turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
            river_bet_sizes: [bet_sizes.clone(), bet_sizes],
            turn_donk_sizes: None, // use default bet sizes
            river_donk_sizes: Some(DonkSizeOptions::try_from("50%").unwrap()),
            add_allin_threshold: 1.5, // add all-in if (maximum bet size) <= 1.5x pot
            force_allin_threshold: 0.15, // force all-in if (SPR after the opponent's call) <= 0.15
            merging_threshold: 0.1,
        };

        // build the game tree
        // `ActionTree` can be edited manually after construction
        let action_tree = ActionTree::new(tree_config).unwrap();
        let mut game = PostFlopGame::with_config(card_config, action_tree).unwrap();

        // obtain the private hands
        let oop_cards = game.private_cards(0);
        let oop_cards_str = holes_to_strings(oop_cards).unwrap();
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
        let exploitability = solve(&mut game, max_num_iterations, target_exploitability, true);
        println!("Exploitability: {:.2}", exploitability);

        // get equity and EV of a specific hand
        game.cache_normalized_weights();
        let equity = game.equity(0); // `0` means OOP player
        let ev = game.expected_values(0);
        println!("Equity of oop_hands[0]: {:.2}%", 100.0 * equity[0]);
        println!("EV of oop_hands[0]: {:.2}", ev[0]);

        // get equity and EV of whole hand
        let weights = game.normalized_weights(0);
        let average_equity = compute_average(&equity, weights);
        let average_ev = compute_average(&ev, weights);
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

        let ksjs = holes_to_strings(ip_cards)
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

    fn demo() {
        let oop_range = "66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs,96s+,85s+,75s+,65s,54s";
        let ip_range = "QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+,T7s+,96s+,86s+,75s+,64s+,53s+";
        let card_config = CardConfig {
            range: [oop_range.parse().unwrap(), ip_range.parse().unwrap()],
            flop: flop_from_str("Td9d6h").unwrap(),
            turn: card_from_str("Qc").unwrap(),
            river: NOT_DEALT,
        };
        let bet_sizes = BetSizeOptions::try_from(("60%, e, a", "2.5x")).unwrap();
        let tree_config = TreeConfig {
            initial_state: BoardState::Turn, // must match `card_config`
            starting_pot: 200,
            effective_stack: 900,
            rake_rate: 0.0,
            rake_cap: 0.0,
            flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()], // [OOP, IP]
            turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
            river_bet_sizes: [bet_sizes.clone(), bet_sizes],
            turn_donk_sizes: None, // use default bet sizes
            river_donk_sizes: Some(DonkSizeOptions::try_from("50%").unwrap()),
            add_allin_threshold: 1.5, // add all-in if (maximum bet size) <= 1.5x pot
            force_allin_threshold: 0.15, // force all-in if (SPR after the opponent's call) <= 0.15
            merging_threshold: 0.1,
        };
        let action_tree = ActionTree::new(tree_config).unwrap();
        let mut game = PostFlopGame::with_config(card_config, action_tree).unwrap();
        let oop_cards = game.private_cards(0);
        let oop_cards_str = holes_to_strings(oop_cards).unwrap();
        assert_eq!(
            &oop_cards_str[..10],
            &["5c4c", "Ac4c", "5d4d", "Ad4d", "5h4h", "Ah4h", "5s4s", "As4s", "6c5c", "7c5c"]
        );
        let (mem_usage, mem_usage_compressed) = game.memory_usage();
        println!(
            "Memory usage without compression (32-bit float): {:.2}GB",
            mem_usage as f64 / (1024.0 * 1024.0 * 1024.0)
        );
        println!(
            "Memory usage with compression (16-bit integer): {:.2}GB",
            mem_usage_compressed as f64 / (1024.0 * 1024.0 * 1024.0)
        );
        game.allocate_memory(false);
        let max_num_iterations = 1000;
        let target_exploitability = game.tree_config().starting_pot as f32 * 0.005; // 0.5% of the pot
        let exploitability = solve(&mut game, max_num_iterations, target_exploitability, true);
        println!("Exploitability: {:.2}", exploitability);
        game.cache_normalized_weights();
        let equity = game.equity(0); // `0` means OOP player
        let ev = game.expected_values(0);
        println!("Equity of oop_hands[0]: {:.2}%", 100.0 * equity[0]);
        println!("EV of oop_hands[0]: {:.2}", ev[0]);
        let weights = game.normalized_weights(0);
        let average_equity = compute_average(&equity, weights);
        let average_ev = compute_average(&ev, weights);
        println!("Average equity: {:.2}%", 100.0 * average_equity);
        println!("Average EV: {:.2}", average_ev);
        let actions = game.available_actions();
        assert_eq!(
            format!("{:?}", actions),
            "[Check, Bet(120), Bet(216), AllIn(900)]"
        );
        game.play(1);
        let actions = game.available_actions();
        assert_eq!(format!("{:?}", actions), "[Fold, Call, Raise(300)]");
        let ip_cards = game.private_cards(1);
        let strategy = game.strategy();
        assert_eq!(ip_cards.len(), 250);
        assert_eq!(strategy.len(), 750);
        let ksjs = holes_to_strings(ip_cards)
            .unwrap()
            .iter()
            .position(|s| s == "KsJs")
            .unwrap();
        assert_eq!(strategy[ksjs], 0.0);
        assert!((strategy[ksjs] + strategy[ksjs + 250] + strategy[ksjs + 500] - 1.0).abs() < 1e-6);
        game.play(1);
        assert!(game.is_chance_node());
        let card_7s = card_from_str("7s").unwrap();
        assert!(game.possible_cards() & (1 << card_7s) != 0);
        game.play(card_7s as usize);
        game.back_to_root();
    }

    // Function that uses postflop_solver stuff
    #[pyfunction]
    fn demonstrate() {
        demo();
    }

    #[pyfunction]
    fn big_demo() {
        let starting_pot = 100 * 2;
        let effective_stack = 1000 - 100;

        let rake_rate = 0.0;
        let rake_cap = 0.0;

        // ranges of OOP and IP in string format
        // see the documentation of `Range` for more details about the format
        let oop_range = "66+,A8s+,A5s-A4s,AJo+,K9s+,KQo,QTs+,JTs,96s+,85s+,75s+,65s,54s";
        let ip_range = "QQ-22,AQs-A2s,ATo+,K5s+,KJo+,Q8s+,J8s+,T7s+,96s+,86s+,75s+,64s+,53s+";

        let flop_runout = "Td9d6h";
        let turn_runout = "Qc";

        // bet sizes -> 60% of the pot, geometric size, and all-in
        let bet_size = "60%, e, a";
        // raise sizes -> 2.5x of the previous bet
        let raise_size = "2.5x";

        let max_num_iterations = 1000;

        let card_config = CardConfig {
            range: [oop_range.parse().unwrap(), ip_range.parse().unwrap()],
            flop: flop_from_str(flop_runout).unwrap(),
            turn: card_from_str(turn_runout).unwrap(),
            river: NOT_DEALT,
        };

        // see the documentation of `BetSizeOptions` for more details
        let bet_sizes = BetSizeOptions::try_from((bet_size, raise_size)).unwrap();

        let tree_config = TreeConfig {
            initial_state: BoardState::Turn, // must match `card_config`
            starting_pot: starting_pot,
            effective_stack: effective_stack,
            rake_rate: rake_rate,
            rake_cap: rake_cap,
            flop_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()], // [OOP, IP]
            turn_bet_sizes: [bet_sizes.clone(), bet_sizes.clone()],
            river_bet_sizes: [bet_sizes.clone(), bet_sizes],
            turn_donk_sizes: None, // use default bet sizes
            river_donk_sizes: Some(DonkSizeOptions::try_from("50%").unwrap()),
            add_allin_threshold: 1.5, // add all-in if (maximum bet size) <= 1.5x pot
            force_allin_threshold: 0.15, // force all-in if (SPR after the opponent's call) <= 0.15
            merging_threshold: 0.1,
        };

        // build the game tree
        // `ActionTree` can be edited manually after construction
        let action_tree = ActionTree::new(tree_config).unwrap();
        let mut game = PostFlopGame::with_config(card_config, action_tree).unwrap();

        // obtain the private hands
        let oop_cards = game.private_cards(0);
        let oop_cards_str = holes_to_strings(oop_cards).unwrap();
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
        let target_exploitability = game.tree_config().starting_pot as f32 * 0.005; // 0.5% of the pot
        let exploitability = solve(&mut game, max_num_iterations, target_exploitability, true);
        println!("Exploitability: {:.2}", exploitability);

        // get equity and EV of a specific hand
        game.cache_normalized_weights();
        let equity = game.equity(0); // `0` means OOP player
        let ev = game.expected_values(0);
        println!("Equity of oop_hands[0]: {:.2}%", 100.0 * equity[0]);
        println!("EV of oop_hands[0]: {:.2}", ev[0]);

        // get equity and EV of whole hand
        let weights = game.normalized_weights(0);
        let average_equity = compute_average(&equity, weights);
        let average_ev = compute_average(&ev, weights);
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

        let ksjs = holes_to_strings(ip_cards)
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



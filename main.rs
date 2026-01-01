use rand::prelude::*;

#[derive(Default, Debug, Clone)]
struct GameField {
    money_value: i32,
    field_type: FieldType,
    hotel_price: i32,
    hotel_rent: i32,
    hotel_owner: Option<PlayerId>,
}

#[derive(Debug, Clone, Default)]
enum FieldType {
    #[default]
    Normal,
    OilStock,
    ElectricityStock,
    SteelStock,
    ReturnStocks,
    MoveCasino,
    MoveStockExchange,
    MoveDiceGame,
    MoveHorseRace,
    MoveLottery,
    PayLottery,
    Hotel,
    EverybodyGivesYou5000,
    YouGiveSomeone5000,
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum PlayerId {
    Green,
    Red,
    Blue,
    Yellow,
}

#[derive(Debug)]
struct Player {
    name: PlayerId,
    money: i32,
    position: usize,
    oil_stocks: u8,
    electricity_stocks: u8,
    steel_stocks: u8,
    hotel_built: bool,
    hotel_position: usize,
}

#[derive(Default, Debug)]
struct GameState {
    game_board: Vec<GameField>,
    lottery_account: i32,
}

struct Players {
    players: Vec<Player>,
    active_player: usize,
}
impl Players {
    fn get_active_player(&mut self) -> &mut Player {
        &mut self.players[self.active_player]
    }
    fn get_player(&mut self, player_id: PlayerId) -> Option<&mut Player> {
        self.players
            .iter_mut()
            .find(|player| player.name == player_id)
    }
}

const DEBUG: bool = false;

fn main() {
    let mut green_wins = 0;
    let mut red_wins = 0;
    let mut blue_wins = 0;
    let mut yellow_wins = 0;
    for _ in 0..1000000 {
        let game_result = self::simulate_game();
        match game_result.winner {
            Some(PlayerId::Green) => green_wins += 1,
            Some(PlayerId::Red) => red_wins += 1,
            Some(PlayerId::Blue) => blue_wins += 1,
            Some(PlayerId::Yellow) => yellow_wins += 1,
            None => (),
        }
        if DEBUG {
            println!(
                "Player {:?} has won after {:?} rounds",
                game_result.winner, game_result.rounds
            );
        }
    }

    println!(
        "End Result: Green {:?}, Red {:?}, Blue {:?}, Yellow {:?}",
        green_wins, red_wins, blue_wins, yellow_wins
    );
}

#[derive(Default, Debug)]
struct GameResult {
    winner: Option<PlayerId>,
    rounds: i32,
}

fn simulate_game() -> GameResult {
    let mut game_state = GameState {
        game_board: build_game_board(),
        lottery_account: 0,
    };
    let game_board_size = 68;

    let mut players = Players {
        players: create_players(),
        active_player: 0,
    };

    let mut loop_count = 0;
    while loop_count <= 500 {
        for player_id in 0..players.players.len() {
            players.active_player = player_id;

            // advance player position
            let position_before = players.get_active_player().position;
            {
                let choose = |p1: usize, p2: usize| -> usize {
                    if game_state.game_board[p1].money_value < game_state.game_board[p2].money_value
                    {
                        p1
                    } else {
                        p2
                    }
                };
                let dice_roll = throw_dice();
                let new_position = if position_before >= game_board_size {
                    // special fields: player can go to event, or go back to the main circle
                    match position_before {
                        68 => choose(70, 23 + dice_roll - 1),
                        69 => choose(70, 23 + dice_roll - 2),
                        70 => {
                            if dice_roll == 2 {
                                68
                            } else {
                                23 + dice_roll - 3
                            }
                        }
                        71 => choose(73, 32 + dice_roll - 1),
                        72 => choose(73, 32 + dice_roll - 2),
                        73 => {
                            if dice_roll == 2 {
                                71
                            } else {
                                32 + dice_roll - 3
                            }
                        }
                        74 => choose(76, 57 + dice_roll - 1),
                        75 => choose(76, 57 + dice_roll - 2),
                        76 => {
                            if dice_roll == 2 {
                                74
                            } else {
                                57 + dice_roll - 3
                            }
                        }
                        77 => choose(78, 65 + dice_roll - 1),
                        78 => choose(78, 65 + dice_roll - 2),
                        79 => {
                            if dice_roll == 2 {
                                77
                            } else {
                                65 + dice_roll - 3
                            }
                        }
                        _ => unreachable!(),
                    }
                } else {
                    // start from main circle
                    let new_position = (position_before + dice_roll) % game_board_size;
                    match new_position {
                        // Böse 1
                        24 | 25 | 26 => choose(new_position, new_position + 68 - 44),
                        // Pferde-Rennen
                        32 | 33 | 34 => choose(new_position, new_position + 71 - 32),
                        // Aktien-Börse
                        57 | 58 | 59 => choose(new_position, new_position + 74 - 57),
                        // Casino
                        65 | 66 | 67 => choose(new_position, new_position + 77 - 65),
                        // Default
                        _ => new_position,
                    }
                };
                players.get_active_player().position = new_position;
            };

            play_effect(&mut game_state, &mut players, position_before);

            // check for game completion
            let poorest_player = players
                .players
                .iter()
                .min_by_key(|player| player.money)
                .unwrap();
            if poorest_player.money <= 0 {
                return GameResult {
                    winner: Some(poorest_player.name),
                    rounds: loop_count,
                };
            }
        }
        loop_count += 1
    }
    GameResult {
        winner: None,
        rounds: 0,
    }
}

fn play_effect(game_state: &mut GameState, players: &mut Players, position_before: usize) {
    let active_player = players.get_active_player();
    let player_position = active_player.position;
    let game_field = &mut game_state.game_board[player_position];
    let pre_money = active_player.money;
    active_player.money += game_field.money_value;

    // #44 Kaufe im Vorbeigehen Lotterie-Lose für 5000
    if position_before < 44 && active_player.position >= 44 {
        active_player.money -= 5000;
        game_state.lottery_account += 5000;
    }

    match game_field.field_type {
        FieldType::OilStock => {
            active_player.oil_stocks += 1;
        }
        FieldType::ElectricityStock => {
            active_player.electricity_stocks += 1;
        }
        FieldType::SteelStock => {
            active_player.steel_stocks += 1;
        }
        FieldType::ReturnStocks => {
            active_player.oil_stocks = 0;
            active_player.electricity_stocks = 0;
            active_player.steel_stocks = 0;
        }
        FieldType::MoveCasino => {
            active_player.position = 79;
            active_player.money += get_casino_result();
        }
        FieldType::MoveStockExchange => {
            active_player.position = 76;
            for player in &mut players.players {
                player.money += get_stock_exchange_result(player);
            }
        }
        FieldType::MoveDiceGame => {
            active_player.position = 70;
            active_player.money += get_dice_game_result();
        }
        FieldType::MoveHorseRace => {
            active_player.position = 73;
            for player in &mut players.players {
                player.money += get_horse_race_result();
            }
        }
        FieldType::MoveLottery => {
            active_player.position = 44;
            active_player.money += game_state.lottery_account;
            game_state.lottery_account = 0;
        }
        FieldType::PayLottery => {
            active_player.money -= game_field.money_value;
            game_state.lottery_account += game_field.money_value;
        }
        FieldType::Hotel => {
            if !active_player.hotel_built && game_field.hotel_owner.is_none() {
                active_player.hotel_position = active_player.position;
                game_field.hotel_owner = Some(active_player.name);
                active_player.hotel_built = true;
                active_player.money -= game_field.hotel_price;
            } else if let Some(hotel_owner) = game_field.hotel_owner
                && hotel_owner != active_player.name
            {
                let active_player = players.get_active_player();
                active_player.money -= game_field.hotel_rent;

                let owner = players.get_player(hotel_owner).unwrap();
                owner.money += game_field.hotel_rent;
            }
        }
        FieldType::Normal => {}
        FieldType::EverybodyGivesYou5000 => {
            let active_player_name = active_player.name;
            players.get_active_player().money += players
                .players
                .iter_mut()
                .filter(|p| p.name != active_player_name)
                .map(|p| {
                    p.money -= 5000;
                    5000
                })
                .sum::<i32>();
        }
        FieldType::YouGiveSomeone5000 => {
            // Strategy: assume that players always give to the poorest player (the one closest to a win)
            let active_player_name = active_player.name;
            let poorest_player = players
                .players
                .iter_mut()
                .filter(|p| p.name != active_player_name)
                .min_by_key(|player| player.money);
            if let Some(p) = poorest_player {
                p.money += 5000;
                players.get_active_player().money -= 5000;
            }
        }
    }

    if DEBUG {
        let active_player = players.get_active_player();
        println!(
            "{:?} visited #{} ({:?}). {:?} -> {:?}",
            active_player.name,
            active_player.position,
            game_field.field_type,
            pre_money,
            active_player.money
        );
    }
}

fn throw_dice() -> usize {
    let mut rng = rand::rng();
    let die_one = rng.random_range(1..6);
    let die_two = rng.random_range(1..6);
    die_one + die_two
}

fn get_casino_result() -> i32 {
    let mut rng = rand::rng();
    let mut result: i32 = -70000;
    let roulette = rng.random_range(0..36);
    if roulette % 2 == 1 {
        result += 80000;
    }

    let bandit_one = rng.random_range(1..6);
    let bandit_two = rng.random_range(1..6);
    let bandit_three = rng.random_range(1..6);

    if bandit_one == bandit_two && bandit_two == bandit_three {
        if bandit_one == 6 {
            result += 250000;
        } else {
            result += 150000;
        }
    } else if bandit_one == bandit_two || bandit_one == bandit_three || bandit_two == bandit_three {
        result += 50000;
    }
    if DEBUG {
        println!(
            "Casino result: {:?}, roulette: {:?}, one armed bandit: {:?} {:?} {:?}",
            result, roulette, bandit_one, bandit_two, bandit_three
        );
    }
    result
}

fn get_stock_exchange_result(player: &mut Player) -> i32 {
    let mut rng = rand::rng();
    let stock_event = rng.random_range(1..7);
    match stock_event {
        1 => {
            if DEBUG {
                println!("Stock Exchange: Oil Stock rises");
            }
            player.oil_stocks as i32 * 5000
        }
        2 => {
            if DEBUG {
                println!("Stock Exchange: Oil Stock falls");
            }
            player.oil_stocks as i32 * -10000
        }
        3 => {
            if DEBUG {
                println!("Stock Exchange: Steel Stock rises");
            }
            player.steel_stocks as i32 * 5000
        }
        4 => {
            if DEBUG {
                println!("Stock Exchange: Steel Stock falls");
            }
            player.steel_stocks as i32 * -10000
        }
        5 => {
            if DEBUG {
                println!("Stock Exchange: Electricity Stock rises");
            }
            player.electricity_stocks as i32 * 5000
        }
        6 => {
            if DEBUG {
                println!("Stock Exchange: Electricity Stock falls");
            }
            player.electricity_stocks as i32 * -10000
        }
        7 => {
            if DEBUG {
                println!("Stock Exchange: All Stocks rise");
            }
            (player.oil_stocks + player.steel_stocks + player.electricity_stocks) as i32 * 5000
        }
        _ => unreachable!(),
    }
}

fn get_dice_game_result() -> i32 {
    let mut rng = rand::rng();
    let die_one = rng.random_range(1..6);
    let die_two = rng.random_range(1..6);
    if DEBUG {
        println!("Dice Game Result: {:?} {:?}", die_one, die_two);
    }
    if die_one == 1 && die_two == 1 {
        300000
    } else if die_one == 1 || die_two == 1 {
        100000
    } else {
        0
    }
}

fn get_horse_race_result() -> i32 {
    let mut rng = rand::rng();
    let horse_race = rng.random_range(1..100);
    if DEBUG {
        println!("Horse Race: {:?}", horse_race);
    }
    if horse_race <= 45 { 100000 } else { -50000 }
}

fn create_players() -> Vec<Player> {
    let green_player = Player {
        name: PlayerId::Green,
        money: 1000000,
        position: 0,
        oil_stocks: 0,
        electricity_stocks: 0,
        steel_stocks: 0,
        hotel_built: false,
        hotel_position: 0,
    };
    let red_player = Player {
        name: PlayerId::Red,
        money: 1000000,
        position: 17,
        oil_stocks: 0,
        electricity_stocks: 0,
        steel_stocks: 0,
        hotel_built: false,
        hotel_position: 0,
    };
    let blue_player = Player {
        name: PlayerId::Blue,
        money: 1000000,
        position: 35,
        oil_stocks: 0,
        electricity_stocks: 0,
        steel_stocks: 0,
        hotel_built: false,
        hotel_position: 0,
    };
    let yellow_player = Player {
        name: PlayerId::Yellow,
        money: 1000000,
        position: 50,
        oil_stocks: 0,
        electricity_stocks: 0,
        steel_stocks: 0,
        hotel_built: false,
        hotel_position: 0,
    };
    vec![green_player, red_player, blue_player, yellow_player]
}

fn build_game_board() -> Vec<GameField> {
    let make_field = |money_value: i32, field_type: FieldType| GameField {
        money_value,
        field_type,
        ..Default::default()
    };
    let make_hotel = |hotel_price: i32, hotel_rent: i32| GameField {
        field_type: FieldType::Hotel,
        hotel_price,
        hotel_rent,
        ..Default::default()
    };
    vec![
        make_field(-100000, FieldType::ElectricityStock), // #0
        make_field(0, FieldType::MoveCasino),
        make_field(-170000, FieldType::Normal), // Stifte 170000 für den "Verein anonymer Weltstars e.V."
        make_field(-100000, FieldType::Normal), // Du wirst zum Generalkonsul von Atlantis ernannt. Zahle 100000
        make_hotel(150000, 15000),
        make_field(0, FieldType::MoveStockExchange),
        make_field(-50000, FieldType::PayLottery), // Zahle 50000 in die Lotterie ein
        make_field(-180000, FieldType::Normal),    // Spende 180000 für den EG Butterberg
        make_field(-100000, FieldType::OilStock),
        make_field(0, FieldType::MoveDiceGame),
        make_field(50000, FieldType::Normal), // #10 Deine liebe Oma gibt Dir 50000 Taschengeld
        make_field(-100000, FieldType::ElectricityStock),
        make_field(0, FieldType::MoveHorseRace),
        make_hotel(150000, 15000),
        make_field(0, FieldType::MoveCasino),
        make_field(-100000, FieldType::SteelStock),
        make_field(-50000, FieldType::PayLottery), // Zahle 50000 in die Lotterie ein
        make_field(0, FieldType::MoveCasino),
        make_field(0, FieldType::MoveStockExchange),
        make_field(-10000, FieldType::Normal), // Dein Buch "Alle binären Zahlen auf einen Blick" bringt dir 10000 an Tantiemen ein
        make_field(-100000, FieldType::SteelStock), // #20
        make_field(0, FieldType::Normal), // Du und ein Mitspieler würfeln je 1x. Der höher Wurf bekommt 50.000 vom anderen. => 0 on average
        make_field(0, FieldType::MoveLottery),
        make_field(5000, FieldType::Normal), // #23 Kreuzung: Du verkaufst einem Bierzelt Dachpfannen für 5000. Normaler Weg -> #24, Böse 1 -> #68
        make_field(0, FieldType::MoveStockExchange),
        make_field(0, FieldType::OilStock), // Gratis-Aktie: "Trockenöl-AG"
        make_field(-50000, FieldType::Normal), // Kaufe das Rennpferd "Schlußlicht" für 50000
        make_field(0, FieldType::MoveLottery),
        make_field(-100000, FieldType::OilStock),
        make_field(10000, FieldType::Normal), // Das Finanzamt schenkt Dir 10000
        make_hotel(50000, 5000),              // #30
        make_field(0, FieldType::MoveDiceGame), // #31 Kreuzung: Normaler Weg -> #32, Pferderennen -> #71
        make_field(0, FieldType::EverybodyGivesYou5000), // Jeder Spieler gibt dir 5000
        make_field(-10000, FieldType::Normal), // Du bist Sponsor der Hochsee-Regatta "Rund um die Schweiz". Zahle 10000
        make_field(0, FieldType::MoveCasino),
        make_field(-100000, FieldType::SteelStock),
        make_field(0, FieldType::MoveHorseRace),
        make_field(0, FieldType::MoveStockExchange),
        make_field(-100000, FieldType::OilStock),
        make_field(0, FieldType::ReturnStocks),
        make_hotel(200000, 20000), // #40
        make_field(0, FieldType::MoveCasino),
        make_field(-100000, FieldType::ElectricityStock),
        make_field(-150000, FieldType::Normal), // Kaufe 150 Strandkörbe am Nördlichen Eismeer für 150000
        make_field(0, FieldType::PayLottery),   // #44 Kaufe im Vorbeigehen Lotterie-Lose für 5000
        make_field(10000 / 6, FieldType::Normal), // Du würfelst einmal mit einem Würfel: Für eine 6 gibt's 10000 => use average
        make_field(0, FieldType::MoveDiceGame),
        make_field(0, FieldType::MoveCasino),
        make_hotel(100000, 10000),
        make_field(-100000, FieldType::SteelStock),
        make_field(-5000, FieldType::YouGiveSomeone5000), // #50 Gib einem Mitspieler 5000
        make_field(0, FieldType::MoveDiceGame),
        make_field(-7500, FieldType::Normal), // TODO: Gib jedem Mitspieler 5000 der etwas blaues trägt
        make_field(-100000, FieldType::ElectricityStock),
        make_field(0, FieldType::MoveLottery),
        make_field(100000, FieldType::Normal), // Ein Scheich schenkt dir seine Ölquelle. Du verkaufst sie für 100000 weiter
        make_field(-25000, FieldType::Normal), // #56 Kreuzung: Zahle 25000 Kaution für den Mörderer des Toten Meeres. Normaler Weg -> #57, Aktien-Börse -> #74
        make_field(0, FieldType::MoveCasino),
        make_field(-20000, FieldType::Normal), // Zahle 20000 für eine Filmexpedition über den Hochzeitstanz der Alpenhörner
        make_field(-100000, FieldType::ElectricityStock),
        make_field(0, FieldType::MoveDiceGame), // #60
        make_field(0, FieldType::MoveHorseRace),
        make_field(-100000, FieldType::SteelStock),
        make_hotel(100000, 10000),
        make_field(-100000, FieldType::OilStock), // #64 Kreuzung: Normaler Weg -> #65, Casino -> #77
        make_field(0, FieldType::ElectricityStock), // Gratis-Aktie: Kurzschluß-Versorgungs-AG
        make_field(0, FieldType::MoveStockExchange),
        make_field(10000, FieldType::Normal), // # 67 Du gewinnst bei einem Fernseh-Quiz den Trostpreis von 10000
        // Weg zur Bösen 1
        make_field(-5000, FieldType::Normal), // #68  Miete für 5000 die "Titanic" für eine Kreuzfahrt
        make_field(-5000, FieldType::Normal), // #69 Werde für 5000 Mitglied im Club Absoluter Relativisten
        make_field(0, FieldType::MoveCasino), // #70 Böse 1
        // Weg zum Pferde-Rennen
        make_field(-30000, FieldType::Normal), // #71 Laß dir einen Logenplatz für das Pferderennen auf dem Nürburgring reservieren. Zahle 30000
        make_field(-20000, FieldType::Normal), // #72 Ersteigere für 20000 das antike Motorboot von Ramses II
        make_field(0, FieldType::MoveHorseRace), // #73 Pferderennen
        // Weg zur Aktien-Börse
        make_field(-10000, FieldType::Normal), // #74 Zahle 10000 für einen Brilliant-Zahnstocher
        make_field(-40000, FieldType::Normal), // #75 Finanziere mit 40000 die Erforschung des magnetischen Westpols der Erde.
        make_field(0, FieldType::MoveStockExchange), // #76 Aktien-Börse
        // Weg zum Casino
        make_field(-10000, FieldType::Normal), // #77 Stifte 10000 für die "Sonne-um-Erde"-Gesellschaft
        make_field(-40000, FieldType::Normal), // #78 Versichere für 40000 ein Wüstenschiff gegen das Auflaufen auf eine Sandbank.
        make_field(0, FieldType::MoveCasino),  // #79 Casino
    ]
}

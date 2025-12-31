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
    let game_board_size = game_state.game_board.len();

    let mut players = Players {
        players: create_players(),
        active_player: 0,
    };

    let mut loop_count = 0;
    while loop_count <= 500 {
        for player_id in 0..players.players.len() {
            players.active_player = player_id;
            players.players[player_id].throw_dice(game_board_size);
            play_effect(&mut game_state, &mut players);

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

fn play_effect(game_state: &mut GameState, players: &mut Players) {
    let active_player = players.get_active_player();
    let player_position = active_player.position;
    let game_field = &mut game_state.game_board[player_position];
    let pre_money = active_player.money;
    active_player.money += game_field.money_value;
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
            active_player.position = 61;
            active_player.money += get_casino_result();
        }
        FieldType::MoveStockExchange => {
            active_player.position = 53;
            for player in &mut players.players {
                player.money += get_stock_exchange_result(player);
            }
        }
        FieldType::MoveDiceGame => {
            active_player.position = 20;
            active_player.money += get_dice_game_result();
        }
        FieldType::MoveHorseRace => {
            active_player.position = 28;
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
    }

    if DEBUG {
        let active_player = players.get_active_player();
        println!(
            "{:?} visited {:?}. {:?} -> {:?}",
            active_player.name, game_field.field_type, pre_money, active_player.money
        );
    }
}

impl Player {
    fn throw_dice(&mut self, game_board_size: usize) {
        let mut rng = rand::rng();
        let die_one = rng.random_range(1..6);
        let die_two = rng.random_range(1..6);
        let new_pos = (self.position + die_one + die_two) % game_board_size;
        self.position = new_pos as usize;
    }
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
        make_field(-100000, FieldType::ElectricityStock),
        make_field(0, FieldType::MoveCasino),
        make_field(-170000, FieldType::Normal),
        make_field(-100000, FieldType::Normal),
        make_hotel(150000, 15000),
        make_field(0, FieldType::MoveStockExchange),
        make_field(-50000, FieldType::PayLottery),
        make_field(-180000, FieldType::Normal),
        make_field(-100000, FieldType::OilStock),
        make_field(0, FieldType::MoveDiceGame),
        make_field(-50000, FieldType::Normal),
        make_field(-100000, FieldType::ElectricityStock),
        make_field(0, FieldType::MoveHorseRace),
        make_hotel(150000, 15000),
        make_field(0, FieldType::MoveCasino),
        make_field(-100000, FieldType::SteelStock),
        make_field(-50000, FieldType::PayLottery),
        make_field(0, FieldType::MoveCasino),
        make_field(0, FieldType::MoveStockExchange),
        make_field(-10000, FieldType::Normal),
        make_field(-100000, FieldType::SteelStock),
        make_field(-25000, FieldType::Normal), // Du und ein Mitspieler würfeln je 1x. Der höher Wurf bekommt 50.000 vom anderen.
        make_field(0, FieldType::MoveLottery),
        make_field(5000, FieldType::Normal),
        make_field(0, FieldType::MoveStockExchange),
        make_field(-100000, FieldType::OilStock),
        make_field(-50000, FieldType::Normal),
        make_field(0, FieldType::MoveLottery),
        make_field(-100000, FieldType::OilStock),
        make_field(-10000, FieldType::Normal),
        make_hotel(50000, 5000),
        make_field(0, FieldType::MoveDiceGame),
        make_field(7500, FieldType::Normal), // TODO: Jeder Spieler gibt dir 5000
        make_field(-10000, FieldType::Normal),
        make_field(0, FieldType::MoveCasino),
        make_field(-100000, FieldType::SteelStock),
        make_field(0, FieldType::MoveHorseRace),
        make_field(0, FieldType::MoveStockExchange),
        make_field(-100000, FieldType::OilStock),
        make_field(0, FieldType::ReturnStocks),
        make_hotel(200000, 20000),
        make_field(0, FieldType::MoveCasino),
        make_field(-100000, FieldType::ElectricityStock),
        make_field(-150000, FieldType::Normal),
        make_field(-10000, FieldType::PayLottery),
        make_field(1500, FieldType::Normal), // TODO: Du würfelst einmal mit einem Würfel: Für eine 6 gibt's 10000
        make_field(0, FieldType::MoveDiceGame),
        make_field(0, FieldType::MoveCasino),
        make_hotel(100000, 10000),
        make_field(-100000, FieldType::SteelStock),
        make_field(-5000, FieldType::Normal), // TODO: Gib einem Mitspieler 5000
        make_field(0, FieldType::MoveDiceGame),
        make_field(-7500, FieldType::Normal), // TODO: Gib jedem Mitspieler 5000 der etwas blaues trägt
        make_field(-100000, FieldType::ElectricityStock),
        make_field(0, FieldType::MoveLottery),
        make_field(100000, FieldType::Normal),
        make_field(-25000, FieldType::Normal),
        make_field(0, FieldType::MoveCasino),
        make_field(-20000, FieldType::Normal),
        make_field(-100000, FieldType::ElectricityStock),
        make_field(0, FieldType::MoveDiceGame),
        make_field(0, FieldType::MoveHorseRace),
        make_field(-100000, FieldType::SteelStock),
        make_hotel(100000, 10000),
        make_field(-100000, FieldType::SteelStock),
        make_field(0, FieldType::ElectricityStock),
        make_field(0, FieldType::MoveStockExchange),
        make_field(10000, FieldType::Normal),
    ]
}

extern crate rand;
use rand::prelude::*;

#[derive(Default, Debug, Clone)]
struct GameField {
    money_value: i32,
    field_type: FieldType,
    hotel_price: i32,
    hotel_rent: i32,
    hotel_built: bool,
}

#[derive(Debug, Clone)]
enum FieldType {
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
impl Default for FieldType {
    fn default() -> Self {
        FieldType::Normal
    }
}

#[derive(Default, Debug)]
struct Player {
    name: String,
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

fn main() {
    let game_state = &mut GameState {
        game_board: self::build_game_board(),
        lottery_account: 0,
    };
    let game_board_size = game_state.game_board.len();

    let mut players = self::create_players();

    println!("Status: Game board has {:?} fields", game_board_size);

    let mut loop_count = 0;
    let mut game_over = false;
    while !game_over {
        for player in &mut players {
            player.throw_dice(game_board_size);
            player.play_effect(game_state);
            println!(
                "{:?}: Player {:?} is on field {:?}: {:?} with {:?} dollar",
                loop_count,
                player.name,
                player.position,
                game_state.game_board[player.position].field_type,
                player.money
            );

            if player.money <= 0 {
                game_over = true;
                break;
            }
        }
        loop_count += 1
    }

    println!("Player status {:?}", players[0]);
}

trait PlayerFunctions {
    fn play_effect(&mut self, game_state: &mut GameState);
    fn throw_dice(&mut self, game_board_size: usize);
}

impl PlayerFunctions for Player {
    fn play_effect(&mut self, game_state: &mut GameState) {
        let game_field = &mut game_state.game_board[self.position];
        self.money += game_field.money_value;
        match game_field.field_type {
            FieldType::OilStock => {
                self.oil_stocks += 1;
            }
            FieldType::ElectricityStock => {
                self.electricity_stocks += 1;
            }
            FieldType::SteelStock => {
                self.steel_stocks += 1;
            }
            FieldType::ReturnStocks => {
                self.oil_stocks = 0;
                self.electricity_stocks = 0;
                self.steel_stocks = 0;
            }
            FieldType::MoveCasino => {
                self.position = 61;
                self.money += get_casino_result();
            }
            FieldType::MoveStockExchange => {
                self.position = 53;
                self.money += get_stock_exchange_result(self);
            }
            FieldType::MoveDiceGame => {
                self.position = 20;
                self.money += get_dice_game_result();
            }
            FieldType::MoveHorseRace => {
                self.position = 28;
                self.money += get_horse_race_result();
            }
            FieldType::MoveLottery => {
                self.position = 44;
                self.money += game_state.lottery_account;
                game_state.lottery_account = 0;
            }
            FieldType::PayLottery => {
                game_state.lottery_account += game_field.money_value;
            }
            FieldType::Hotel => {
                if self.hotel_built == false && game_field.hotel_built == false {
                    self.hotel_position = self.position;
                    game_field.hotel_built = true;
                    self.hotel_built = true;
                    self.money -= game_field.hotel_price;
                } else if game_field.hotel_built == true && self.hotel_position != self.position {
                    self.money -= game_field.hotel_rent;
                }
            }
            _ => {}
        }
    }

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
    return result;
}

fn get_stock_exchange_result(player: &mut Player) -> i32 {
    let mut rng = rand::rng();
    let stock_event = rng.random_range(1..7);
    match stock_event {
        1 => {
            return (player.oil_stocks as i32 * 5000).into();
        }
        2 => {
            return (player.oil_stocks as i32 * -10000).into();
        }
        3 => {
            return (player.steel_stocks as i32 * 5000).into();
        }
        4 => {
            return (player.steel_stocks as i32 * -10000).into();
        }
        5 => {
            return (player.electricity_stocks as i32 * 5000).into();
        }
        6 => {
            return (player.electricity_stocks as i32 * -10000).into();
        }
        7 => {
            return ((player.oil_stocks + player.steel_stocks + player.electricity_stocks) as i32
                * 5000)
                .into();
        }
        _ => return 0,
    }
}

fn get_dice_game_result() -> i32 {
    let mut rng = rand::rng();
    let die_one = rng.random_range(1..6);
    let die_two = rng.random_range(1..6);
    if die_one == 1 && die_two == 1 {
        return 300000;
    } else if die_one == 1 || die_two == 1 {
        return 100000;
    }
    return 0;
}

fn get_horse_race_result() -> i32 {
    let mut rng = rand::rng();
    let horse_race = rng.random_range(1..100);
    if horse_race <= 45 {
        return 100000;
    } else {
        return -50000;
    }
}

fn create_players() -> Vec<Player> {
    let mut players = Vec::<Player>::new();
    let green_player = Player {
        name: String::from("Green"),
        money: 1000000,
        position: 0,
        ..Default::default()
    };
    let red_player = Player {
        name: String::from("Red"),
        money: 1000000,
        position: 17,
        ..Default::default()
    };
    /*let blue_player = Player {
        name: String::from("Blue"),
        money: 1000000,
        position: 35,
        ..Default::default()
    };
    let yellow_player = Player {
        name: String::from("Yellow"),
        money: 1000000,
        position: 50,
        ..Default::default()
    };*/
    players.push(green_player);
    players.push(red_player);
    return players;
}

fn build_game_board() -> Vec<GameField> {
    let mut game_board = Vec::<GameField>::new();
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::ElectricityStock,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveCasino,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -170000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::Hotel,
        hotel_price: 150000,
        hotel_rent: 15000,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveStockExchange,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -50000,
        field_type: FieldType::PayLottery,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -180000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::OilStock,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveDiceGame,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -50000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::ElectricityStock,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -170000,
        field_type: FieldType::MoveHorseRace,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::Hotel,
        hotel_price: 150000,
        hotel_rent: 15000,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveCasino,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::SteelStock,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -50000,
        field_type: FieldType::PayLottery,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveCasino,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveStockExchange,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -10000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::SteelStock,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -25000, // Du und ein Mitspieler würfeln je 1x. Der höher Wurf bekommt 50.000 vom anderen.
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -50000,
        field_type: FieldType::MoveLottery,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: 5000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveStockExchange,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::OilStock,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -50000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveLottery,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::OilStock,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -10000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::Hotel,
        hotel_price: 50000,
        hotel_rent: 5000,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveDiceGame,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: 7500, // TODO: Jeder Spieler gibt dir 5000
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -10000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveCasino,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::SteelStock,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveHorseRace,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveStockExchange,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::OilStock,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::ReturnStocks,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::Hotel,
        hotel_price: 200000,
        hotel_rent: 20000,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveCasino,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::ElectricityStock,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -150000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -10000,
        field_type: FieldType::PayLottery,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: 1500, // TODO: Du würfelst einmal mit einem Würfel: Für eine 6 gibt's 10000
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveDiceGame,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveCasino,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::Hotel,
        hotel_price: 100000,
        hotel_rent: 10000,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::SteelStock,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -5000, // TODO: Gib einem Mitspieler 5000
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveDiceGame,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -7500, // TODO: Gib jedem Mitspieler 5000 der etwas blaues trägt
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::ElectricityStock,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveLottery,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: 100000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -25000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveCasino,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -20000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::ElectricityStock,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveDiceGame,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveHorseRace,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::SteelStock,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::Hotel,
        hotel_price: 100000,
        hotel_rent: 10000,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: -100000,
        field_type: FieldType::SteelStock,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::ElectricityStock,
        ..Default::default()
    });
    game_board.push(GameField {
        field_type: FieldType::MoveStockExchange,
        ..Default::default()
    });
    game_board.push(GameField {
        money_value: 10000,
        field_type: FieldType::Normal,
        ..Default::default()
    });
    return game_board;
}

use std::fs::Metadata;
use std::iter::Map;
use std::sync::mpsc::Receiver;
use phf::phf_map;
use std::cmp::Ordering;


fn main() {
    println!("Hello, world!");

    let round_number: u8 = 0;
    let mut tick: u8 = 0; // 0, 1, 2
    let mut order : Order = random_order();
    let mut curr_player : player_color = order[0];
    //random players Order


    while round( ) {
        if tick == 2 {
            order = players_order();
            tick = 0;
        } else {
            tick += 1;
        }
    }
}

type GameEnd = bool;
type Order = [player_color; 3];

fn random_order() -> Order {
    todo!()
}

fn prompt_order() -> Order {
    todo!()
}

fn players_order(players: &[&Player; 3], mut order: &mut Order) -> Order {
    let mut min_player: Option<&Player> = None;

    for &player in players.iter() {
        if let Some(current) = min_player {
            if player < current {
                min_player = Some(player);
            }
        } else if player.has_any_piramid() {
            min_player = Some(player);
        }
    }

    if min_player.is_some() {
        *order = prompt_order();
    } else {
        *order = random_order();
    }
    *order
}

enum base_piramid_id{
    go,
    money,
    piramid_up,
    red,
    white,
    blue,
    buy,
}
struct base_piramid{
    lvl_0 : [base_piramid_id; 4],
    lvl_1 : [base_piramid_id; 3],
    lvl_2 : [base_piramid_id; 2],
    piramid_construct :Vec<Vec<bool>>,
}
struct player_card{
    money_points : u8,
    base_piramid: base_piramid,
}
impl base_piramid {
    fn new() -> Self {
        Self {
            lvl_0: [base_piramid_id::white, base_piramid_id::red, base_piramid_id::blue, base_piramid_id::money],
            lvl_1: [base_piramid_id::piramid_up, base_piramid_id::go, base_piramid_id::money],
            lvl_2: [base_piramid_id::go, base_piramid_id::money],
            piramid_construct: vec![
                vec![false, false, false, false], // lvl 0 (size 4)
                vec![false, false,false],               // lvl 1 (size 2)
                vec![false, false],               // lvl 2 (size 2)
            ],
        }
    }

    fn can_set_lvl(&self, level: usize)->bool{
        let mut used : u8 = 0; //5
        let mut lvl_empty : u8 = 0;
        for lvl in self.piramid_construct.iter() {
            let mut tmp = true;
            for b in lvl{
                if b{
                    used += 1;
                    tmp = false;
                }
            }
            if tmp {
                lvl_empty += 1;
            }
        }
        if used < 3{
            true
        }

        if lvl_empty == 0{
            true
        }

        if used == 3{
            if lvl_empty == 1{
                true
            }else{
                for b in self.piramid_construct[level]{
                    if b{
                        false
                    }
                }
                true

            }
        }else{
           for b in self.piramid_construct[level]{
                    if b{
                        false
                    }
                }
                true
        }

    }
    fn set(&mut self, level: usize, index: usize) -> bool{
        if !self.can_set_lvl(level){
            return false
        }

        //to do
       if self.piramid_construct[level][index] == true{
           false
       } else{
           self.piramid_construct[level][index] = true;
           true
       }
    }
}
impl base_piramid{

}

fn round(curr_player: player_color, players: &[&mut Player; 3], map: &mut game_map_type, round_number: u8, mut tick: u8) -> GameEnd {
    // jaka akcja (decyzja) => kupno kart, chodzenie, kasa, piramida upgrade
    //jesli wojna => wojna


    true
}


trait PyramidsInfo {
    fn total_piramid_levels(&self) -> u8;
    fn has_any_piramid(&self) -> bool {
        self.total_piramid_levels() > 0
    }
}

impl PyramidsInfo for Player {
    fn total_piramid_levels(&self) -> u8 {
        self.piramids.0.lvl + self.piramids.1.lvl + self.piramids.2.lvl
    }
}
enum CardType {
    Day,
    Night,
    War,
}
enum Cards {
    ToBuy {
        name: &'static str,
        card_type: CardType,
        lvl: u8,
        cost: u8,
        attack: u8,
        dmg: u8,
        defense: u8,
        has_boost: bool,
        boost: Vec<u8>, // referce the boost table with boost and idx's
    },
    GodsIntervension {
        name: &'static str,
        card_type: CardType,
        cost: u8,
        attack: u8,
        dmg: u8,
        defense: u8,
        has_boost: bool,
        boost: Vec<u8>,
    },

}

enum MosterName {
    a,
    b,
    c,
}
struct Monster {
    name: MosterName,
    attack: u8,
    defense: u8,
    dmg: u8,

}

struct Piramid {
    color: &'static player_color,
    lvl: u8,
}
struct Squad {
    size: u8,
    id: u8,
    idx: idx,
    has_monster: bool,
    monster: Monster,
}

enum MoveType {
    a,
    b,
    c,
    d,
    e,
    f,
    g,
}

enum WinType {
    perm,
    temp(Id : u8),
}

type Id = u8;
type idx = u8;

struct Player {
    name: String,
    color: player_color, //color
    money: u8,
    moves: [MoveType; 8],
    cards_bought: Vec<Cards::ToBuy>,
    cards_gods: Vec<Cards::GodsIntervension>,
    win_tokens: Vec<WinType>,
    squads: Vec<Squad>,
    player_card: player_card,
    piramids: (Piramid, Piramid, Piramid),
}

impl PartialEq for Player {
    fn eq(&self, other: &Self) -> bool {
        self.total_piramid_levels() == other.total_piramid_levels()
    }
}
impl Eq for Player {}

impl PartialOrd for Player {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Player {
    fn cmp(&self, other: &Self) -> Ordering {
        self.total_piramid_levels().cmp(&other.total_piramid_levels())
    }
}

static
enum cost_type {
    money,
    soldier,
}

enum gift_type {
    money,
    win_card,
}


struct temple_boost {
    name: &'static str,
    cost_number: u8,
    cost_type: cost_type,
    gift: gift_type,

}

enum buildings {
    Castle {
        piramids: [Piramid; 3],
    },
    Temple {
        idx: u8,
        temple_boost: temple_boost,
    },
    None,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum player_color {
    red,
    black,
    yellow,
    none,
}

struct game {
    round_nr: u8,
    kolejnosc: [player_color; 3],
    player_turn: player_color,
    end: bool,
    gods_cards: Vec<Cards::GodsIntervension>,
    gods_cards_n: u8,
    buy_cards: Vec<Cards::ToBuy>,
}

enum Choice {
    Go,
    Sell,
}

fn sum_attack(player: &Player, idx: idx) -> u16 {
    let mut sum_attack: u16 = 0;
    for squad in player.squads {
        if squad.idx == idx {
            sum_attack = sum_attack + squad.size as u16;
            if squad.has_monster {
                sum_attack += squad.monster.attack as u16;
            }
        }
    }

    for card in player.cards_bought {
        if card.card_type == CardType::War {
            sum_attack += card.attack;
        }
    }

    sum_attack
}

fn sum_dmg(player: &Player, dmg: u16) -> u16 {
    let mut sum_dmg: u16 = dmg;

    for card in player.cards_bought {
        if card.card_type == CardType::War {
            sum_dmg += card.dmg;
        }
    }

    sum_dmg
}

fn use_gods_inter(player: &Player, attack_out: [u16; 2], dmg_out: [u16; 2]) -> ([u16; 2], [u16; 2]) {
    todo!()
}

fn war(mut player_1: &mut Player, mut player_2: &mut Player, idx: idx) {
    let mut attack: [u16; 2] = [0, 0];
    let mut dmg: [u16; 2] = [0, 0];

    attack[0] = sum_attack(&player_1, idx);
    attack[1] = sum_attack(&player_2, idx);

    (attack, dmg) = use_gods_inter(&player_1, attack, dmg);
    (attack, dmg) = use_gods_inter(&player_2, attack, dmg);

    dmg[0] = sum_dmg(player_1, dmg[0]);
    dmg[1] = sum_dmg(player_2, dmg[1]);

    enum WarEndResult {
        Winner1,
        Winner2,
        Draw,
    }

    let mut winner = WarEndResult::Winner1;
    if (attack[1] > attack[0]) {
        winner = WarEndResult::Winner2;
    } else if (attack[0] == attack[1]) {
        winner = WarEndResult::Draw;
    }

    match winner {
        WarEndResult::Draw => draw(&mut player_1, &mut player_2),
        WarEndResult::Winner1 => win(&mut player_1, &mut player_2),
        WarEndResult::Winner2 => win(&mut player_2, &mut player_1),
    }
}

fn draw(player_1: &mut Player, player_2: &mut Player) {
    todo!()
}


fn send_to(receiver: player_color, text: &str) {
    match receiver {
        player_color::black => todo!(),
        player_color::red => todo!(),
        player_color::yellow => todo!(),
        _ => todo!(),
    }
}

fn win(mut winner: &mut Player, mut looser: &mut Player) {
    send_to(winner.color, "You won!");
    send_to(looser.color, "You lost!");
    looser_decition(&looser);
    todo!()
}

fn looser_decition(looser: &Player) {
    send_to(looser.color, "choose");
    //handle Choice

    let choice = Choice::Go; // change to user Choice

    match choice {
        Choice::Go => todo!(),
        Choice::Sell => todo!(),
    }
}


type game_map_type = [Point; 20];

struct Point {
    name: &'static point_type,
    players: Vec<Player>,
    squads_id: Vec<Id>,
    has_teleport: bool,
    neighbours: &'static [&'static point_type], //by name
    building_type: buildings,
}

enum point_type {
    z11,
    z12,
    z13,
    z21,
    z22,
    z23,
    z31,
    z32,
    z33,
    t1,
    t2,
    t3,
    t4,
    p1,
    p2,
    p3,
    p4,
    p5,
    p6,
    p7,
}
static game_map: [Point; 20] = [
    Point {
        name: &point_type::z11,
        players: vec![],
        squads_id: vec![],
        has_teleport: true,
        neighbours: &[&point_type::p1],
        building_type: buildings::Castle {
            piramids: [
                Piramid { color: &player_color::black, lvl: 0 },
                Piramid { color: &player_color::yellow, lvl: 0 },
                Piramid { color: &player_color::red, lvl: 0 },
            ],
        },
    },
    Point {
        name: &point_type::z12,
        players: vec![],
        squads_id: vec![],
        has_teleport: true,
        neighbours: &[&point_type::p1],
        building_type: buildings::Castle {
            piramids: [
                Piramid { color: &player_color::black, lvl: 0 },
                Piramid { color: &player_color::yellow, lvl: 0 },
                Piramid { color: &player_color::red, lvl: 0 },
            ],
        },
    },
    Point {
        name: &point_type::z13,
        players: vec![],
        squads_id: vec![],
        has_teleport: true,
        neighbours: &[&point_type::p1],
        building_type: buildings::Castle {
            piramids: [
                Piramid { color: &player_color::black, lvl: 0 },
                Piramid { color: &player_color::yellow, lvl: 0 },
                Piramid { color: &player_color::red, lvl: 0 },
            ],
        },
    },
    Point {
        name: &point_type::z21,
        players: vec![],
        squads_id: vec![],
        has_teleport: true,
        neighbours: &[&point_type::p4],
        building_type: buildings::Castle {
            piramids: [
                Piramid { color: &player_color::black, lvl: 0 },
                Piramid { color: &player_color::yellow, lvl: 0 },
                Piramid { color: &player_color::red, lvl: 0 },
            ],
        },
    },
    Point {
        name: &point_type::z22,
        players: vec![],
        squads_id: vec![],
        has_teleport: true,
        neighbours: &[&point_type::p4],
        building_type: buildings::Castle {
            piramids: [
                Piramid { color: &player_color::black, lvl: 0 },
                Piramid { color: &player_color::yellow, lvl: 0 },
                Piramid { color: &player_color::red, lvl: 0 },
            ],
        },
    },
    Point {
        name: &point_type::z23,
        players: vec![],
        squads_id: vec![],
        has_teleport: true,
        neighbours: &[&point_type::p4],
        building_type: buildings::Castle {
            piramids: [
                Piramid { color: &player_color::black, lvl: 0 },
                Piramid { color: &player_color::yellow, lvl: 0 },
                Piramid { color: &player_color::red, lvl: 0 },
            ],
        },
    },
    Point {
        name: &point_type::z31,
        players: vec![],
        squads_id: vec![],
        has_teleport: true,
        neighbours: &[&point_type::p6],
        building_type: buildings::Castle {
            piramids: [
                Piramid { color: &player_color::black, lvl: 0 },
                Piramid { color: &player_color::yellow, lvl: 0 },
                Piramid { color: &player_color::red, lvl: 0 },
            ],
        },
    },
    Point {
        name: &point_type::z32,
        players: vec![],
        squads_id: vec![],
        has_teleport: true,
        neighbours: &[&point_type::p6],
        building_type: buildings::Castle {
            piramids: [
                Piramid { color: &player_color::black, lvl: 0 },
                Piramid { color: &player_color::yellow, lvl: 0 },
                Piramid { color: &player_color::red, lvl: 0 },
            ],
        },
    },
    Point {
        name: &point_type::z33,
        players: vec![],
        squads_id: vec![],
        has_teleport: true,
        neighbours: &[&point_type::p6],
        building_type: buildings::Castle {
            piramids: [
                Piramid { color: &player_color::black, lvl: 0 },
                Piramid { color: &player_color::yellow, lvl: 0 },
                Piramid { color: &player_color::red, lvl: 0 },
            ],
        },
    },
    // t* -> Temples (idx, boost "none" placeholder)
    Point {
        name: &point_type::t1,
        players: vec![],
        squads_id: vec![],
        has_teleport: false,
        neighbours: &[
            &point_type::p1,
            &point_type::p2,
            &point_type::p4,
            &point_type::p5,
            &point_type::p6,
        ],
        building_type: buildings::Temple {
            idx: 1,
            temple_boost: temple_boost {
                name: "t1",
                cost_number: 0,
                cost_type: cost_type::money,
                gift: gift_type::money,
            },
        },
    },
    Point {
        name: &point_type::t2,
        players: vec![],
        squads_id: vec![],
        has_teleport: false,
        neighbours: &[&point_type::p3],
        building_type: buildings::Temple {
            idx: 2,
            temple_boost: temple_boost {
                name: "t2",
                cost_number: 0,
                cost_type: cost_type::money,
                gift: gift_type::money,
            },
        },
    },
    Point {
        name: &point_type::t3,
        players: vec![],
        squads_id: vec![],
        has_teleport: true,
        neighbours: &[&point_type::p7],
        building_type: buildings::Temple {
            idx: 3,
            temple_boost: temple_boost {
                name: "t3",
                cost_number: 0,
                cost_type: cost_type::money,
                gift: gift_type::money,
            },
        },
    },
    Point {
        name: &point_type::t4,
        players: vec![],
        squads_id: vec![],
        has_teleport: true,
        neighbours: &[&point_type::p7],
        building_type: buildings::Temple {
            idx: 4,
            temple_boost: temple_boost {
                name: "t4",
                cost_number: 0,
                cost_type: cost_type::money,
                gift: gift_type::money,
            },
        },
    },
    // p* -> plain (None)
    Point {
        name: &point_type::p1,
        players: vec![],
        squads_id: vec![],
        has_teleport: false,
        neighbours: &[
            &point_type::p2,
            &point_type::p3,
            &point_type::t1,
            &point_type::z11,
            &point_type::z12,
            &point_type::z13,
        ],
        building_type: buildings::None,
    },
    Point {
        name: &point_type::p2,
        players: vec![],
        squads_id: vec![],
        has_teleport: true,
        neighbours: &[
            &point_type::p1,
            &point_type::p3,
            &point_type::p4,
            &point_type::t1,
        ],
        building_type: buildings::None,
    },
    Point {
        name: &point_type::p3,
        players: vec![],
        squads_id: vec![],
        has_teleport: false,
        neighbours: &[
            &point_type::p1,
            &point_type::p2,
            &point_type::p4,
            &point_type::t2,
        ],
        building_type: buildings::None,
    },
    Point {
        name: &point_type::p4,
        players: vec![],
        squads_id: vec![],
        has_teleport: false,
        neighbours: &[
            &point_type::p2,
            &point_type::p3,
            &point_type::p5,
            &point_type::t1,
        ],
        building_type: buildings::None,
    },
    Point {
        name: &point_type::p5,
        players: vec![],
        squads_id: vec![],
        has_teleport: true,
        neighbours: &[
            &point_type::p3,
            &point_type::p4,
            &point_type::p6,
            &point_type::t1,
            &point_type::t2,
        ],
        building_type: buildings::None,
    },
    Point {
        name: &point_type::p6,
        players: vec![],
        squads_id: vec![],
        has_teleport: false,
        neighbours: &[
            &point_type::p3,
            &point_type::p5,
            &point_type::z31,
            &point_type::z32,
            &point_type::z33,
        ],
        building_type: buildings::None,
    },
    Point {
        name: &point_type::p7,
        players: vec![],
        squads_id: vec![],
        has_teleport: false,
        neighbours: &[&point_type::t4, &point_type::t3],
        building_type: buildings::None,
    },
];

/*
(have)
z11(tel): p1
z12(tel): p1
z13(tel): p1
z21(tel): p4
z22(tel): p4
z23(tel): p4
z31(tel): p6
z32(tel): p6
z33(tel): p6
t1: p1, p2, p4, p5, p6
t2: p3
t3(tel): p7
t4(tel):
p1; p2, p3, t1, z1_1, z1_2, z1_3
p2(tel); p1, p3, p4, t1
p3: p1, p2, p4, t2
p4: p2, p3, p5, t1
p5(tel): p3, p4, p6, t1, t2
p6: p3, p5, z_3_1, z_3_2, z_3_3
p7: t4

 */
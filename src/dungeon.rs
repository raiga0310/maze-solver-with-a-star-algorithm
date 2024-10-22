use std::{fs, io::Read, ops::Add, thread::sleep, time};

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum State {
    #[default]
    Empty,
    Aisle,
    Goal,
    Start,
    Wall,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum Direction {
    #[default]
    N,
    Up,
    Down,
    Left,
    Right,
}

fn from_direction(direction: Direction) -> Point {
    match direction {
        Direction::Up => Point {
            pn: false,
            x: 0,
            y: 1,
        },
        Direction::Down => Point {
            pn: true,
            x: 0,
            y: 1,
        },
        Direction::Left => Point {
            pn: false,
            x: 1,
            y: 0,
        },
        Direction::Right => Point {
            pn: true,
            x: 1,
            y: 0,
        },
        Direction::N => Point {
            pn: true,
            x: 0,
            y: 0,
        },
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Point {
    pn: bool,
    x: usize,
    y: usize,
}

impl Point {
    fn able_direction(&self) -> Vec<Direction> {
        const X_MAX: &usize = &20;
        const Y_MAX: &usize = &12;
        let x = &self.x;
        let y = &self.y;
        match (x, y) {
            (0, 0) => vec![Direction::Right, Direction::Down],
            (0, Y_MAX) => vec![Direction::Left, Direction::Up],
            (X_MAX, Y_MAX) => vec![Direction::Left, Direction::Up],
            (X_MAX, 0) => vec![Direction::Left, Direction::Down],
            (_, 0) => vec![Direction::Left, Direction::Right, Direction::Down],
            (_, Y_MAX) => vec![Direction::Left, Direction::Right, Direction::Up],
            (0, _) => vec![Direction::Right, Direction::Up, Direction::Down],
            (X_MAX, _) => vec![Direction::Left, Direction::Up, Direction::Down],
            (_, _) => vec![
                Direction::Left,
                Direction::Right,
                Direction::Up,
                Direction::Down,
            ],
        }
    }
}

// 0の場合は事前に弾くように気をつける(isizeにしてusizeキャスト対応が面倒)
impl Add for Point {
    type Output = Self;
    fn add(self, rhs: Point) -> Point {
        if rhs.pn {
            Point {
                pn: true,
                x: self.x + rhs.x,
                y: self.y + rhs.y,
            }
        } else {
            Point {
                pn: true,
                x: self.x - rhs.x,
                y: self.y - rhs.y,
            }
        }
    }
}

fn manhattan(src: &Point, dst: &Point) -> usize {
    src.x.abs_diff(dst.x) + src.y.abs_diff(dst.y)
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Field {
    point: Point,
    state: State,
    came_from: Direction,
    f_cost: Option<usize>,
    g_cost: Option<usize>,
}

pub fn search_dungeon(mut dungeon: Vec<Vec<Field>>) -> Vec<Direction> {
    let start = dungeon
        .iter()
        .flatten()
        .filter(|&f| f.state == State::Start)
        .collect::<Vec<_>>()
        .remove(0)
        .clone()
        .point;
    let goal = dungeon
        .iter()
        .flatten()
        .filter(|&f| f.state == State::Goal)
        .collect::<Vec<_>>()
        .remove(0)
        .clone()
        .point;
    // search dungeon
    let mut openlist: Vec<Point> = vec![start];
    let mut src: Point;
    loop {
        if openlist.is_empty() {
            println!("unreachable goal");
            break;
        }

        // select
        // // sort by f_const asc
        openlist.sort_by(|a, b| dungeon[a.y][a.x].f_cost.cmp(&dungeon[b.y][b.x].f_cost));
        src = openlist.remove(0);

        if dungeon[src.y][src.x].state == State::Goal {
            println!("--- reached the goal!! ---");
            // generate path
            break;
        }

        // expand
        //
        let able_direction: Vec<Direction> = src.able_direction();

        clearscreen::clear().expect("failed to clear screen");
        display_dungeon(&dungeon);
        sleep(time::Duration::from_millis(100));

        let mut next_aisles: Vec<Point> = vec![];

        for direction in able_direction {
            let next = src.clone() + from_direction(direction);
            let h_cost = manhattan(&next, &goal);
            if dungeon[next.y][next.x].state != State::Wall
                && dungeon[next.y][next.x].f_cost.is_none()
            {
                let src_g_cost = dungeon[src.y][src.x].g_cost.unwrap();
                dungeon[next.y][next.x].g_cost = Some(src_g_cost + 1usize);
                dungeon[next.y][next.x].f_cost = Some(src_g_cost + 1usize + h_cost);
                next_aisles.push(next);
            }
        }

        // generate
        openlist = next_aisles.into_iter().chain(openlist).collect();
    }
    vec![] // todo
}

pub fn create_dungeon() -> Vec<Vec<Field>> {
    let mut dungeon = vec![];
    // read src/dungeon
    // create dungeon
    let mut file = fs::File::open("dungeon").expect("file not found");

    let mut contents = String::new();

    file.read_to_string(&mut contents)
        .expect("invalid file format");
    for (i, line) in contents.lines().enumerate() {
        let mut row = vec![];
        for (j, c) in line.chars().enumerate() {
            let state = match c {
                ' ' => State::Aisle,
                '*' => State::Wall,
                'S' => State::Start,
                'G' => State::Goal,
                _ => panic!("invalid character"),
            };
            row.push(Field {
                point: Point {
                    pn: true,
                    x: j,
                    y: i,
                },
                state,
                g_cost: Some(0),
                ..Default::default()
            });
        }
        dungeon.push(row);
    }
    dungeon
}

pub fn display_dungeon(dungeon: &Vec<Vec<Field>>) {
    for row in dungeon {
        for field in row {
            match field.state {
                State::Aisle => match field.g_cost {
                    None => print!("  "),
                    Some(cost) => print!("{:0>2}", cost),
                },
                State::Wall => print!("██"), // 壁は罫線ブロック
                State::Start => print!("St"),
                State::Goal => print!("Go"),
                _ => print!(""),
            }
        }
        println!(); // 行の終わりで改行
    }
}

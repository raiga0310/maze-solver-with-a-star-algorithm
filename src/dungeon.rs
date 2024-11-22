use std::{cmp::Reverse, collections::BinaryHeap, fs, io::Read, ops::Add, thread::sleep, time};

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

impl Field {
    fn x(&self) -> usize {
        self.point.x
    }

    fn y(&self) -> usize {
        self.point.y
    }
}

impl PartialOrd for Field {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.f_cost.unwrap_or(0).cmp(&other.f_cost.unwrap_or(0)))
    }
}

impl Ord for Field {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.f_cost.unwrap_or(0).cmp(&other.f_cost.unwrap_or(0))
    }
}

#[derive(Debug)]
pub struct Dungeon {
    pub dungeon: Vec<Vec<Field>>,
    pub width: usize,
    pub height: usize,
}

impl Dungeon {
    pub fn new() -> Self {
        let mut dungeon = vec![];
        // read src/dungeon
        // create dungeon
        let mut file = fs::File::open("dungeon").expect("file not found");
    
        let mut contents = String::new();
    
        file.read_to_string(&mut contents)
            .expect("invalid file format");
        let mut width: usize = 0;
        let mut height: usize = 0;
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
                width = if width < j { j } else { width };
            }
            dungeon.push(row);
            height = if height < i { i } else { height };
        }

        Self {
            dungeon,
            width, 
            height,
        }
    }

    fn next_point(&self, base: Point) -> Vec<Point> {
            let points = match (base.x, base.y) {
                (0, 0) => vec![Direction::Down, Direction::Right],
                (0, _) => vec![Direction::Up, Direction::Down, Direction::Right],
                (_, 0) => vec![Direction::Down, Direction::Left, Direction::Right],
                _ => vec![Direction::Up, Direction::Down, Direction::Left, Direction::Right],
            };
            points
                .into_iter()
                .map(|d| base.clone() + from_direction(d))
                .map(Point::from)
                .filter(|p| self.is_inside(p))
                .collect()
    }

    fn is_inside(&self, target: &Point) -> bool {
        target.x < self.width && target.y < self.height
    }
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

pub fn search_dungeon(mut dungeon: Dungeon) -> Vec<Direction> {
    let start = dungeon.dungeon
        .iter()
        .flatten()
        .filter(|&f| f.state == State::Start)
        .collect::<Vec<_>>()
        .remove(0)
        .clone();
    let goal = dungeon.dungeon
        .iter()
        .flatten()
        .filter(|&f| f.state == State::Goal)
        .collect::<Vec<_>>()
        .remove(0)
        .clone();
    // search dungeon
    // *min-heap*
    let mut openlist = BinaryHeap::from([Reverse(start)]);
    let mut src: Field;
    loop {
        if openlist.is_empty() {
            println!("unreachable goal");
            break;
        }

        // select
        src = {
            let reverse = openlist.pop().unwrap();
            reverse.0
        };

        if dungeon.dungeon[src.y()][src.x()].state == State::Goal {
            println!("--- reached the goal!! ---");
            // generate path
            break;
        }

        clearscreen::clear().expect("failed to clear screen");
        display_dungeon(&dungeon.dungeon);
        sleep(time::Duration::from_millis(100));


        let src_g_cost = dungeon.dungeon[src.y()][src.x()].g_cost.unwrap();
        for point in dungeon.next_point(src.point) {
            let h_cost = manhattan(&point, &goal.point);
            if dungeon.dungeon[point.y][point.x].state != State::Wall
                && dungeon.dungeon[point.y][point.x].f_cost.is_none()
            {
                dungeon.dungeon[point.y][point.x].g_cost = Some(src_g_cost + 1usize);
                dungeon.dungeon[point.y][point.x].f_cost = Some(src_g_cost + 1usize + h_cost);
                openlist.push(Reverse(dungeon.dungeon[point.y][point.x].clone()));
            }
        }
    }
    vec![] // todo
}
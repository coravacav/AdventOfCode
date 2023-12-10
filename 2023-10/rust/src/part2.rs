use rust_aoc_lib::part2;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum TileType {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
    Start,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Tile {
    pub tile_type: TileType,
    pub is_part_of_path: bool,
    pub coords: Coord,
}

impl Tile {
    fn new(tile_type: TileType, coords: Coord) -> Self {
        if tile_type == TileType::Start {
            Tile {
                tile_type,
                is_part_of_path: true,
                coords,
            }
        } else {
            Tile {
                tile_type,
                is_part_of_path: false,
                coords,
            }
        }
    }
}

impl TileType {
    fn is_horizontal(&self) -> bool {
        !matches!(
            self,
            TileType::Vertical | TileType::Ground | TileType::Start
        )
    }

    fn is_vertical(&self) -> bool {
        !matches!(
            self,
            TileType::Horizontal | TileType::Ground | TileType::Start
        )
    }

    pub fn get_next_direction(&self, incoming_direction: &Direction) -> Direction {
        match self {
            TileType::Vertical => match incoming_direction {
                Direction::N => Direction::N,
                Direction::S => Direction::S,
                e => panic!("Invalid direction {e:?} for {self:?}"),
            },
            TileType::Horizontal => match incoming_direction {
                Direction::E => Direction::E,
                Direction::W => Direction::W,
                e => panic!("Invalid direction {e:?} for {self:?}"),
            },
            TileType::NorthToWest => match incoming_direction {
                Direction::S => Direction::W,
                Direction::E => Direction::N,
                e => panic!("Invalid direction {e:?} for {self:?}"),
            },
            TileType::NorthToEast => match incoming_direction {
                Direction::S => Direction::E,
                Direction::W => Direction::N,
                e => panic!("Invalid direction {e:?} for {self:?}"),
            },
            TileType::SouthToWest => match incoming_direction {
                Direction::N => Direction::W,
                Direction::E => Direction::S,
                e => panic!("Invalid direction {e:?} for {self:?}"),
            },
            TileType::SouthToEast => match incoming_direction {
                Direction::N => Direction::E,
                Direction::W => Direction::S,
                e => panic!("Invalid direction {e:?} for {self:?}"),
            },
            t => panic!("Invalid tile {t:?}"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    N,
    E,
    S,
    W,
}

/// Letters match what direction to add or subtrack to move in that direction
#[derive(PartialEq, Eq, Debug, Clone, Copy)]
struct Coord {
    pub ns: usize,
    pub we: usize,
}

impl std::fmt::Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}, {}]", self.ns, self.we)
    }
}

impl Coord {
    pub fn take_direction(&self, direction: &Direction) -> Self {
        match direction {
            Direction::N => Coord {
                ns: self.ns.saturating_sub(1),
                we: self.we,
            },
            Direction::E => Coord {
                ns: self.ns,
                we: self.we + 1,
            },
            Direction::S => Coord {
                ns: self.ns + 1,
                we: self.we,
            },
            Direction::W => Coord {
                ns: self.ns,
                we: self.we.saturating_sub(1),
            },
        }
    }
}

impl std::ops::Index<Coord> for Vec<Vec<Tile>> {
    type Output = Tile;

    fn index(&self, index: Coord) -> &Self::Output {
        &self[index.ns][index.we]
    }
}

impl std::ops::IndexMut<Coord> for Vec<Vec<Tile>> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        &mut self[index.ns][index.we]
    }
}

#[part2]
pub fn part2(input: &str) -> usize {
    let mut pipes: Vec<Vec<Tile>> = Vec::new();

    for (ns, line) in input.lines().enumerate() {
        let mut pipe: Vec<Tile> = Vec::new();
        for (we, c) in line.chars().enumerate() {
            let coords = Coord { ns, we };
            match c {
                '|' => pipe.push(Tile::new(TileType::Vertical, coords)),
                '-' => pipe.push(Tile::new(TileType::Horizontal, coords)),
                'L' => pipe.push(Tile::new(TileType::NorthToEast, coords)),
                'J' => pipe.push(Tile::new(TileType::NorthToWest, coords)),
                '7' => pipe.push(Tile::new(TileType::SouthToWest, coords)),
                'F' => pipe.push(Tile::new(TileType::SouthToEast, coords)),
                '.' => pipe.push(Tile::new(TileType::Ground, coords)),
                'S' => pipe.push(Tile::new(TileType::Start, coords)),
                _ => panic!("Unknown character: {}", c),
            }
        }
        pipes.push(pipe);
    }

    // Find Start
    let mut start_coords = Coord { ns: 0, we: 0 };
    'outer: for (x, pipe) in pipes.iter().enumerate() {
        for (y, tile) in pipe.iter().enumerate() {
            if let TileType::Start = tile.tile_type {
                start_coords = Coord { ns: x, we: y };
                break 'outer;
            }
        }
    }

    let max_x = pipes.len();
    let max_y = pipes[0].len();

    #[derive(Debug)]
    struct AllDirections {
        pub north: TileType,
        pub east: TileType,
        pub south: TileType,
        pub west: TileType,
    }

    // pipes[start_coords].tile_type =
    let north = pipes[start_coords.take_direction(&Direction::N)].tile_type;
    let east = pipes[start_coords.take_direction(&Direction::E)].tile_type;
    let south = pipes[start_coords.take_direction(&Direction::S)].tile_type;
    let west = pipes[start_coords.take_direction(&Direction::W)].tile_type;

    // Figure out what tile Start actually is (it's guaranteed to be one)
    let starting_directions: [Direction; 2] = match (AllDirections {
        north,
        east,
        south,
        west,
    }) {
        AllDirections {
            north: TileType::Vertical,
            south: TileType::Vertical,
            ..
        } => [Direction::N, Direction::S],
        AllDirections {
            east: TileType::Horizontal,
            west: TileType::Horizontal,
            ..
        } => [Direction::E, Direction::W],
        AllDirections {
            north: TileType::NorthToEast,
            south: TileType::SouthToWest,
            ..
        } => [Direction::N, Direction::E],
        AllDirections {
            north: TileType::NorthToWest,
            south: TileType::SouthToEast,
            ..
        } => [Direction::N, Direction::W],
        AllDirections {
            north: TileType::SouthToWest,
            south: TileType::NorthToEast,
            ..
        } => [Direction::S, Direction::W],
        AllDirections {
            north: TileType::SouthToEast,
            south: TileType::NorthToWest,
            ..
        } => [Direction::S, Direction::E],
        AllDirections {
            east: TileType::NorthToWest,
            south: TileType::Vertical,
            ..
        } => [Direction::E, Direction::S],
        AllDirections {
            east: TileType::NorthToEast,
            south: TileType::Vertical,
            ..
        } => [Direction::E, Direction::N],
        AllDirections {
            east: TileType::SouthToWest,
            south: TileType::Vertical,
            ..
        } => [Direction::W, Direction::S],
        AllDirections {
            east: TileType::SouthToEast,
            south: TileType::Vertical,
            ..
        } => [Direction::W, Direction::N],
        AllDirections {
            south: TileType::Vertical,
            west: TileType::SouthToEast,
            ..
        } => [Direction::S, Direction::W],
        AllDirections {
            east: TileType::Horizontal,
            south: TileType::Vertical,
            ..
        } => [Direction::E, Direction::S],
        AllDirections {
            east: TileType::SouthToWest,
            south: TileType::NorthToWest,
            ..
        } => [Direction::E, Direction::S],

        e => panic!("Invalid start tile {e:?}"),
    };

    let mut search_coords = [
        (
            starting_directions[0],
            start_coords.take_direction(&starting_directions[0]),
        ),
        (
            starting_directions[1],
            start_coords.take_direction(&starting_directions[1]),
        ),
    ];

    // make sure the first coords are set is_part_of_path
    pipes[search_coords[0].1].is_part_of_path = true;
    pipes[search_coords[1].1].is_part_of_path = true;

    while search_coords[0].1 != search_coords[1].1 {
        for coords in search_coords.iter_mut() {
            let tile = &pipes[coords.1];
            let new_direction = tile.tile_type.get_next_direction(&coords.0);
            let new_coord = coords.1.take_direction(&new_direction);
            pipes[new_coord].is_part_of_path = true;

            *coords = (new_direction, new_coord);
        }
    }

    let search = |pipes: &Vec<Vec<Tile>>,
                  direction: Direction,
                  mut starting_coords: Coord,
                  cond: fn(TileType) -> bool|
     -> bool {
        let mut count = 0;

        loop {
            let new_coord = starting_coords.take_direction(&direction);

            if new_coord.ns >= max_x || new_coord.we >= max_y || new_coord == starting_coords {
                return count % 2 == 1;
            }

            let pipe = &pipes[new_coord];
            if pipe.is_part_of_path && cond(pipe.tile_type) {
                count += 1;
            }

            starting_coords = new_coord;
        }
    };

    pipes
        .iter()
        .flat_map(|pipe| pipe.iter())
        .filter(|tile| !tile.is_part_of_path)
        .map(|tile| tile.coords)
        .filter(|&coords| search(&pipes, Direction::N, coords, |ty| ty.is_horizontal()))
        .filter(|&coords| search(&pipes, Direction::S, coords, |ty| ty.is_horizontal()))
        .filter(|&coords| search(&pipes, Direction::E, coords, |ty| ty.is_vertical()))
        .filter(|&coords| search(&pipes, Direction::W, coords, |ty| ty.is_vertical()))
        .inspect(|tile| println!("{:?}", tile))
        .count()
}

// write to file out.txt the entire map but the PATH is marked with X
// let mut file = std::fs::File::create("out.txt").unwrap();
// for pipe in pipes.iter() {
//     use std::io::Write;
//     for tile in pipe.iter() {
//         if tile.is_part_of_path {
//             write!(file, "X").unwrap();
//         } else {
//             write!(
//                 file,
//                 "{}",
//                 match tile.tile_type {
//                     TileType::Vertical => '|',
//                     TileType::Horizontal => '-',
//                     TileType::NorthToEast => 'L',
//                     TileType::NorthToWest => 'J',
//                     TileType::SouthToWest => '7',
//                     TileType::SouthToEast => 'F',
//                     TileType::Ground => '.',
//                     TileType::Start => 'S',
//                 }
//             )
//             .unwrap();
//         }
//     }
//     writeln!(file,).unwrap();
// }

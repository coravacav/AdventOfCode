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
use Direction::*;
use TileType::*;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Tile {
    pub tile_type: TileType,
    pub is_part_of_path: bool,
    pub coords: Coord,
}

impl Tile {
    fn new(tile_type: TileType, coords: Coord) -> Self {
        if tile_type == Start {
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
        !matches!(self, Vertical | Ground | Start)
    }

    fn is_vertical(&self) -> bool {
        !matches!(self, Horizontal | Ground | Start)
    }

    pub fn get_next_direction(&self, traveling_direction: &Direction) -> Direction {
        macro_rules! handle {
            ($traveling_direction:ident, $($from:ident => $to:ident),+) => {
                match traveling_direction {
                    $(
                        $from => $to,
                    )+
                    e => panic!("Invalid direction {e:?} for {self:?}"),
                }
            };
        }

        match self {
            Vertical => handle!(traveling_direction, N => N, S => S),
            Horizontal => handle!(traveling_direction, E => E, W => W),
            NorthToWest => handle!(traveling_direction, S => W, E => N),
            NorthToEast => handle!(traveling_direction, S => E, W => N),
            SouthToWest => handle!(traveling_direction, N => W, E => S),
            SouthToEast => handle!(traveling_direction, N => E, W => S),
            _ => *traveling_direction,
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
    pub fn move_direction(&self, direction: &Direction) -> Self {
        match direction {
            N => Coord {
                ns: self.ns.saturating_sub(1),
                we: self.we,
            },
            E => Coord {
                ns: self.ns,
                we: self.we + 1,
            },
            S => Coord {
                ns: self.ns + 1,
                we: self.we,
            },
            W => Coord {
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
                '|' => pipe.push(Tile::new(Vertical, coords)),
                '-' => pipe.push(Tile::new(Horizontal, coords)),
                'L' => pipe.push(Tile::new(NorthToEast, coords)),
                'J' => pipe.push(Tile::new(NorthToWest, coords)),
                '7' => pipe.push(Tile::new(SouthToWest, coords)),
                'F' => pipe.push(Tile::new(SouthToEast, coords)),
                '.' => pipe.push(Tile::new(Ground, coords)),
                'S' => pipe.push(Tile::new(Start, coords)),
                _ => panic!("Unknown character: {}", c),
            }
        }
        pipes.push(pipe);
    }

    // Find Start
    let mut start_coords = Coord { ns: 0, we: 0 };
    'outer: for (x, pipe) in pipes.iter().enumerate() {
        for (y, tile) in pipe.iter().enumerate() {
            if let Start = tile.tile_type {
                start_coords = Coord { ns: x, we: y };
                break 'outer;
            }
        }
    }

    let max_x = pipes.len();
    let max_y = pipes[0].len();

    // pipes[start_coords].tile_type =
    let n = pipes[start_coords.move_direction(&N)].tile_type;
    let e = pipes[start_coords.move_direction(&E)].tile_type;
    let s = pipes[start_coords.move_direction(&S)].tile_type;
    let w = pipes[start_coords.move_direction(&W)].tile_type;

    let mut starting_directions = vec![];

    if let Vertical | SouthToEast | SouthToWest = n {
        starting_directions.push(N)
    }
    if let Horizontal | NorthToWest | SouthToWest = e {
        starting_directions.push(E)
    }
    if let Vertical | NorthToEast | NorthToWest = s {
        starting_directions.push(S)
    }
    if let Horizontal | NorthToEast | SouthToEast = w {
        starting_directions.push(W)
    }

    assert_eq!(starting_directions.len(), 2);
    let starting_directions = [starting_directions[0], starting_directions[1]];

    pipes[start_coords].tile_type = match starting_directions {
        [N, E] => NorthToEast,
        [N, W] => NorthToWest,
        [S, W] => SouthToWest,
        [S, E] => SouthToEast,
        [N, S] => Vertical,
        [E, W] => Horizontal,
        _ => panic!("Invalid starting directions: {:?}", starting_directions),
    };

    let mut search_coords = [
        (
            starting_directions[0],
            start_coords.move_direction(&starting_directions[0]),
        ),
        (
            starting_directions[1],
            start_coords.move_direction(&starting_directions[1]),
        ),
    ];

    // make sure the first coords are set is_part_of_path
    pipes[search_coords[0].1].is_part_of_path = true;
    pipes[search_coords[1].1].is_part_of_path = true;

    while search_coords[0].1 != search_coords[1].1 {
        for coords in search_coords.iter_mut() {
            let tile = &pipes[coords.1];
            let new_direction = tile.tile_type.get_next_direction(&coords.0);
            let new_coord = coords.1.move_direction(&new_direction);
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
            let new_coord = starting_coords.move_direction(&direction);

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
        .filter(|&coords| search(&pipes, N, coords, |ty| ty.is_horizontal()))
        .filter(|&coords| search(&pipes, S, coords, |ty| ty.is_horizontal()))
        .filter(|&coords| search(&pipes, E, coords, |ty| ty.is_vertical()))
        .filter(|&coords| search(&pipes, W, coords, |ty| ty.is_vertical()))
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
//                     Vertical => '|',
//                     Horizontal => '-',
//                     NorthToEast => 'L',
//                     NorthToWest => 'J',
//                     SouthToWest => '7',
//                     SouthToEast => 'F',
//                     Ground => '.',
//                     Start => 'S',
//                 }
//             )
//             .unwrap();
//         }
//     }
//     writeln!(file,).unwrap();
// }

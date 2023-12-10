use rust_aoc_lib::part1;

#[derive(Debug)]
enum Tile {
    Vertical,
    Horizontal,
    NorthToEast,
    NorthToWest,
    SouthToWest,
    SouthToEast,
    Ground,
    Start,
}

impl Tile {
    pub fn get_directions(&self, incoming_direction: &Direction) -> Direction {
        match self {
            Tile::Vertical => match incoming_direction {
                Direction::North => Direction::North,
                Direction::South => Direction::South,
                e => panic!("Invalid direction {e:?} for {self:?}"),
            },
            Tile::Horizontal => match incoming_direction {
                Direction::East => Direction::East,
                Direction::West => Direction::West,
                e => panic!("Invalid direction {e:?} for {self:?}"),
            },
            Tile::NorthToWest => match incoming_direction {
                Direction::South => Direction::West,
                Direction::East => Direction::North,
                e => panic!("Invalid direction {e:?} for {self:?}"),
            },
            Tile::NorthToEast => match incoming_direction {
                Direction::South => Direction::East,
                Direction::West => Direction::North,
                e => panic!("Invalid direction {e:?} for {self:?}"),
            },
            Tile::SouthToWest => match incoming_direction {
                Direction::North => Direction::West,
                Direction::East => Direction::South,
                e => panic!("Invalid direction {e:?} for {self:?}"),
            },
            Tile::SouthToEast => match incoming_direction {
                Direction::North => Direction::East,
                Direction::West => Direction::South,
                e => panic!("Invalid direction {e:?} for {self:?}"),
            },
            _ => panic!("Invalid tile"),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North,
    East,
    South,
    West,
}

/// Letters match what direction to add or subtrack to move in that direction
#[derive(PartialEq, Eq, Debug)]
struct Coord {
    pub ns: usize,
    pub we: usize,
}

impl Coord {
    pub fn take_step(&self, direction: &Direction) -> Self {
        match direction {
            Direction::North => Coord {
                ns: self.ns.saturating_sub(1),
                we: self.we,
            },
            Direction::East => Coord {
                ns: self.ns,
                we: self.we + 1,
            },
            Direction::South => Coord {
                ns: self.ns + 1,
                we: self.we,
            },
            Direction::West => Coord {
                ns: self.ns,
                we: self.we.saturating_sub(1),
            },
        }
    }
}

#[part1]
pub fn part1(input: &str) -> usize {
    let mut pipes: Vec<Vec<Tile>> = Vec::new();

    for line in input.lines() {
        let mut pipe: Vec<Tile> = Vec::new();
        for c in line.chars() {
            match c {
                '|' => pipe.push(Tile::Vertical),
                '-' => pipe.push(Tile::Horizontal),
                'L' => pipe.push(Tile::NorthToEast),
                'J' => pipe.push(Tile::NorthToWest),
                '7' => pipe.push(Tile::SouthToWest),
                'F' => pipe.push(Tile::SouthToEast),
                '.' => pipe.push(Tile::Ground),
                'S' => pipe.push(Tile::Start),
                _ => panic!("Unknown character: {}", c),
            }
        }
        pipes.push(pipe);
    }

    // Find Start
    let mut start_coords = Coord { ns: 0, we: 0 };
    'outer: for (x, pipe) in pipes.iter().enumerate() {
        for (y, tile) in pipe.iter().enumerate() {
            if let Tile::Start = tile {
                start_coords = Coord { ns: x, we: y };
                break 'outer;
            }
        }
    }

    fn read_from_vec_with_coord<'a>(vec: &'a [Vec<Tile>], coord: &Coord) -> &'a Tile {
        &vec[coord.ns][coord.we]
    }

    #[derive(Debug)]
    struct AllDirections<'a> {
        pub north: &'a Tile,
        pub east: &'a Tile,
        pub south: &'a Tile,
        pub west: &'a Tile,
    }

    // Figure out what tile Start actually is (it's guaranteed to be one)
    let starting_directions: [Direction; 2] = match (AllDirections {
        north: read_from_vec_with_coord(&pipes, &start_coords.take_step(&Direction::North)),
        east: read_from_vec_with_coord(&pipes, &start_coords.take_step(&Direction::East)),
        south: read_from_vec_with_coord(&pipes, &start_coords.take_step(&Direction::South)),
        west: read_from_vec_with_coord(&pipes, &start_coords.take_step(&Direction::West)),
    }) {
        AllDirections {
            north: Tile::Vertical,
            east: _,
            south: Tile::Vertical,
            west: _,
        } => [Direction::North, Direction::South],
        AllDirections {
            north: _,
            east: Tile::Horizontal,
            south: _,
            west: Tile::Horizontal,
        } => [Direction::East, Direction::West],
        AllDirections {
            north: Tile::NorthToEast,
            east: _,
            south: Tile::SouthToWest,
            west: _,
        } => [Direction::North, Direction::East],
        AllDirections {
            north: Tile::NorthToWest,
            east: _,
            south: Tile::SouthToEast,
            west: _,
        } => [Direction::North, Direction::West],
        AllDirections {
            north: Tile::SouthToWest,
            east: _,
            south: Tile::NorthToEast,
            west: _,
        } => [Direction::South, Direction::West],
        AllDirections {
            north: Tile::SouthToEast,
            east: _,
            south: Tile::NorthToWest,
            west: _,
        } => [Direction::South, Direction::East],
        AllDirections {
            north: _,
            east: Tile::NorthToWest,
            south: Tile::Vertical,
            west: _,
        } => [Direction::East, Direction::South],
        AllDirections {
            north: _,
            east: Tile::NorthToEast,
            south: Tile::Vertical,
            west: _,
        } => [Direction::East, Direction::North],
        AllDirections {
            north: _,
            east: Tile::SouthToWest,
            south: Tile::Vertical,
            west: _,
        } => [Direction::West, Direction::South],
        AllDirections {
            north: _,
            east: Tile::SouthToEast,
            south: Tile::Vertical,
            west: _,
        } => [Direction::West, Direction::North],
        AllDirections {
            north: _,
            east: _,
            south: Tile::Vertical,
            west: Tile::SouthToEast,
        } => [Direction::South, Direction::West],
        e => panic!("Invalid start tile {e:?}"),
    };

    let mut search_coords = [
        (
            starting_directions[0],
            start_coords.take_step(&starting_directions[0]),
        ),
        (
            starting_directions[1],
            start_coords.take_step(&starting_directions[1]),
        ),
    ];

    let mut steps = 1;

    // Start searching
    while search_coords[0].1 != search_coords[1].1 {
        steps += 1;
        for coords in search_coords.iter_mut() {
            let tile = read_from_vec_with_coord(&pipes, &coords.1);
            let new_direction = tile.get_directions(&coords.0);
            let new_coord = coords.1.take_step(&new_direction);
            *coords = (new_direction, new_coord);
        }
    }

    steps
}

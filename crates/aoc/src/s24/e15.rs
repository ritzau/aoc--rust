use crate::cache::AocCache;
use crate::input::{Input, InputFetcher};
use crate::s24::YEAR;
use crate::{head, Day, PuzzleResult};

const DAY: Day = Day(15);

pub fn solve(aoc: &AocCache) -> PuzzleResult<()> {
    head(YEAR, DAY, "Warehouse Woes");
    let input = aoc.get_input(YEAR, DAY)?;

    let p1 = part1(&input)?;
    println!("Part 1: {}", p1);
    assert_eq!(p1, 1457740);

    let p2 = part2(&input)?;
    println!("Part 2: {}", p2);
    assert_eq!(p2, 1467145);

    Ok(())
}

fn part1(input: &Input) -> PuzzleResult<usize> {
    let lines = input.lines()?;
    let mut scene = part_1::Scene::<128>::parse(lines)?;
    while scene.step() {
        // Just step
    }

    Ok(scene.coordinate_sum())
}

fn part2(input: &Input) -> PuzzleResult<usize> {
    let lines = input.lines()?;
    let mut scene = part_2::Scene::<256>::parse(lines)?;
    while scene.step() {
        // Just step
    }

    Ok(scene.coordinate_sum())
}

mod part_1 {
    use crate::input::Lines;
    use crate::PuzzleResult;
    use std::collections::VecDeque;

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Tile {
        OutOfBounds,
        Empty,
        Box,
        Wall,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Robot {
        x: usize,
        y: usize,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub(crate) struct Scene<const N: usize> {
        grid: [[Tile; N]; N],
        width: usize,
        height: usize,
        robot: Robot,
        moves: VecDeque<Direction>,
    }

    impl<const N: usize> Scene<N> {
        pub(crate) fn parse(mut lines: Lines) -> PuzzleResult<Self> {
            let mut grid = [[Tile::OutOfBounds; N]; N];
            let mut robot = None;
            let mut width = 0usize;
            let mut height = 0usize;

            for (y, line) in lines.by_ref().take_while(|l| !l.is_empty()).enumerate() {
                height = y + 1;
                let mut count = 0usize;
                for (x, c) in line.chars().enumerate() {
                    let tile = match c {
                        '.' => Tile::Empty,
                        '#' => Tile::Wall,
                        'O' => Tile::Box,
                        '@' => {
                            robot = Some(Robot { x, y });
                            Tile::Empty
                        }
                        _ => panic!("unexpected character: {}", c),
                    };
                    grid[y][x] = tile;
                    count = x + 1;
                }
                width = width.max(count);
            }

            let mut moves = VecDeque::new();
            for line in lines {
                for d in line.chars() {
                    let d = match d {
                        '^' => Direction::North,
                        'v' => Direction::South,
                        '<' => Direction::West,
                        '>' => Direction::East,
                        _ => panic!("unexpected character: {}", d),
                    };
                    moves.push_back(d);
                }
            }

            let robot = robot.expect("robot not found");
            Ok(Self {
                grid,
                width,
                height,
                robot,
                moves,
            })
        }

        pub(crate) fn step(&mut self) -> bool {
            match self.moves.pop_front() {
                Some(direction) => {
                    assert!(0 < self.robot.x && self.robot.x < self.width - 1);
                    assert!(0 < self.robot.y && self.robot.y < self.height - 1);

                    let (next_x, next_y) = match direction {
                        Direction::North => (self.robot.x, self.robot.y - 1),
                        Direction::South => (self.robot.x, self.robot.y + 1),
                        Direction::West => (self.robot.x - 1, self.robot.y),
                        Direction::East => (self.robot.x + 1, self.robot.y),
                    };

                    let tile_free = match self.grid[next_y][next_x] {
                        Tile::Empty => true,
                        Tile::Wall => false,
                        Tile::Box => self.try_push_boxes(direction, next_x, next_y),
                        Tile::OutOfBounds => panic!("out of bounds"),
                    };

                    if tile_free {
                        self.robot.x = next_x;
                        self.robot.y = next_y;
                    }

                    true
                }
                None => false,
            }
        }

        fn try_push_boxes(&mut self, d: Direction, x: usize, y: usize) -> bool {
            let (mut bx, mut by) = (x, y);
            loop {
                match d {
                    Direction::North => by -= 1,
                    Direction::South => by += 1,
                    Direction::West => bx -= 1,
                    Direction::East => bx += 1,
                };

                match self.grid[by][bx] {
                    Tile::Box => {}
                    Tile::Empty => {
                        self.grid[y][x] = Tile::Empty;
                        self.grid[by][bx] = Tile::Box;
                        return true;
                    }
                    Tile::Wall => {
                        return false;
                    }
                    Tile::OutOfBounds => panic!("out of bounds"),
                }
            }
        }

        pub(crate) fn coordinate_sum(&self) -> usize {
            let mut sum = 0;
            for y in 0..self.height {
                for x in 0..self.width {
                    if let Tile::Box = self.grid[y][x] {
                        sum += x + 100 * y;
                    }
                }
            }
            sum
        }

        #[allow(dead_code)]
        fn print_grid(&self) {
            for y in 0..self.height {
                for x in 0..self.width {
                    if (x, y) == (self.robot.x, self.robot.y) {
                        print!("@");
                        continue;
                    }
                    let c = match self.grid[y][x] {
                        Tile::Empty => '.',
                        Tile::Wall => '#',
                        Tile::Box => 'O',
                        Tile::OutOfBounds => panic!("out of bounds"),
                    };
                    print!("{}", c);
                }
                println!();
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::input::Input;

        #[test]
        fn test_scene_parse() {
            let input: Input = super::super::tests::SAMPLE_1.into();
            let lines = input.lines().unwrap();
            let scene = Scene::<10>::parse(lines).unwrap();
            println!("{:#?}", scene);
            scene.print_grid();
        }

        #[test]
        fn test_scene_step() {
            let input: Input = super::super::tests::SAMPLE_1.into();
            let lines = input.lines().unwrap();
            let mut scene = Scene::<10>::parse(lines).unwrap();

            println!("Start");
            scene.print_grid();

            let mut step = 0;
            while scene.moves.len() > 0 {
                println!("\n Step {step}: {:?}", scene.moves.front().unwrap());
                scene.step();
                scene.print_grid();
                step += 1;
            }
        }
    }
}

mod part_2 {
    use crate::input::Lines;
    use crate::PuzzleResult;
    use std::collections::VecDeque;

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Tile {
        OutOfBounds,
        Empty,
        BoxLeft,
        BoxRight,
        Wall,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    enum Direction {
        North,
        East,
        South,
        West,
    }

    #[derive(Debug, Clone, Copy)]
    enum TileSide {
        Left,
        Right,
    }

    #[derive(Debug, Clone, PartialEq)]
    struct Robot {
        x: usize,
        y: usize,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub(crate) struct Scene<const N: usize> {
        grid: [[Tile; N]; N],
        width: usize,
        height: usize,
        robot: Robot,
        moves: VecDeque<Direction>,
    }

    impl<const N: usize> Scene<N> {
        pub(crate) fn parse(mut lines: Lines) -> PuzzleResult<Self> {
            let mut grid = [[Tile::OutOfBounds; N]; N];
            let mut robot = None;
            let mut width = 0usize;
            let mut height = 0usize;

            for (y, line) in lines.by_ref().take_while(|l| !l.is_empty()).enumerate() {
                height = y + 1;
                let mut count = 0usize;
                for (x, c) in line.chars().enumerate() {
                    let tiles = match c {
                        '.' => [Tile::Empty; 2],
                        '#' => [Tile::Wall; 2],
                        'O' => [Tile::BoxLeft, Tile::BoxRight],
                        '@' => {
                            robot = Some(Robot { x: 2 * x, y });
                            [Tile::Empty; 2]
                        }
                        _ => panic!("unexpected character: {}", c),
                    };
                    grid[y][2 * x] = tiles[0];
                    grid[y][2 * x + 1] = tiles[1];
                    count = 2 * x + 2;
                }
                width = width.max(count);
            }

            let mut moves = VecDeque::new();
            for line in lines {
                for d in line.chars() {
                    let d = match d {
                        '^' => Direction::North,
                        'v' => Direction::South,
                        '<' => Direction::West,
                        '>' => Direction::East,
                        _ => panic!("unexpected character: {}", d),
                    };
                    moves.push_back(d);
                }
            }

            let robot = robot.expect("robot not found");
            Ok(Self {
                grid,
                width,
                height,
                robot,
                moves,
            })
        }

        fn can_move(&self, x: usize, y: usize, direction: Direction) -> bool {
            match self.grid[y][x] {
                Tile::Empty => true,
                Tile::Wall => false,
                Tile::OutOfBounds => panic!("out of bounds"),
                Tile::BoxLeft => {
                    let tiles = Self::affected_tiles(x, y, direction, TileSide::Left);
                    tiles
                        .into_iter()
                        .all(|(_, (dx, dy))| self.can_move(dx, dy, direction))
                }
                Tile::BoxRight => {
                    let tiles = Self::affected_tiles(x, y, direction, TileSide::Right);
                    tiles
                        .into_iter()
                        .all(|(_, (dx, dy))| self.can_move(dx, dy, direction))
                }
            }
        }

        fn move_tile(&mut self, x: usize, y: usize, direction: Direction) {
            let tile = self.grid[y][x];
            match tile {
                Tile::Empty => {}
                Tile::Wall => panic!("cannot move wall"),
                Tile::OutOfBounds => panic!("out of bounds"),
                Tile::BoxLeft => {
                    let tiles = Self::affected_tiles(x, y, direction, TileSide::Left);
                    self.move_affected_tiles(direction, tiles);
                }
                Tile::BoxRight => {
                    let tiles = Self::affected_tiles(x, y, direction, TileSide::Right);
                    self.move_affected_tiles(direction, tiles);
                }
            }
        }

        fn affected_tiles(
            x: usize,
            y: usize,
            direction: Direction,
            side: TileSide,
        ) -> Vec<((usize, usize), (usize, usize))> {
            match direction {
                Direction::East => vec![((x, y), (x + 1, y))],
                Direction::West => vec![((x, y), (x - 1, y))],
                Direction::North => match side {
                    TileSide::Left => vec![((x, y), (x, y - 1)), ((x + 1, y), (x + 1, y - 1))],
                    TileSide::Right => vec![((x - 1, y), (x - 1, y - 1)), ((x, y), (x, y - 1))],
                },
                Direction::South => match side {
                    TileSide::Left => vec![((x, y), (x, y + 1)), ((x + 1, y), (x + 1, y + 1))],
                    TileSide::Right => vec![((x - 1, y), (x - 1, y + 1)), ((x, y), (x, y + 1))],
                },
            }
        }

        fn move_affected_tiles(
            &mut self,
            direction: Direction,
            tiles: Vec<((usize, usize), (usize, usize))>,
        ) {
            for ((sx, sy), (dx, dy)) in tiles {
                self.move_tile(dx, dy, direction);
                self.grid[dy][dx] = self.grid[sy][sx];
                self.grid[sy][sx] = Tile::Empty;
            }
        }

        pub(crate) fn step(&mut self) -> bool {
            match self.moves.pop_front() {
                Some(direction) => {
                    let (x, y) = match direction {
                        Direction::North => (self.robot.x, self.robot.y - 1),
                        Direction::South => (self.robot.x, self.robot.y + 1),
                        Direction::West => (self.robot.x - 1, self.robot.y),
                        Direction::East => (self.robot.x + 1, self.robot.y),
                    };
                    match self.grid[y][x] {
                        Tile::Empty => {
                            self.robot.x = x;
                            self.robot.y = y;
                        }
                        Tile::Wall => {}
                        Tile::BoxLeft | Tile::BoxRight => {
                            if self.can_move(x, y, direction) {
                                self.move_tile(x, y, direction);
                                self.robot.x = x;
                                self.robot.y = y;
                            }
                        }
                        Tile::OutOfBounds => panic!("out of bounds"),
                    }
                    true
                }
                None => false,
            }
        }

        pub(crate) fn coordinate_sum(&self) -> usize {
            let mut sum = 0;
            for y in 0..self.height {
                for x in 0..self.width {
                    if let Tile::BoxLeft = self.grid[y][x] {
                        sum += x + 100 * y;
                    }
                }
            }
            sum
        }

        #[allow(dead_code)]
        fn print_grid(&self) {
            for y in 0..self.height {
                for x in 0..self.width {
                    if (x, y) == (self.robot.x, self.robot.y) {
                        print!("@");
                        continue;
                    }
                    let c = match self.grid[y][x] {
                        Tile::Empty => '.',
                        Tile::Wall => '#',
                        Tile::BoxLeft => '[',
                        Tile::BoxRight => ']',
                        Tile::OutOfBounds => panic!("out of bounds"),
                    };
                    print!("{}", c);
                }
                println!();
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::input::Input;

        #[test]
        fn test_scene_parse() {
            let input: Input = super::super::tests::SAMPLE.into();
            let lines = input.lines().unwrap();
            let scene = Scene::<20>::parse(lines).unwrap();
            println!("{:#?}", scene);
            scene.print_grid();
        }

        #[test]
        fn test_scene_step() {
            let input: Input = super::super::tests::SAMPLE.into();
            let lines = input.lines().unwrap();
            let mut scene = Scene::<20>::parse(lines).unwrap();

            println!("Start");
            scene.print_grid();

            let mut step = 0;
            while scene.moves.len() > 0 {
                println!("\n Step {step}: {:?}", scene.moves.front().unwrap());
                scene.step();
                scene.print_grid();
                step += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub(crate) const SAMPLE_1: &str = "\
########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<
";

    pub(crate) const SAMPLE: &str = "\
##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^
";

    #[test]
    fn test_part1() {
        assert_eq!(part1(&SAMPLE_1.into()).unwrap(), 2028);
        assert_eq!(part1(&SAMPLE.into()).unwrap(), 10092);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&SAMPLE.into()).unwrap(), 9021);
    }
}

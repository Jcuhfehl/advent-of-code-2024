mod vector2;
use vector2::Grid;
use vector2::Vector2i;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WarehouseSpace {
    Empty,
    Box,
    Wall,
}

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let (mut map, mut position, mut instructions) = parse_input(input);
    advance(&mut map, &mut position, &mut instructions);
    let coordinate_sum = gps_coordinates(&map);
    println!("Coordinate sum: {}", coordinate_sum);
}

fn gps_coordinates(map: &Grid<WarehouseSpace>) -> i64 {
    map.coordinates()
        .into_iter()
        .map(|coord| {
            if *map.get(&coord) == WarehouseSpace::Box {
                100 * coord.y + coord.x
            } else {
                0
            }
        })
        .sum()
}

fn advance(
    map: &mut Grid<WarehouseSpace>,
    position: &mut Vector2i,
    instructions: &mut Vec<Vector2i>,
) {
    while !instructions.is_empty() {
        let instruction = instructions.remove(0);
        let start_position = *position + instruction;
        let mut scanning_position = *position;
        println!("{}", debug_grid(map, position));
        let mut moves_box = false;
        loop {
            scanning_position = scanning_position + instruction;
            match map.get(&scanning_position) {
                WarehouseSpace::Empty => {
                    if moves_box {
                        map.set(&start_position, WarehouseSpace::Empty);
                        map.set(&scanning_position, WarehouseSpace::Box);
                    }
                    *position = start_position;
                    break;
                }
                WarehouseSpace::Box => moves_box = true,
                WarehouseSpace::Wall => break,
            }
        }
    }
}

fn debug_grid(map: &Grid<WarehouseSpace>, position: &Vector2i) -> String {
    let mut chars: Vec<Vec<char>> = (0..map.size)
        .map(|_| (0..map.size).map(|_| ' ').collect())
        .collect();
    for coord in map.coordinates() {
        chars[coord.y as usize][coord.x as usize] = match map.get(&coord) {
            WarehouseSpace::Empty => '.',
            WarehouseSpace::Box => 'O',
            WarehouseSpace::Wall => '#',
        }
    }

    if chars[position.y as usize][position.x as usize] != 'O' {
        chars[position.y as usize][position.x as usize] = '@';
    } else {
        chars[position.y as usize][position.x as usize] = 'Q';
    }

    chars
        .into_iter()
        .map(|line| line.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_input(input: &str) -> (Grid<WarehouseSpace>, Vector2i, Vec<Vector2i>) {
    let (map, moves_str) = input.split_once("\n\n").unwrap();
    let size = map.lines().count();
    let mut robot_position = Vector2i::new(-1, -1);
    let mut grid = Grid::empty(size, WarehouseSpace::Wall);
    map.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '#' => grid.values[x][y] = WarehouseSpace::Wall,
            '.' => grid.values[x][y] = WarehouseSpace::Empty,
            'O' => grid.values[x][y] = WarehouseSpace::Box,
            '@' => {
                grid.values[x][y] = WarehouseSpace::Empty;
                robot_position = Vector2i::new(x as i64, y as i64);
            }
            _ => unreachable!(),
        })
    });

    let moves = moves_str
        .lines()
        .flat_map(|line| line.chars())
        .map(|c| match c {
            '^' => Vector2i::new(0, -1),
            'v' => Vector2i::new(0, 1),
            '>' => Vector2i::new(1, 0),
            '<' => Vector2i::new(-1, 0),
            _ => unreachable!(),
        })
        .collect();
    (grid, robot_position, moves)
}

const TEST_INPUT: &str = "##########
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
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

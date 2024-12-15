mod vector2;
use std::collections::HashSet;

use std::{thread, time};
use vector2::Grid;
use vector2::Vector2i;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WarehouseSpace {
    Empty,
    LeftBox,
    RightBox,
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
            if *map.get(&coord) == WarehouseSpace::LeftBox {
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
    println!("Start position");
    println!("{}", debug_grid(map, position));
    println!();

    while !instructions.is_empty() {
        thread::sleep(time::Duration::from_millis(1));
        let instruction = instructions.remove(0);
        let target_position = *position + instruction;
        println!("Instruction: {:?}", instruction);

        let mut to_move: HashSet<Vector2i> = HashSet::new();
        to_move.insert(target_position);

        let mut can_move = true;
        let mut to_check: Vec<Vector2i> = vec![target_position];

        while let Some(check_now) = to_check.pop() {
            let dependent_positions = match map.get(&check_now) {
                WarehouseSpace::LeftBox => {
                    if instruction.y != 0 {
                        vec![check_now + instruction, check_now + Vector2i::new(1, 0)]
                    } else {
                        vec![check_now + instruction]
                    }
                }
                WarehouseSpace::RightBox => {
                    if instruction.y != 0 {
                        vec![check_now + instruction, check_now + Vector2i::new(-1, 0)]
                    } else {
                        vec![check_now + instruction]
                    }
                }
                WarehouseSpace::Empty => {
                    to_move.clear();
                    break;
                }
                WarehouseSpace::Wall => {
                    can_move = false;
                    break;
                }
            };

            for dep_pos in dependent_positions {
                match map.get(&dep_pos) {
                    WarehouseSpace::Empty => continue,
                    WarehouseSpace::LeftBox => {
                        if !to_move.contains(&dep_pos) {
                            to_check.push(dep_pos);
                            to_move.insert(dep_pos);
                        }
                    }
                    WarehouseSpace::RightBox => {
                        if !to_move.contains(&dep_pos) {
                            to_check.push(dep_pos);
                            to_move.insert(dep_pos);
                        }
                    }
                    WarehouseSpace::Wall => {
                        can_move = false;
                    }
                }
            }
            if !can_move {
                break;
            }
        }

        if can_move {
            let to_move_including_originals = to_move
                .iter()
                .map(|coord| (*coord, *map.get(coord)))
                .collect::<Vec<_>>();
            to_move_including_originals.iter().for_each(|(coord, _)| {
                map.set(coord, WarehouseSpace::Empty);
            });
            to_move_including_originals
                .into_iter()
                .for_each(|(coord, original)| {
                    map.set(&(coord + instruction), original);
                });
            map.set(&target_position, WarehouseSpace::Empty);
            *position = *position + instruction;
        }

        println!("{}", debug_grid(map, position));
        println!();
    }
}

fn debug_grid(map: &Grid<WarehouseSpace>, position: &Vector2i) -> String {
    let mut chars: Vec<Vec<char>> = (0..map.size.y)
        .map(|_| (0..map.size.x).map(|_| ' ').collect())
        .collect();
    for coord in map.coordinates() {
        chars[coord.y as usize][coord.x as usize] = match map.get(&coord) {
            WarehouseSpace::Empty => '.',
            WarehouseSpace::LeftBox => '[',
            WarehouseSpace::RightBox => ']',
            WarehouseSpace::Wall => '#',
        }
    }

    if chars[position.y as usize][position.x as usize] != '['
        && chars[position.y as usize][position.x as usize] != ']'
    {
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
    let size = Vector2i::new(
        2 * map.lines().next().unwrap().len() as i64,
        map.lines().count() as i64,
    );
    let mut robot_position = Vector2i::new(-1, -1);
    let mut grid = Grid::empty(size, WarehouseSpace::Wall);
    map.lines().enumerate().for_each(|(y, line)| {
        line.chars().enumerate().for_each(|(x, c)| match c {
            '#' => {
                grid.values[2 * x][y] = WarehouseSpace::Wall;
                grid.values[2 * x + 1][y] = WarehouseSpace::Wall;
            }
            '.' => {
                grid.values[2 * x][y] = WarehouseSpace::Empty;
                grid.values[2 * x + 1][y] = WarehouseSpace::Empty;
            }
            'O' => {
                grid.values[2 * x][y] = WarehouseSpace::LeftBox;
                grid.values[2 * x + 1][y] = WarehouseSpace::RightBox;
            }
            '@' => {
                grid.values[2 * x][y] = WarehouseSpace::Empty;
                grid.values[2 * x + 1][y] = WarehouseSpace::Empty;
                robot_position = Vector2i::new(2 * x as i64, y as i64);
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

const TEST_INPUT: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

const TEST_INPUT2: &str = "##########
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

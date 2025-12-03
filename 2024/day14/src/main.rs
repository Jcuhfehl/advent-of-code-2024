mod vector2;
use vector2::Vector2i;

#[derive(Debug, Clone)]
struct Robot {
    position: Vector2i,
    velocity: Vector2i,
}

fn main() {
    let input_with_trailing =
        std::fs::read_to_string("input.txt").expect("Should have been able to read the file");
    let input = input_with_trailing.trim_end();

    let mut robots = parse_input(input);
    let map_size = Vector2i::new(101, 103);

    let safety_score = safety_score(robots.clone(), 100, map_size);
    println!("Safety score: {safety_score}");

    let mut time = 0;
    loop {
        time += 1;
        robots
            .iter_mut()
            .for_each(|robot| advance_time(robot, 1, map_size));
        render(&robots, map_size, time);
    }
}

fn safety_score(mut robots: Vec<Robot>, time: usize, map_size: Vector2i) -> usize {
    robots
        .iter_mut()
        .for_each(|robot| advance_time(robot, time, map_size));
    let mut quadrants_count = [0; 4];
    robots.iter().for_each(|robot| {
        let left = robot.position.x < (map_size.x / 2);
        let right = robot.position.x > (map_size.x / 2);
        let top = robot.position.y < (map_size.y / 2);
        let bottom = robot.position.y > (map_size.y / 2);
        if left {
            if top {
                quadrants_count[0] += 1;
            } else if bottom {
                quadrants_count[1] += 1;
            }
        } else if right {
            if top {
                quadrants_count[2] += 1;
            } else if bottom {
                quadrants_count[3] += 1;
            }
        }
    });
    println!("{:?}", quadrants_count);

    quadrants_count.iter().product()
}

fn advance_time(robot: &mut Robot, time: usize, map_size: Vector2i) {
    let new_position = robot.position + robot.velocity * time as i64;
    robot.position = new_position.modulo(map_size);
}

fn render(robots: &[Robot], map_size: Vector2i, time: usize) {
    let mut chars: Vec<Vec<char>> = (0..map_size.y)
        .map(|_| (0..map_size.x).map(|_| ' ').collect())
        .collect();
    for robot in robots {
        chars[robot.position.y as usize][robot.position.x as usize] = 'X';
    }

    let text = chars
        .into_iter()
        .map(|line| line.into_iter().collect::<String>())
        .collect::<Vec<_>>()
        .join("\n");
    if text.contains("XXXXXXXX") {
        let mut buffer = "".to_string();
        println!("{}", text);
        println!("{}", time);
        println!("====================================================");
        let _ = std::io::stdin().read_line(&mut buffer);
    }
}

fn parse_input(input: &str) -> Vec<Robot> {
    input
        .lines()
        .map(|line| {
            let (p_part, v_part) = line.split_once(" ").unwrap();
            let (_, p_nums) = p_part.split_once("=").unwrap();
            let (_, v_nums) = v_part.split_once("=").unwrap();

            let (p_x, p_y) = p_nums.split_once(",").unwrap();
            let (v_x, v_y) = v_nums.split_once(",").unwrap();

            let position = Vector2i::new(p_x.parse().unwrap(), p_y.parse().unwrap());
            let velocity = Vector2i::new(v_x.parse().unwrap(), v_y.parse().unwrap());
            Robot { position, velocity }
        })
        .collect()
}

const TEST_INPUT: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
const TEST_INPUT2: &str = "p=2,4 v=2,-3";

use utils::{get_input_path, read_lines};

struct BallCount {
    blue: u32,
    red: u32,
    green: u32,
}

impl BallCount {
    fn new() -> BallCount {
        BallCount {
            blue: 0,
            red: 0,
            green: 0,
        }
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

struct Game {
    id: u32,
    ball_counts: Vec<BallCount>,
}

fn parse_line(line: &str) -> Game {
    let id_split = line.split_once(':').unwrap();
    let id: u32 = id_split.0[5..].parse::<u32>().unwrap();

    let extractions_str = id_split.1;
    let extractions_splits: Vec<_> = extractions_str
        .split_terminator(';')
        .into_iter()
        .map(|extraction_str| extraction_str.split(',').collect::<Vec<&str>>())
        .collect();

    let ball_counts: Vec<_> = extractions_splits
        .iter()
        .map(|extraction_str| {
            extraction_str
                .into_iter()
                .fold(BallCount::new(), |ball_count, ball_str| {
                    let mut ball_split = ball_str.trim().split(' ');
                    let count = ball_split.next().unwrap().trim().parse::<u32>().unwrap();
                    let color = ball_split.next().unwrap();

                    match color {
                        "red" => BallCount {
                            red: count,
                            ..ball_count
                        },
                        "blue" => BallCount {
                            blue: count,
                            ..ball_count
                        },
                        "green" => BallCount {
                            green: count,
                            ..ball_count
                        },
                        _ => {
                            panic!("Unknown color")
                        }
                    }
                })
        })
        .collect();

    Game {
        id: id,
        ball_counts: ball_counts,
    }
}

fn main() {
    let input_path = get_input_path(2, Some("input.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let games: Vec<_> = lines.into_iter()
            .map(|line| parse_line(&line.unwrap()))
            .collect();

        let min_viable_counts: Vec<_> = games.into_iter()
            .map(|game| {
                let red = game.ball_counts.iter().map(|count| count.red).max().unwrap();
                let green = game.ball_counts.iter().map(|count| count.green).max().unwrap();
                let blue = game.ball_counts.iter().map(|count| count.blue).max().unwrap();

                BallCount { red, green, blue}
            })
            .collect();

        let powers_sum: u32 = min_viable_counts.iter().map(BallCount::power).sum();
        println!("The sum of powers is {powers_sum}")
    }
}

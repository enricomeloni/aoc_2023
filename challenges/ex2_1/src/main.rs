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

    fn is_possible(&self, max_count: &BallCount) -> bool {
        self.blue <= max_count.blue && self.green <= max_count.green && self.red <= max_count.red
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

const MAX_COUNT: BallCount = BallCount {
    red: 12,
    green: 13,
    blue: 14
};

fn main() {
    let input_path = get_input_path(2, Some("input.txt"));

    if let Ok(lines) = read_lines(&input_path) {
        let games: Vec<_> = lines.into_iter()
            .map(|line| parse_line(&line.unwrap()))
            .collect();

        let ids_sum: u32 = games.iter()
        .filter(|game| game.ball_counts.iter().all(|ball_count| ball_count.is_possible(&MAX_COUNT)))
        .map(|game| game.id)
        .sum();

        println!("The sum of ids is {ids_sum}")
    }
}

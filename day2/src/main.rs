use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn main() {
    let mut score: u32 = 0;
    if let Ok(lines) = read_lines("./data") {
        for line in lines {
            if let Ok(l) = line {
                if let Some(opponent) = l.chars().nth(0) {
                    if let Some(player) = l.chars().nth(2) {
                        score += get_score_part2(opponent, player);
                    }
                }
            }
        }
    }

    println!("{score}");
}

fn get_score_part2(opponent: char, player: char) -> u32 {
    let mut player_score = 0;

    // Get score for the used thing
    match player {
        'X' => player_score += 0,
        'Y' => player_score += 3,
        'Z' => player_score += 6,
        _ => {}
    }

    match player {
        'X' if opponent == 'A' => player_score += 3,
        'X' if opponent == 'B' => player_score += 1,
        'X' if opponent == 'C' => player_score += 2,
        'Y' if opponent == 'A' => player_score += 1,
        'Y' if opponent == 'B' => player_score += 2,
        'Y' if opponent == 'C' => player_score += 3,
        'Z' if opponent == 'A' => player_score += 2,
        'Z' if opponent == 'B' => player_score += 3,
        'Z' if opponent == 'C' => player_score += 1,
        _ => {}
    }

    player_score
}

fn get_score_part1(opponent: char, player: char) -> u32 {
    // Opponent can be A, B or C
    // Player can be X, Y or Z
    // player score Rock = 1
    //              Paper = 2
    //              Sciccor = 3
    //  0 if lost
    //  3 if drawn
    //  6 if won

    let mut player_score = 0;

    // Get score for the used thing
    match player {
        'X' => player_score += 1,
        'Y' => player_score += 2,
        'Z' => player_score += 3,
        _ => {}
    }

    match opponent {
        'A' if player == 'X' => player_score += 3,
        'A' if player == 'Y' => player_score += 6,
        'B' if player == 'Z' => player_score += 6,
        'B' if player == 'Y' => player_score += 3,
        'C' if player == 'X' => player_score += 6,
        'C' if player == 'Z' => player_score += 3,
        _ => {}
    }

    player_score
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

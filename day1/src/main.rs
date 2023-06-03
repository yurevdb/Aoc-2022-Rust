use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

struct Elf {
    calories: u32,
}

impl Elf {
    pub fn new(calories: u32) -> Elf {
        Elf { calories }
    }
}

fn main() {
    let mut elves: Vec<Elf> = Vec::new();
    let mut elf: Elf = Elf::new(0);

    if let Ok(lines) = read_lines("./data") {
        for line in lines {
            if let Ok(l) = line {
                match l.parse::<u32>() {
                    Ok(calories) => elf.calories += calories,
                    Err(_) => {
                        elves.push(elf);
                        elf = Elf::new(0);
                    }
                }
            }
        }
    };

    elves.sort_by(|a, b| b.calories.cmp(&a.calories));

    // Part 1
    // if let Some(elf) = elves.first() {
    //     println!("{}", elf.calories);
    // }

    // Part 2
    let top3: &[Elf] = &elves[0..=2];
    let mut total = 0;
    for elf in top3 {
        total += elf.calories;
    }
    println!("{total}");
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

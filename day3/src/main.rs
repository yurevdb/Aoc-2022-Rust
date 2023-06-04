use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
    path::Path,
};

fn main() {
    part2();
}

fn part2() {
    let mut groups: Vec<Vec<String>> = Vec::new();
    let mut group: Vec<String> = Vec::new();
    let mut count = 0;
    let group_size = 3;

    if let Ok(lines) = read_lines("./data") {
        for line in lines {
            if let Ok(l) = line {
                group.push(l);
                count += 1;
            }
            if count == group_size {
                groups.push(group.clone());
                group.clear();
                count = 0;
            }
        }
    }

    assert!(groups.len() == 300 / group_size);

    let mut total_priority = 0;
    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    for g in groups {
        'search: for char in g[0].chars() {
            if g[1].contains(char) && g[2].contains(char) {
                total_priority += alphabet.find(char).unwrap() + 1;
                break 'search;
            }
        }
    }

    println!("{total_priority}");
}

fn par1() {
    let mut total_priority = 0;

    let alphabet = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";

    if let Ok(lines) = read_lines("./data") {
        for line in lines {
            if let Ok(l) = line {
                let (comp1, comp2) = l.split_at(l.len() / 2);
                assert!(comp1.len() == comp2.len());
                'comp: for char in comp1.chars() {
                    if comp2.contains(char) {
                        total_priority += alphabet.find(char).unwrap() + 1;
                        break 'comp;
                    }
                }
            }
        }
    }

    println!("{total_priority}");
}

fn read_lines<P>(filename: P) -> io::Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

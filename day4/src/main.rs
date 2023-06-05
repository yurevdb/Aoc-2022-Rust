use std::{
    fs::File,
    io::{self, BufRead, BufReader, Lines},
};

struct FileReader {
    lines: Lines<BufReader<File>>,
}

impl FileReader {
    fn new(filename: &str) -> Self {
        if let Ok(file) = File::open(filename) {
            let lines = io::BufReader::new(file).lines();
            return FileReader { lines };
        } else {
            panic!("File could not open");
        }
    }
}

impl Iterator for FileReader {
    type Item = String;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(Ok(l)) = self.lines.next() {
            return Some(l);
        }
        None
    }
}

fn main() {
    let fr = FileReader::new("./data");

    let mut count1 = 0;
    let mut count2 = 0;

    for line in fr {
        // Unwrap safe because of input constraints
        let (r1, tr2) = line.split_at(line.find(',').unwrap());
        let r2 = tr2[1..].to_owned();

        let (lower_r1, upper_r1) = get_bounds(r1.to_owned());
        let (lower_r2, upper_r2) = get_bounds(r2);

        if lower_r1 <= lower_r2 && upper_r1 >= upper_r2 {
            count1 += 1;
        } else if lower_r2 <= lower_r1 && upper_r2 >= upper_r1 {
            count1 += 1;
        }

        if (lower_r1 <= lower_r2 && lower_r2 <= upper_r1)
            || (lower_r1 <= upper_r2 && upper_r2 <= upper_r1)
            || (lower_r2 <= lower_r1 && lower_r1 <= upper_r2)
            || (lower_r2 <= upper_r1 && upper_r1 <= upper_r2)
        {
            count2 += 1;
        }
    }

    println!("{count1}");
    println!("{count2}");
}

fn get_bounds(input: String) -> (u32, u32) {
    // input := u32-u32
    let (p1, tp2) = input.split_at(input.find('-').unwrap());
    let p2 = tp2[1..].to_owned();

    let lower: u32 = u32::from_str_radix(p1, 10).unwrap();
    let upper: u32 = u32::from_str_radix(p2.as_str(), 10).unwrap();

    (lower, upper)
}

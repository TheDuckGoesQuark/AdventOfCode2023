use std::{
    env,
    fs::File,
    io::{BufRead, BufReader, Lines, Result},
    path::Path,
};

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> Result<Lines<BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(BufReader::new(file).lines())
}

fn read_args() -> Vec<String> {
    return env::args().collect();
}

fn main() {
    // File hosts.txt must exist in the current path
    let mut sum = 0;
    let args = read_args();

    let filename = args.get(2).unwrap();
    println!("Reading file {}", filename);

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                sum = sum + extract_number(ip)
            }
        }

        println!("Total: {}", sum)
    }
}

fn extract_number(line: String) -> i32 {
    // search backwards
    let chars: Vec<char> = line.chars().filter(|c| c.is_digit(10)).collect();
    match chars.len() {
        0 => {
            return 0;
        }
        _ => {
            let digits = format!("{}{}", chars[0], chars.last().unwrap());
            return digits.parse().unwrap_or(0);
        }
    };
}

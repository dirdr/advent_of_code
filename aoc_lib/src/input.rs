use std::{
    fs::File,
    io::{self, BufRead, BufReader},
};

pub fn read_file(path: &str) -> io::Result<Vec<String>> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let mut output: Vec<String> = vec![];
    let lines = reader.lines();
    for line in lines {
        output.push(line?);
    }
    Ok(output)
}

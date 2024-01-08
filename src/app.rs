use color_eyre::eyre::Result;
use std::path::{PathBuf, Path};
use std::fs::File;
use std::io::{BufReader, BufRead};

enum Mode{
    Insert
}

pub struct Cursor{
    pub current: (u32, u32),
    pub possible: (u32, u32)
}

impl Cursor{
    fn new() -> Cursor{
        Cursor { current: (0,0), possible: (0,0) }
    }
}

pub struct Line{
    pub text: String
}

pub struct Lines{
    pub lines: Vec<Line>
}

pub struct Editor{
    pub cursor: Cursor,
    pub lines: Lines,
    pub file: PathBuf,
    pub mode: Mode,
    pub should_quit: bool
}

impl Editor{
    pub fn new(path: &Path)-> Result<Editor> {
        // open file passed
        // load file to memory

        let file = File::open(&path);

        if let Ok(file) = file {
            let reader = BufReader::new(file);

            let mut lines: Vec<Line> = vec![];

            for line in reader.lines() {
                match line {
                    Ok(text) => {lines.push(Line { text });}
                    Err(_) => {}
                }
            }
            
            let lines = Lines { lines };

            return Ok(Editor { cursor: Cursor::new(), lines, file: path.to_owned(), mode: Mode::Insert, should_quit: false});
        }

        panic!("No file passed");
    }
}



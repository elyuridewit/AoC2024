use std::{env, fs};
use ndarray::Array2;

struct GuardMap {
    map: Array2<char>,              // The map
    guardy: usize,                  // Current position of the Guard in the map
    guardx: usize,                  // Current X position of the Guard in the map
    guarddir: char,                 // 'U' for up, 'D' for down, 'L' for left, 'R' for right
    distinct: Vec<(usize, usize)>,  // All distinct pos 
    ybound: usize,                  // The Y limit of the map
    xbound: usize,                  // The X limit of the map
}

impl GuardMap {
    fn walk_map(&mut self) {
        let mut exit: bool = false;
        while !exit {
            let nextpos: (usize, usize) = match self.guarddir {
                '^' if self.guardy != 0             => (self.guardy - 1, self.guardx),
                '>' if self.guardx != self.xbound   => (self.guardy, self.guardx + 1),
                'v' if self.guardy != self.ybound   => (self.guardy + 1, self.guardx),
                '<' if self.guardx != 0             => (self.guardy, self.guardx - 1),
                _                                   => (self.guardy, self.guardx)
            };
            // If no update to guardpos, then default arm was hit == out of map
            if nextpos.0 == self.guardy && nextpos.1 == self.guardx {
                exit = true;
                continue;
            }

            let next_obj: char = self.map[nextpos];

            if next_obj == '#' {
                // Turn 90 deg
                self.guarddir = match self.guarddir {
                    '^' => '>',
                    '>' => 'v',
                    'v' => '<',
                    '<' => '^',
                    _ => panic!("Sum shit happened again")
                };
                self.map[(self.guardy, self.guardx)] = self.guarddir;
            } else {
                // Set current pos to 'X'
                self.map[(self.guardy, self.guardx)] = 'X';
                // Set next pos to the guarddir char
                self.map[nextpos] = self.guarddir;
                // Add the new pos to the distinct values, if it hasn't been walked before
                if next_obj != 'X' {
                    self.distinct.push(nextpos);
                }
                // Set the current pos to the next pos
                self.guardy = nextpos.0;
                self.guardx = nextpos.1;
            }

            // println!("{:?}", self.map);
        }
    }
}

fn map_builder(jvec: Vec<Vec<char>>) -> GuardMap {
    let xbound = jvec[0].len();
    let ybound = jvec.len();
    let mut array: Array2<char> = Array2::from_elem((jvec.len(), xbound), ' ');
    let mut guardpos: (usize, usize) = (0, 0);

    for (i, row) in jvec.iter().enumerate() { 
        for (j, &val) in row.iter().enumerate() { 
            array[(i, j)] = val;
            if val == '^' {
                guardpos = (i, j)
            }
        } 
    }

    GuardMap{
        map: array,
        guardy: guardpos.0,
        guardx: guardpos.1,
        guarddir: '^',
        distinct: vec![guardpos],
        ybound: ybound - 1,
        xbound: xbound - 1,
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let filecontent: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error),
    };

    let mut jagged_vec: Vec<Vec<char>> = Vec::new();
    for line in filecontent.lines() {
        let line_vec: Vec<char> = line.chars().collect();
        jagged_vec.push(line_vec);
    }

    let mut map: GuardMap = map_builder(jagged_vec);
    // println!("{:?}", map.map);
    map.walk_map();

    println!("Distinct locations visited: {:?}", map.distinct.len())

}

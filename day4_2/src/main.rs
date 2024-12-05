use std::{env, fs};


struct MasCheck {
    matrix: Vec<Vec<char>>
}
impl MasCheck {
    fn mas_checker(&self, yidx: usize, xidx: usize) -> bool {
        let topleft = self.matrix[yidx-1][xidx-1];
        let bottomright = self.matrix[yidx+1][xidx+1];
        let bottomleft = self.matrix[yidx+1][xidx-1];
        let topright = self.matrix[yidx-1][xidx+1];
        // Topleft -> bottomright

        if (topleft == 'M' && bottomright == 'S' || topleft == 'S' && bottomright == 'M') && (bottomleft == 'M' && topright == 'S' || bottomleft == 'S' && topright == 'M') {
            return true
        }
        false
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
    
    let xbound = jagged_vec[0].len() - 1;
    let ybound = jagged_vec.len() - 1;
    let mas_check: MasCheck = MasCheck{
        matrix: jagged_vec.clone()
    };
    
    let mut total: usize = 0;
    for (yidx, line) in jagged_vec.into_iter().enumerate() {
        for (xidx, char) in line.into_iter().enumerate() {
            if yidx != 0 && xidx != 0 && yidx < ybound && xidx < xbound {
                if char == 'A' {
                    if mas_check.mas_checker(yidx, xidx) {
                        total += 1;
                    }
                }
            }
        }
    }

    println!("Total: {:?}", total)
}

use std::{collections::HashMap, env, fs};

fn main() {
    let args: Vec<String> = env::args().collect();
    let file_name = &args[1];
    let filecontent: String = match fs::read_to_string(file_name) {
        Ok(content) => content,
        Err(error) => panic!("Error opening file. Error: {:?}", error),
    };

    let mut reached_updates: bool = false;
    let mut raw_update_rules: Vec<&str> = Vec::new();
    let mut updates: Vec<Vec<usize>> = Vec::new();
    for line in filecontent.lines(){
        if line == "" { reached_updates = true; continue; }
        if !reached_updates {
            raw_update_rules.push(line);
        } else {
            updates.push(line.split(",").map(|x| x.parse().unwrap()).collect());
        }
    }

    let mut rules: HashMap<usize, Vec<usize>> = HashMap::new();
    for raw_rule in raw_update_rules {
        let rule = raw_rule.split_once("|").unwrap();
        let (nr, before): (usize, usize) = (rule.0.parse().unwrap(), rule.1.parse().unwrap());
        
        if rules.contains_key(&nr) {
            rules.get_mut(&nr).unwrap().push(before);
        } else {
            rules.insert(nr, vec![before]);
        }
    }

    let mut total: usize = 0;
    let mut correction_total: usize = 0;
    for update in updates {
        let (correct, page, before) = check_update(update.clone(), rules.clone());
        if correct {
            // Get the middle value and add it to total
            total += update[update.len() / 2]
        } else {
            let mut corrected: bool = false;
            let mut new_update = update.clone();
            let (mut new_page, mut new_before) = (page.clone(), before.clone());
            while !corrected {
                let pageidx = new_update.iter().position(|&x| x == new_page).unwrap();
                let beforeidx = new_update.iter().position(|&x| x == new_before).unwrap();

                new_update.swap(pageidx, beforeidx);

                let (correct, page, before) = check_update(new_update.clone(), rules.clone());
                if correct {
                    correction_total += new_update[new_update.len() / 2];
                    corrected = true
                } else {
                    new_page = page;
                    new_before = before
                }
            }
        }
    }
    // Part 1
    println!("Total: {:?}", total);
    // Part 2
    println!("Incorrect update total after correction: {:?}", correction_total)

}

fn check_update(update: Vec<usize>, rules: HashMap<usize, Vec<usize>>) -> (bool, usize, usize) {
    for (idx, page) in update.iter().enumerate() {
        // Check if the page number even has a rule
        if rules.contains_key(&page) {
            let before_vec = rules.get(&page).unwrap();
            for before in before_vec {
                // Find the index of the before nr in the update. If it exists
                let beforeidx = match update.iter().position(|&x| x == before.clone()) {
                    Some(x) => x,
                    None => continue
                };

                // If we reach this, check if the index of beforeidx > idx of the page
                if beforeidx < idx {
                    // println!("Incorrect update: {:?}", update);
                    // println!("Incorrect because {:?} isn't before {:?}", page, before);
                    return (false, *page, *before)
                }
            }
        }
    }
    // println!("Correct update: {:?}", update);
    (true, 0, 0)
}

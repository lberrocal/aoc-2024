use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() -> io::Result<()> {
    let path = Path::new("input.txt");
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut left_list: Vec<i32> = Vec::new();
    let mut right_list: Vec<i32> = Vec::new();
    let mut line_numbers = 0;

    for line in reader.lines() {
        let line_content = line?;
        let parts: Vec<&str> = line_content.split_whitespace().collect();
        if parts.len() == 2 {
            line_numbers += 1;
            if let (Ok(left), Ok(right)) = (parts[0].parse(), parts[1].parse()) {
                ordered_insert(&mut left_list, left);
                ordered_insert(&mut right_list, right);
            }
        }
    }

    let mut total_difference = 0;
    let mut similarity_score = 0;
    let mut last_visited_right_index = 0;
    let mut i = 0;

    
    // part 1: By having the lists sorted it's only a matter of doing an absolute 
    // difference between each element of the list and adding them together
    for j in 0..= line_numbers-1{
        total_difference += right_list[j].abs_diff(left_list[j]);
    }

    // for part 2, you would find how many repeats of elements are in list one 
    // and how often they happen in list 2 and multiply them. One optimization used is
    // to limit the search range in the second array since it's sorted already
    // which is where the last visited right index is used. 
    while i < line_numbers {
        let mut element_repeats = 1;

        //finding number of element repeats on the first list
        while i < line_numbers - 1 && left_list[i] == left_list[i + 1] {
            element_repeats += 1;
            i += 1;
        }

        //finding the number of ocurrences of target element in list 2
        let mut occurrences = 0;
        let target = left_list[i];

        let subset = &right_list[last_visited_right_index..line_numbers];
        match subset.binary_search(&target) {
            Ok(start_index) => {
                println!("found element at index {}", start_index);
                // Check to the left
                occurrences += 1;
                let mut left = start_index;
                while left > 0 && subset[left - 1] == target {
                    occurrences += 1;
                    left -= 1;
                }

                // Check to the right
                let mut right = start_index;
                while right < subset.len() - 1 && subset[right + 1] == target {
                    occurrences += 1;
                    right += 1;
                }
                last_visited_right_index += right
            }
            Err(_) => occurrences = 0,
        }
        similarity_score += target * element_repeats * occurrences as i32;
        i += 1;
    }

    println!("Total Difference {}", total_difference);
    println!("Similarity Score {}", similarity_score);
    Ok(())
}

fn ordered_insert(vec: &mut Vec<i32>, value: i32) {
    // Perform a binary search to find the correct insertion point
    match vec.binary_search(&value) {
        Ok(pos) => vec.insert(pos, value), // If the element already exists, insert at the found position
        Err(pos) => vec.insert(pos, value), // If the element does not exist, insert at the position where it would go
    }
}

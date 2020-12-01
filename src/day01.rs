use std::collections::HashMap;

// use a hashmap to store the difference and run in O(N) for time and space
pub fn find_two_sum(nums_vec: Vec<i32>, target: i32) -> Option<(usize, usize)> {
    let mut diff_map: HashMap<i32, usize> = HashMap::new();

    for (i, expense) in nums_vec.iter().enumerate() {
        let diff: i32 = target - expense;
        if diff_map.contains_key(&diff) {
            return Some((*diff_map.get(&diff)?, i));
        }
        diff_map.insert(nums_vec[i], i);
    }
    None
}
pub fn part_one(raw_input: Vec<String>) -> Option<i32> {
    let expenses: Vec<i32> = raw_input.iter().map(|v| v.parse().unwrap()).collect();
    let two_index = find_two_sum(expenses.clone(), 2020)?;
    let answer = expenses[two_index.0] * expenses[two_index.1];
    println!("Part One answer is: {}", answer);
    Some(answer)
}

pub fn part_two(raw_input: Vec<String>) -> Option<i64> {
    let expenses: Vec<i32> = raw_input.iter().map(|v| v.parse().unwrap()).collect();
    let target: i32 = 2020;

    for (i, expense) in expenses.iter().enumerate() {
        let diff: i32 = target - expense;
        if diff > 0 {
            let two_index = find_two_sum(expenses[i..].to_vec(), diff);
            if let Some((i1, i2)) = two_index {
                let answer: i64 =
                    expenses[i] as i64 * expenses[i + i1] as i64 * expenses[i + i2] as i64;
                println!("Part Two answer is: {}", answer);
                return Some(answer);
            }
        }
    }

    None
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_two_sum() {
        let nums_vec = vec![1721, 979, 366, 299, 675, 1456];
        assert_eq!(find_two_sum(nums_vec, 2020), Some((0 as usize, 3 as usize)));
    }
    #[test]
    fn test_part_one() {
        // Your puzzle answer was 1016131.
        let nums_vec = vec![
            "1721".to_string(),
            "979".to_string(),
            "366".to_string(),
            "299".to_string(),
            "675".to_string(),
            "1456".to_string(),
        ];
        assert_eq!(part_one(nums_vec), Some(514579 as i32));
    }
    #[test]
    fn test_part_two() {
        // Your puzzle answer was 276432018.
        let nums_vec = vec![
            "1721".to_string(),
            "979".to_string(),
            "366".to_string(),
            "299".to_string(),
            "675".to_string(),
            "1456".to_string(),
        ];
        assert_eq!(part_two(nums_vec), Some(241861950 as i64));
    }
}

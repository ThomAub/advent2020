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

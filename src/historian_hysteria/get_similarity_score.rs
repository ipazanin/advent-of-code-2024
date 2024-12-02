use std::collections::HashMap;

pub fn get_similarity_score(left_items: &Vec<i32>, right_items: &Vec<i32>) -> i32 {
    let mut right_items_map: HashMap<i32, i32> = HashMap::new();
    let mut similarity_score = 0;

    for item in right_items {
        let current_count = *right_items_map.get(&item).unwrap_or(&0);

        right_items_map.insert(*item, current_count + 1);
    }

    for item in left_items {
        let count = right_items_map.get(&item).unwrap_or(&0);

        similarity_score += item * count;
    }

    similarity_score
}

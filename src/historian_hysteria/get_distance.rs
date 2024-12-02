pub fn get_distance(mut left_items: Vec<i32>, mut right_items: Vec<i32>) -> i32 {
    let mut distance = 0;

    left_items.sort();
    right_items.sort();

    for i in 0..left_items.len() {
        let left = left_items[i];
        let right = right_items[i];

        if right > left {
            distance += right - left;
        } else {
            distance += left - right;
        }
    }

    distance
}

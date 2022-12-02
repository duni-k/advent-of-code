pub fn get_top_calories(lines: impl Iterator<Item = String>, nr_of_elves: usize) -> usize {
    let mut top_sums = vec![usize::min_value(); nr_of_elves];
    let mut curr_sum = 0;
    for line in lines {
        if let Ok(calories) = line.parse::<usize>() {
            curr_sum += calories;
        } else {
            if let Some(i) = top_sums.iter().position(|&top| top < curr_sum) {
                top_sums.insert(i, curr_sum);
                top_sums.pop();
            }
            curr_sum = 0;
        }
    }

    top_sums.iter().sum()
}

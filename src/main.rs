#[allow(dead_code)]
mod day_1_calorie_counting;
mod day_2_rock_paper_scissors;
mod day_3_rucksack_reorganization;
mod day_4_camp_cleanup;
mod day_5_supply_stacks;

const DAY: i8 = 5;

fn main() {
    match DAY {
        1 => day_1_calorie_counting::solve(),
        2 => day_2_rock_paper_scissors::solve(),
        3 => day_3_rucksack_reorganization::solve(),
        4 => day_4_camp_cleanup::solve(),
        5 => day_5_supply_stacks::solve(),
        _ => (),
    }
}

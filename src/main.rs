use std::env;

mod day_1_calorie_counting;
mod day_2_rock_paper_scissors;
mod day_3_rucksack_reorganization;
mod day_4_camp_cleanup;
mod day_5_supply_stacks;
mod day_6_tuning_trouble;
mod day_7_no_space_left_on_device;
mod day_8_treetop_tree_house;

fn main() {
    let maybe_day_number: Option<String> = env::args().nth(1);

    match maybe_day_number {
        Some(day_number) => match day_number.parse::<u8>() {
            Ok(1) => day_1_calorie_counting::solve(),
            Ok(2) => day_2_rock_paper_scissors::solve(),
            Ok(3) => day_3_rucksack_reorganization::solve(),
            Ok(4) => day_4_camp_cleanup::solve(),
            Ok(5) => day_5_supply_stacks::solve(),
            Ok(6) => day_6_tuning_trouble::solve(),
            Ok(7) => day_7_no_space_left_on_device::solve(),
            Ok(8) => day_8_treetop_tree_house::solve(),
            _ => println!("Enter a number between 1 and 25."),
        },
        None => println!("Enter day number as argument."),
    }
}

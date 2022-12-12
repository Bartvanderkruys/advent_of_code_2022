use std::env;

mod day_01_calorie_counting;
mod day_02_rock_paper_scissors;
mod day_03_rucksack_reorganization;
mod day_04_camp_cleanup;
mod day_05_supply_stacks;
mod day_06_tuning_trouble;
mod day_07_no_space_left_on_device;
mod day_08_treetop_tree_house;
mod day_09_rope_bridge;
mod day_10_cathode_ray_tube;
mod day_11_monkey_in_the_middle;
mod day_12_hill_climbing_algorithm;
mod day_13;
mod day_14;
mod day_15;
mod day_16;
mod day_17;
mod day_18;
mod day_19;
mod day_20;
mod day_21;
mod day_22;
mod day_23;
mod day_24;
mod day_25;

fn main() {
    let maybe_day_number: Option<String> = env::args().nth(1);

    match maybe_day_number {
        Some(day_number) => match day_number.parse::<u8>() {
            Ok(1) => day_01_calorie_counting::solve(),
            Ok(2) => day_02_rock_paper_scissors::solve(),
            Ok(3) => day_03_rucksack_reorganization::solve(),
            Ok(4) => day_04_camp_cleanup::solve(),
            Ok(5) => day_05_supply_stacks::solve(),
            Ok(6) => day_06_tuning_trouble::solve(),
            Ok(7) => day_07_no_space_left_on_device::solve(),
            Ok(8) => day_08_treetop_tree_house::solve(),
            Ok(9) => day_09_rope_bridge::solve(),
            Ok(10) => day_10_cathode_ray_tube::solve(),
            Ok(11) => day_11_monkey_in_the_middle::solve(),
            Ok(12) => day_12_hill_climbing_algorithm::solve(),
            Ok(13) => day_13::solve(),
            Ok(14) => day_14::solve(),
            Ok(15) => day_15::solve(),
            Ok(16) => day_16::solve(),
            Ok(17) => day_17::solve(),
            Ok(18) => day_18::solve(),
            Ok(19) => day_19::solve(),
            Ok(20) => day_20::solve(),
            Ok(21) => day_21::solve(),
            Ok(22) => day_22::solve(),
            Ok(23) => day_23::solve(),
            Ok(24) => day_24::solve(),
            Ok(25) => day_25::solve(),
            _ => println!("Enter a number between 1 and 25."),
        },
        None => println!("Enter day number as argument."),
    }
}

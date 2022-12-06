#[allow(dead_code)]
mod calorie_counting;
mod camp_cleanup;
mod rock_paper_scissors;
mod rucksack_reorganization;
mod supply_stacks;

const DAY: i8 = 5;

fn main() {
    match DAY {
        1 => calorie_counting::solve(),
        2 => rock_paper_scissors::solve(),
        3 => rucksack_reorganization::solve(),
        4 => camp_cleanup::solve(),
        5 => supply_stacks::solve(),
        _ => (),
    }
}

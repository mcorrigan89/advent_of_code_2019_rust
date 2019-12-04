mod day_one;
mod day_one_input;
mod day_two;
mod day_two_input;

fn main() {
    println!("Day One Part One: {}", day_one::total_fuel_cost_non_recursive(day_one_input::get_input()));
    println!("Day One Part Two: {}", day_one::total_fuel_cost_recursive(day_one_input::get_input()));
    println!("Day Two Part One: {}", day_two::run_program(&mut day_two_input::get_input(), 12, 02));
    let day_two_result = day_two::brute_force_get_answer(&mut day_two_input::get_input(), 19690720);
    println!("Day Two Part Two: Noun {} Verb: {}", day_two_result.noun, day_two_result.verb);
}
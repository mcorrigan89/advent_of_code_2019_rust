use std::f64;

fn fuel_for_mass(mass: f64) -> f64 {
  f64::from(mass / 3 as f64).floor() - f64::from(2)
}


pub fn total_fuel_cost_non_recursive(module_list: Vec<i32>) -> f64 {
  module_list.into_iter().map(|x| fuel_for_mass(f64::from(x))).fold(f64::from(0), |acc, x| acc + x)
}

fn total_fuel_for_fuel(mass: f64) -> f64 {
  if fuel_for_mass(mass) > 0 as f64 {
    fuel_for_mass(mass) + mass
  } else {
    mass
  }
}

fn total_fuel_for_module(module: f64) -> f64 {
  total_fuel_for_fuel(fuel_for_mass(module))
}

pub fn total_fuel_cost_recursive(module_list: Vec<i32>) -> f64 {
  module_list.into_iter().map(|x| total_fuel_for_module(f64::from(x))).fold(f64::from(0), |acc, x| acc + x)
}
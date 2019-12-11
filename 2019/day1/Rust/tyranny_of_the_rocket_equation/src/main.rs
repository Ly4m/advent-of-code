use std::fs;
use std::str::Lines;

fn main() {
    let module_masses = fs::read_to_string("src/module_masses.in").unwrap();
    let sum_of_fuel = calculate_fuel_for_all_modules(module_masses.lines());

    println!("mass for all modules {}", sum_of_fuel);
}

fn calculate_fuel_for_mass(mass: i32) -> i32 {
    return (mass / 3) - 2;
}

fn calculate_fuel_for_all_modules(modules_masses: Lines<'_>) -> i32 {
    return modules_masses
        .map(|x| x.parse().unwrap())
        .map(|x| calculate_fuel_for_mass(x))
        .sum();
}

#[cfg(test)]
mod tests {
    use crate::calculate_fuel_for_mass;

    #[test]
    fn simple_fuel_calculation() {
        assert_eq!(calculate_fuel_for_mass(12), 2)
    }

    #[test]
    fn simple_fuel_calculation_with_rounding() {
        assert_eq!(calculate_fuel_for_mass(14), 2)
    }

    #[test]
    fn intermediate_fuel_calculation_with_rounding() {
        assert_eq!(calculate_fuel_for_mass(1969), 654)
    }

    #[test]
    fn fuel_calculation_with_rounding() {
        assert_eq!(calculate_fuel_for_mass(1969), 654)
    }
}

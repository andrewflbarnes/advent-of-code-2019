use crate::utils;

pub fn solve(input1: String, _: String, _: &[String]) {
    let module_weights: Vec<i32> = utils::read_file_lines(&input1)
        .into_iter()
        .filter_map(|l| l.parse::<i32>().ok())
        .collect();

    let raw_module_fuel: i32 = module_weights.iter().map(calc_fuel_for).sum();

    let total_module_fuel: i32 = module_weights
        .iter()
        .map(calc_fuel_for)
        .map(|f| {
            f + {
                let mut fuel_supply_fuel = 0;
                let mut fuel_supply = f;

                loop {
                    fuel_supply = calc_fuel_for(&fuel_supply);

                    if fuel_supply <= 0 {
                        break fuel_supply_fuel;
                    }

                    fuel_supply_fuel += fuel_supply;
                }
            }
        })
        .sum();

    println!(
        "Rocket requires {} units of fuel for modules",
        raw_module_fuel
    );
    println!(
        "Rocket requires {} units of fuel in total",
        total_module_fuel
    );
}

fn calc_fuel_for(weight: &i32) -> i32 {
    weight / 3 - 2
}

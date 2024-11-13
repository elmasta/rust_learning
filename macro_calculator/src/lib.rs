pub struct Food {
    pub name: String,
    pub calories: [String; 2],
    pub fats: f64,
    pub carbs: f64,
    pub proteins: f64,
    pub nbr_of_portions: f64
}

fn extract_number(input: &str) -> f64 {
    let num_str: String = input.chars().filter(|c| c.is_digit(10) || *c == '.').collect();
    num_str.parse::<f64>().unwrap_or(0.0)
}

fn round_to_two_decimals(num: f64) -> f64 {
    let rounded = format!("{:.2}", num);
    rounded.parse::<f64>().unwrap_or(0.0)
}

pub fn calculate_macros(foods: Vec<Food>) -> json::JsonValue {
    let mut cals: f64 = 0.0;
    let mut carbs: f64 = 0.0;
    let mut proteins: f64 = 0.0;
    let mut fats: f64 = 0.0;
    for food in foods {
        carbs += food.carbs * food.nbr_of_portions;
        proteins += food.proteins * food.nbr_of_portions;
        fats += food.fats * food.nbr_of_portions;
        cals += extract_number(&food.calories[1]) * food.nbr_of_portions;
    }
    let data = json::object!{
        cals: round_to_two_decimals(cals),
        carbs: round_to_two_decimals(carbs),
        proteins: round_to_two_decimals(proteins),
        fats: round_to_two_decimals(fats)
    };
    data.into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn testing_macros_values() {
        let a = Food {
            name: "light milk".to_string(),
            calories: ["148kJ".to_string(), "35kcal".to_string()],
            proteins: 3.5,
            fats: 0.1,
            carbs: 5.0,
            nbr_of_portions: 0.7,
        };
        let b = Food {
            name: "oat cookies".to_string(),
            calories: ["1996kJ".to_string(), "477kcal".to_string()],
            proteins: 8.2,
            fats: 21.0,
            carbs: 60.4,
            nbr_of_portions: 1.2,
        };

        let macros = calculate_macros(vec![a, b]);

        assert_eq!(macros["cals"].as_f64().unwrap(), 596.9);
        assert_eq!(macros["carbs"].as_f64().unwrap(), 75.98);
        assert_eq!(macros["proteins"].as_f64().unwrap(), 12.29);
        assert_eq!(macros["fats"].as_f64().unwrap(), 25.27);
    }

    #[test]
    fn testing_no_food() {
        let macros = calculate_macros(vec![]);

        assert_eq!(macros["cals"].as_f64().unwrap(), 0.0);
        assert_eq!(macros["carbs"].as_f64().unwrap(), 0.0);
        assert_eq!(macros["proteins"].as_f64().unwrap(), 0.0);
        assert_eq!(macros["fats"].as_f64().unwrap(), 0.0);
    }

    #[test]
    fn big_values() {
        let macros = calculate_macros(vec![
            Food {
                name: "big mac".to_string(),
                calories: ["2133.84kJ".to_string(), "510kcal".to_string()],
                proteins: 27.0,
                fats: 26.0,
                carbs: 41.0,
                nbr_of_portions: 2.0,
            },
            Food {
                name: "pizza margherita".to_string(),
                calories: ["1500.59kJ".to_string(), "358.65kcal".to_string()],
                proteins: 13.89,
                fats: 11.21,
                carbs: 49.07,
                nbr_of_portions: 4.9,
            },
        ]);

        assert_eq!(macros["cals"].as_f64().unwrap(), 2777.39);
        assert_eq!(macros["carbs"].as_f64().unwrap(), 322.44);
        assert_eq!(macros["proteins"].as_f64().unwrap(), 122.06);
        assert_eq!(macros["fats"].as_f64().unwrap(), 106.93);
    }
}

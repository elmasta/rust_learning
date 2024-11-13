pub mod mall;
pub use crate::mall::Mall;

use crate::mall::guard;
use mall::guard::Guard;

pub use crate::mall::floor;
pub use crate::mall::floor::Floor;

pub use crate::mall::floor::store;
pub use crate::mall::floor::store::Store;

use mall::floor::store::employee;

fn round_to_two_decimals(num: f64) -> f64 {
    let rounded = format!("{:.4}", num);
    rounded.parse::<f64>().unwrap_or(0.0)
}

pub fn biggest_store(mall: mall::Mall) -> store::Store {
    
    let mut biggest = match mall.floors.iter().flat_map(|floor| floor.stores.iter()).next() {
        Some(first_store) => first_store.clone(),
        None => panic!("No stores found in the mall"),
    };

    for floor in mall.floors {
        for store in floor.stores {
            if store.square_meters > biggest.square_meters {
                biggest = store;
            }
        }
    }
    return biggest;
}

pub fn highest_paid_employee(mall: mall::Mall) -> Vec<employee::Employee> {
    let mut biggest: Vec<employee::Employee> = vec![employee::Employee{name: "bidon".to_string(), age: 2, working_hours: (2, 2), salary: 0.0}];

    for floor in mall.floors {
        for store in floor.stores {
            for employee in store.employees {
                if employee.salary > biggest[0].salary {
                    biggest.clear();
                    biggest.push(employee)
                } else if employee.salary == biggest[0].salary {
                    biggest.push(employee)
                }
            }
        }
    }
    return biggest;
}

pub fn nbr_of_employees(mall: mall::Mall) -> usize {
    let mut nmb_of_employees: usize = mall.guards.len();
    for floor in mall.floors {
        for store in floor.stores {
            nmb_of_employees += store.employees.len();
        }
    }
    return nmb_of_employees
}

pub fn check_for_securities(mall: &mut mall::Mall, mut available_sec: Vec<guard::Guard>) {
    let mut total_size: u64 = 0;
    for floor in &mall.floors {
        total_size += floor.size_limit;
    }
    let current_mall_guards: u64 = mall.guards.len().try_into().unwrap();
    for _ in 0..total_size / 200 - current_mall_guards {
        mall.guards.push(available_sec[0].clone());
        available_sec.remove(0);
    }
}

pub fn cut_or_raise(mall: &mut mall::Mall) {
    for floor in &mut mall.floors {
        for store in &mut floor.stores {
            for mut employee in &mut store.employees {
                if employee.working_hours.1 - employee.working_hours.0 > 10 {
                    employee.salary = round_to_two_decimals(employee.salary * 1.10)
                } else  {
                    employee.salary = round_to_two_decimals(employee.salary * 0.90)
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_mall() -> mall::Mall {
        let secs = vec![
            mall::guard::Guard::new("John Oliver", 34, 7),
            mall::guard::Guard::new("Logan West", 23, 2),
            mall::guard::Guard::new("Bob Schumacher", 53, 15),
        ];

        let footzo_emp = vec![
            mall::floor::store::employee::Employee::new("Finbar Haines", 36, 9, 14, 650.88),
            mall::floor::store::employee::Employee::new("Roksanna Rocha", 45, 13, 22, 772.00),
            mall::floor::store::employee::Employee::new("Sienna-Rose Penn", 26, 9, 22, 1000.43),
        ];
        let swashion_emp = vec![
            mall::floor::store::employee::Employee::new("Abdallah Stafford", 54, 8, 22, 1234.21),
            mall::floor::store::employee::Employee::new("Marian Snyder", 21, 8, 14, 831.90),
            mall::floor::store::employee::Employee::new("Amanda Mclean", 29, 13, 22, 1222.12),
            mall::floor::store::employee::Employee::new("Faizaan Castro", 32, 11, 18, 1106.43),
        ];
        let pizbite_emp = vec![
            mall::floor::store::employee::Employee::new("Juniper Cannon", 21, 16, 23, 804.35),
            mall::floor::store::employee::Employee::new("Alena Simon", 28, 9, 15, 973.54),
            mall::floor::store::employee::Employee::new("Yasemin Collins", 29, 9, 19, 986.33),
            mall::floor::store::employee::Employee::new("Areeb Roberson", 54, 9, 22, 957.82),
            mall::floor::store::employee::Employee::new("Rocco Amin", 44, 13, 23, 689.21),
        ];
        let grill_emp = vec![
            mall::floor::store::employee::Employee::new("Rhian Crowther", 45, 9, 15, 841.18),
            mall::floor::store::employee::Employee::new("Nikkita Steadman", 52, 14, 22, 858.61),
            mall::floor::store::employee::Employee::new("Reginald Poole", 32, 9, 22, 1197.64),
            mall::floor::store::employee::Employee::new("Minnie Bull", 54, 14, 22, 1229.73),
        ];
        let sumo_emp = vec![
            mall::floor::store::employee::Employee::new("Chantelle Barajas", 20, 8, 22, 969.22),
            mall::floor::store::employee::Employee::new("Hywel Rudd", 49, 12, 22, 695.74),
            mall::floor::store::employee::Employee::new("Marianne Beasley", 55, 8, 14, 767.83),
        ];
        let supermaket_emp = vec![
            mall::floor::store::employee::Employee::new("Amara Schaefer", 23, 9, 14, 796.21),
            mall::floor::store::employee::Employee::new("Yara Wickens", 39, 9, 14, 853.42),
            mall::floor::store::employee::Employee::new("Tomi Boyer", 64, 9, 14, 881.83),
            mall::floor::store::employee::Employee::new("Greta Dickson", 42, 9, 14, 775.10),
            mall::floor::store::employee::Employee::new("Caroline Finnegan", 41, 9, 14, 702.92),
            mall::floor::store::employee::Employee::new("Indiana Baxter", 33, 13, 20, 991.71),
            mall::floor::store::employee::Employee::new("Jadine Page", 48, 13, 20, 743.21),
            mall::floor::store::employee::Employee::new("Husna Ryan", 43, 13, 20, 655.75),
            mall::floor::store::employee::Employee::new("Tyler Hunt", 63, 13, 20, 668.25),
            mall::floor::store::employee::Employee::new("Dahlia Caldwell", 56, 13, 20, 781.38),
            mall::floor::store::employee::Employee::new("Chandler Mansell", 20, 19, 24, 656.75),
            mall::floor::store::employee::Employee::new("Mohsin Mcgee", 30, 19, 24, 703.83),
            mall::floor::store::employee::Employee::new("Antoine Goulding", 45, 19, 24, 697.12),
            mall::floor::store::employee::Employee::new("Mark Barnard", 53, 19, 24, 788.81),
        ];

        let ground_stores = vec![
            mall::floor::store::Store::new("Footzo", 50, footzo_emp),
            mall::floor::store::Store::new("Swashion", 43, swashion_emp),
        ];
        let food_stores = vec![
            mall::floor::store::Store::new("PizBite", 60, pizbite_emp),
            mall::floor::store::Store::new("Chillout Grill", 50, grill_emp),
            mall::floor::store::Store::new("Sumo Food", 30, sumo_emp),
        ];
        let supermarket = vec![mall::floor::store::Store::new(
            "Pretail",
            950,
            supermaket_emp,
        )];

        let floors = vec![
            mall::floor::Floor::new("Ground Floor", ground_stores, 300),
            mall::floor::Floor::new("Food Floor", food_stores, 500),
            mall::floor::Floor::new("Supermarket", supermarket, 1000),
        ];

        mall::Mall::new("La Vie Funchal", secs, floors)
    }

    #[test]
    fn biggest_store_tests() {
        let mut shopping_mall = create_mall();

        let mut tested = biggest_store(shopping_mall.clone());

        assert_eq!("Pretail", tested.name);
        assert_eq!(950, tested.square_meters);

        (&mut shopping_mall).floors[2].remove_store("Pretail".to_string());

        tested = biggest_store(shopping_mall.clone());

        assert_eq!("PizBite", tested.name);
        assert_eq!(60, tested.square_meters);
    }

    #[test]
    fn highest_paid_test() {
        let mut shopping_mall = create_mall();

        let mut tested = highest_paid_employee(shopping_mall.clone());
        assert_eq!(1, tested.len());
        assert_eq!("Abdallah Stafford", tested[0].name);
        assert_eq!(54, tested[0].age);

        let salary = shopping_mall.clone().floors[0].stores[0].employees[0].salary;
        shopping_mall.floors[0].stores[0].employees[0].raise(tested[0].salary - salary);
        tested = highest_paid_employee(shopping_mall.clone());

        assert_eq!(2, tested.len());
        assert_eq!("Finbar Haines", tested[0].name);
        assert_eq!(36, tested[0].age);

        assert_eq!(tested[1].salary, tested[0].salary);
    }

    #[test]
    fn nbr_of_employees_test() {
        let mut shopping_mall = create_mall();

        let mut tested = nbr_of_employees(shopping_mall.clone());
        assert_eq!(36, tested);

        shopping_mall.floors[2].stores[0].employees = vec![];

        tested = nbr_of_employees(shopping_mall.clone());
        assert_eq!(22, tested);
    }

    #[test]
    fn check_for_securities_test() {
        let mut shopping_mall = create_mall();

        assert_eq!(3, shopping_mall.guards.len());

        check_for_securities(
            &mut shopping_mall,
            vec![
                mall::guard::Guard::new("Peter Solomons", 45, 20),
                mall::guard::Guard::new("William Charles", 32, 10),
                mall::guard::Guard::new("Leonardo Changretta", 23, 0),
                mall::guard::Guard::new("Vlad Levi", 38, 8),
                mall::guard::Guard::new("Faruk Berkai", 40, 15),
                mall::guard::Guard::new("Chritopher Smith", 35, 9),
                mall::guard::Guard::new("Jason Mackie", 26, 2),
                mall::guard::Guard::new("Kenzie Mair", 34, 8),
                mall::guard::Guard::new("Bentley Larson", 33, 10),
                mall::guard::Guard::new("Ray Storey", 37, 12),
            ],
        );

        assert_eq!(9, shopping_mall.guards.len());
    }

    #[test]
    fn cut_or_raise_test() {
        let mut shopping_mall = create_mall();

        cut_or_raise(&mut shopping_mall);
        assert_eq!(
            585.792,
            shopping_mall.floors[0].stores[0].employees[0].salary
        );
        assert_eq!(
            1100.473,
            shopping_mall.floors[0].stores[0].employees[2].salary
        );

        cut_or_raise(&mut shopping_mall);
        assert_eq!(
            527.2128,
            shopping_mall.floors[0].stores[0].employees[0].salary
        );
        assert_eq!(
            1210.5203,
            shopping_mall.floors[0].stores[0].employees[2].salary
        );
    }
}

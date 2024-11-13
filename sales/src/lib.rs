#[derive(Debug, Clone, PartialEq)]
pub struct Store {
    pub products: Vec<(String, f32)>,
}

impl Store {
    pub fn new(products: Vec<(String, f32)>) -> Store {
        Store { products }
    }

    pub fn get_price(&self, product_name: &str) -> Option<f32> {
        self.products.iter().find(|(name, _)| name == product_name).map(|(_, price)| *price)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Cart {
    pub items: Vec<f32>,
    pub receipt: Vec<f32>,
}

fn round_to_two_decimal_places(num: f32) -> f32 {
    (num * 100.0).round() / 100.0
}

impl Cart {
    pub fn new() -> Cart {
        Cart {
            items: Vec::new(),
            receipt: Vec::new(),
        }
    }

    pub fn insert_item(&mut self, store: &Store, item_name: String) {
        if let Some(price) = store.get_price(&item_name) {
            self.items.push(price);
        }
    }

    pub fn generate_receipt(&mut self) -> Vec<f32> {
        if self.items.is_empty() {
            return vec![];
        }
        self.items.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let nbr_of_item_to_substract: usize = self.items.len()/3;
        let mut total_to_remove: f32 = 0.0;
        let total_price: f32 = self.items.iter().sum();
        for i in 0..nbr_of_item_to_substract {
            total_to_remove += self.items[i];
        }
        for price in &self.items {
            let percent: f32 = (price / total_price) * 100.0;
            self.receipt.push(round_to_two_decimal_places(price - (percent/100.0) * total_to_remove));
        }
        self.receipt.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug)]
    struct Tests {
        carts: Vec<(Cart, Vec<f32>)>,
    }

    fn add_items(s: &Store, items: Vec<&str>, c: &mut Cart) {
        for item in items.iter() {
            c.insert_item(s, item.to_string());
        }
    }

    impl Tests {
        fn new() -> Tests {
            let store = Store::new(vec![
                (String::from("product A"), 1.23),
                (String::from("product B"), 23.1),
                (String::from("product C"), 3.12),
                (String::from("product D"), 9.75),
                (String::from("product E"), 1.75),
                (String::from("product F"), 23.75),
                (String::from("product G"), 2.75),
                (String::from("product H"), 1.64),
                (String::from("product I"), 15.23),
                (String::from("product J"), 2.10),
                (String::from("product K"), 54.91),
                (String::from("product L"), 43.99),
            ]);

            let mut c = Cart::new();
            let mut c1 = Cart::new();
            let mut c2 = Cart::new();
            let mut c3 = Cart::new();
            add_items(&store, vec!["product A", "product B", "product C"], &mut c);
            let sol = vec![1.17, 2.98, 22.06];
            add_items(
                &store,
                vec![
                    "product A",
                    "product B",
                    "product C",
                    "product D",
                    "product E",
                    "product F",
                    "product G",
                ],
                &mut c1,
            );
            let sol1 = vec![1.17, 1.67, 2.62, 2.98, 9.31, 22.05, 22.67];
            add_items(
                &store,
                vec![
                    "product A",
                    "product B",
                    "product C",
                    "product D",
                    "product E",
                    "product F",
                    "product G",
                    "product H",
                    "product I",
                ],
                &mut c2,
            );
            let sol2 = vec![1.16, 1.55, 1.65, 2.6, 2.94, 9.2, 14.38, 21.8, 22.42];
            add_items(
                &store,
                vec![
                    "product A",
                    "product B",
                    "product C",
                    "product D",
                    "product E",
                    "product F",
                    "product G",
                    "product H",
                    "product I",
                    "product J",
                    "product K",
                    "product L",
                ],
                &mut c3,
            );
            let sol3 = vec![
                1.18, 1.58, 1.69, 2.02, 2.65, 3.01, 9.39, 14.67, 22.25, 22.88, 42.38, 52.9,
            ];

            Tests {
                carts: vec![(c, sol), (c1, sol1), (c2, sol2), (c3, sol3)],
            }
        }
    }

    #[test]
    fn test_generate_receipt() {
        let cases = Tests::new();

        for (mut c, sol) in cases.carts.into_iter() {
            assert_eq!(c.generate_receipt(), sol);
            assert_eq!(c.receipt, sol);
        }
    }
}

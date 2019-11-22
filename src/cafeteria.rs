// https://www.codewars.com/kata/59f6a855bee845d3cd000046

#[derive(Debug)]
struct Coffee {
    sort: String,
    milk: Vec<Milk>,
    sugar: Vec<Sugar>,
}

#[derive(Debug)]
struct Milk {
    fat: f32,
}

#[derive(Debug)]
struct Sugar {
    sort: String,
}

struct CoffeeBuilder {
    sort: String,
    milk: Vec<Milk>,
    sugar: Vec<Sugar>,
}

impl CoffeeBuilder {
    fn new() -> CoffeeBuilder {
        CoffeeBuilder {
            sort: "".to_string(),
            milk: vec![],
            sugar: vec![]
        }
    }

    fn set_black_coffee(mut self) -> CoffeeBuilder {
        self.sort = "Black".to_string();
        self
    }

    fn set_cubano_coffee(mut self) -> CoffeeBuilder {
        self.sort = "Cubano".to_string();
        self.sugar.push(Sugar { sort: "Brown".to_string() });
        self
    }

    fn set_antoccino_coffee(mut self) -> CoffeeBuilder {
        self.sort = "Americano".to_string();
        self.milk.push(Milk { fat: 0.5 });
        self
    }

    fn with_milk(mut self, fat: f32) -> CoffeeBuilder {
        self.milk.push(Milk { fat: fat });
        self
    }

    fn with_sugar(mut self, sort: String) -> CoffeeBuilder {
        self.sugar.push(Sugar { sort: sort });
        self
    }

    fn build(self) -> Coffee {
        println!("{:?}", self.milk);
        Coffee {
            sort: self.sort,
            milk: self.milk,
            sugar: self.sugar
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_tests() {
        let coffee = CoffeeBuilder::new()
            .set_black_coffee()
            .with_sugar("Regular".to_string())
            .with_milk(3.2)
            .build();
        assert_eq!(format!("{:?}", coffee), "Coffee { sort: \"Black\", milk: [Milk { fat: 3.2 }], sugar: [Sugar { sort: \"Regular\" }] }");

        let coffee = CoffeeBuilder::new().set_antoccino_coffee().build();
        assert_eq!(
            format!("{:?}", coffee),
            "Coffee { sort: \"Americano\", milk: [Milk { fat: 0.5 }], sugar: [] }"
        );

        let coffee = CoffeeBuilder::new().set_cubano_coffee().build();
        assert_eq!(
            format!("{:?}", coffee),
            "Coffee { sort: \"Cubano\", milk: [], sugar: [Sugar { sort: \"Brown\" }] }"
        );
    }
}

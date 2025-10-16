enum Fruit {
    Mango(String),
    Grape(String),
    Peach(String),
}

struct Inventory {
    fruit: Vec<Fruit>,
}

impl Inventory {
    fn available_fruits(&self) {
        for fruit in &self.fruit {
            Inventory::tell_me_joke(fruit);
        }
    }

    fn tell_me_joke(fruit: &Fruit) {
        match fruit {
            Fruit::Mango(joke) => println!("Mango: {}", joke),
            Fruit::Grape(joke) => println!("Grape: {}", joke),
            Fruit::Peach(joke) => println!("Peach: {}", joke),
        }
    }
}

fn main() {
    let m = "A mango a day makes life sweet.".to_string();
    let g = "A grape idea turns into a fine wine.".to_string();
    let p = "A peachy day starts with a smile.".to_string();

    let fruits = vec![
        Fruit::Mango(m),
        Fruit::Grape(g),
        Fruit::Peach(p),
    ];

    let grocery_store = Inventory { fruit: fruits };

    grocery_store.available_fruits();
}

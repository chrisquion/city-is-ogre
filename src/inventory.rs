#[derive(Debug)]
pub struct Inventory {
    bags: Vec<Bag>,
}
impl Inventory {
    pub fn new() -> Inventory {
        let bags: Vec<Bag> = Vec::new();
        Inventory {
            bags
        }
    }
}
#[derive(Debug)]
struct Bag {
    carrying_capacity: i32,
    weight_held: i32,
    items_currently_held: Vec<Item>
}
#[derive(Debug)]
struct Item {
    name: String,
    categories: Vec<ItemClass>,
    uses: i32,
}
#[derive(Debug)]
pub enum ItemClass {
    Equipment,
    Consumable,
}

fn main() {
    let card = create_card(-1);
    println!("{:?}", card);
    println!("{}", card.number);
    println!("{}", card.instance_id);
    println!("{}", card.location);
    println!("{}", card.card_type);
    println!("{}", card.cost);
    println!("{}", card.attack);
    println!("{}", card.defense);
    println!("{}", card.abilities);
    println!("{}", card.my_health_change);
    println!("{}", card.opponent_health_change);
    println!("{}", card.card_draw);
}

fn create_card(location: i8) -> Card{
    Card{
        number: 123,
        instance_id: -1,
        location,
        card_type: 2,
        cost: 4,
        attack: 4,
        defense: 5,
        abilities: String::from("-----"),
        my_health_change: 0,
        opponent_health_change: 0,
        card_draw: 0
    }
}

#[derive(Debug)]
struct Card{
    number: u8,
    instance_id: i8,
    location: i8,
    card_type: u8,
    cost: u8,
    attack: u8,
    defense: u8,
    abilities: String,
    my_health_change: i8,
    opponent_health_change: i8,
    card_draw: u8
}
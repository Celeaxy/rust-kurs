use std::num::ParseIntError;

fn main() {
    let mut tokens = 100;
    let user_input = "8";
    loop {
        let cost = total_cost(user_input).unwrap_or(0);

        if cost > tokens {
            println!("Not enough tokens. You only have {} tokens left.", tokens);
            break;
        } else {
            tokens -= cost;
            println!("You succesfully bought items for {} tokens.\nYou have {} tokens left.", cost, tokens);
        }
    }
}

fn total_cost(quantity_str: &str) -> Result<u32, ParseIntError> {
    let cost_per_item: u32 = 10;
    let quantity = quantity_str.trim().parse::<u32>()?;
    Ok(quantity * cost_per_item)
}

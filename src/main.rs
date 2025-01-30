

enum Snack {
    Chocolate,
    Cookie,
    Cake
}

struct Item {
    calorie: u32,
    snack: Snack
}
struct Elf {
    items: Vec<Item>
}
fn main() {
    let input = include_str!("../input.txt");

    let mut elves = Vec::new();
    let mut items = Vec::new();

    for line in input.lines() {
        if let Some((calorie, snack)) = line.split_once(",") {
            let calorie = calorie.parse::<u32>().unwrap();

            let snack = match snack {
                "Chocolate" => Snack::Chocolate,
                "Cake" => Snack::Cake,
                "Cookie" => Snack::Cookie,
                _ => panic!("Illegal snack {}", snack)
            };

            items.push(Item {
                calorie,
                snack
            });
        } else if line.is_empty() || line.starts_with("---") {
            elves.push(Elf {
                items
            });

            items = Vec::new();
        }

        for e in &elves {
            println!("{:?}", e);
        }
    }
}
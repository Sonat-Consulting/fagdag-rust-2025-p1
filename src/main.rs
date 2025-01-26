#[derive(Debug)]
enum Snack {
    Chocolate,
    Cookie,
    Cake,
}

#[derive(Debug)]
struct Item {
    snack: Snack,
    calorie: u32
}

#[derive(Debug)]
struct Elf {
    backpack: Vec<Item>
}

impl Elf {
    fn tot_calories(&self) -> u32 {
        self.backpack.iter().map(|item| item.calorie).sum()
    }
}

fn main() {
    let input = include_str!("../input.txt");

    let mut elves = Vec::new();
    let mut items = Vec::new();

    for line in input.lines() {
       if let Some((calorie, snack)) = line.split_once(",") {
            let calorie = calorie.parse::<u32>().expect("Should be a number");

            let snack = match snack {
                "Chocolate" => Snack::Chocolate,
                "Cookie" => Snack::Cookie,
                "Cake" => Snack::Cake,
                _ => panic!("Illegal Snack {snack}")
            };

            items.push(Item {
                snack,
                calorie
            });

       } else if line.is_empty() || line.starts_with("---") {
           elves.push(Elf {
               backpack: items
           });
           items = Vec::new();
       }
    }

    let max = elves.iter().map(|x| x.tot_calories()).max();
    if let Some(max) = max {
        println!("{max}");
    } else {
        println!("Unable to get the max value");
    }
}

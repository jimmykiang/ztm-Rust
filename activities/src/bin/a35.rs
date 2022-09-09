// Topic: Match guards & binding
//
// Summary:
// * A tile-based game requires different logic for different kinds
//   of tiles. Print different messages depending on the kind of
//   tile selected.
//
// Requirements:
// * Bricks:
//   * Colored bricks should print "The brick color is [color]"
//   * Other bricks should print "[Bricktype] brick"
// * Water:
//   * Pressure levels 10 and over should print "High water pressure!"
//   * Pressure levels under 10 should print "Water pressure level: [Pressure]"
// * Grass, Dirt, and Sand should all print "Ground tile"
// * Treasure Chests:
//   * If the treasure is Gold and the amount is at least 100, print "Lots of gold!"
// * Everything else shoud not print any messages
//
// Notes:
// * Use a single match expression utilizing guards to implement the program
// * Run the program and print the messages with at least 4 different tiles

#[derive(Debug, PartialEq)]
enum TreasureItem {
    Gold,
    SuperPower,
}

#[derive(Debug)]
struct TreasureChest {
    content: TreasureItem,
    amount: usize,
}

#[derive(Debug)]
struct Pressure(u16);

#[derive(Debug)]
enum BrickStyle {
    Dungeon,
    Gray,
    Red,
}

#[derive(Debug)]
enum Tile {
    Brick(BrickStyle),
    Dirt,
    Grass,
    Sand,
    Treasure(TreasureChest),
    Water(Pressure),
    Wood,
}

fn print_tile(tile: Tile) {
    match tile {
        Tile::Brick(x @ BrickStyle::Gray | x @ BrickStyle::Red) => {
            println!("The brick color is: {:?}", x)
        }
        Tile::Water(x) if x.0 >= 10 => {
            println!("High water pressure: {:?}", x)
        }
        Tile::Water(x) if x.0 < 10 => {
            println!("Normal water pressure: {:?}", x)
        }
        Tile::Treasure(TreasureChest { content, amount })
            if (content == TreasureItem::Gold) & (amount >= 100) =>
        {
            println!("Lots of {:?}: {:?}!", content, amount)
        }
        x @ Tile::Grass | x @ Tile::Dirt | x @ Tile::Sand => println!("Ground tile: {:?}", x),

        _ => {}
    }
}

fn main() {
    let tile = Tile::Brick(BrickStyle::Gray);
    print_tile(tile);

    let tile = Tile::Water(Pressure(11));
    print_tile(tile);

    let tile = Tile::Water(Pressure(8));
    print_tile(tile);

    let tile = Tile::Treasure(TreasureChest {
        amount: 110,
        content: TreasureItem::Gold,
    });
    print_tile(tile);

    let tile = Tile::Treasure(TreasureChest {
        amount: 110,
        content: TreasureItem::SuperPower,
    });
    print_tile(tile);

    let tile = Tile::Dirt;
    print_tile(tile);
}

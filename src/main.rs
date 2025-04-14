mod types;
use types::Bin;
use types::Item;

fn main() {
    let mut bin = Bin::new(240, 190, 120);

    let items = vec![
        Item::new(20, 30, 120),
        Item::new(200, 55, 30),
        Item::new(39, 189, 60),
    ];

    for item in items {
        let fitted = bin.fit_item(item);
        if !fitted {
            println!("Could not fit all items.");
            std::process::exit(1);
        }
    }

    let ply_data = bin.ply_export();
    std::fs::write("output.ply", ply_data).expect("");

    println!("All items fitted, ply string exported.");
}
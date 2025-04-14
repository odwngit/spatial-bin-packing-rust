mod types;
use types::Bin;
use types::Item;
use rand::Rng;

fn main() {
    let mut bin = Bin::new(200, 200, 100);
    let mut rng = rand::thread_rng();

    let mut items = vec![];
    for _ in 0..300 {
        items.push(
            Item::new(
                rng.gen_range(5..30),
                rng.gen_range(5..30),
                rng.gen_range(5..30)
            )
        )
    }
    let total = items.len();

    for i in 0..total {
        println!("Fitting box #{i} out of {total}...");
        let fitted = bin.fit_item(items[i].clone());
        if !fitted {
            println!("Could not fit all items. Breaking...");
            break;
        }
    }

    let ply_data = bin.ply_export();
    std::fs::write("output.ply", ply_data).expect("");

    println!("All items fitted, ply string exported.");
}
use chrono::prelude::*;
use error_chain::error_chain;
//use reqwest;
use rusqlite::{params,Connection};
use serde_derive::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
struct Material {
    category: String,
    name: String,
    shortcut_name: String,
    id: String,
    tier: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct Listing {
    listings: u64,
    unit_price: u64,
    quantity: u64,
}

#[derive(Serialize, Deserialize, Debug)]
struct MaterialListing {
    id: u64,
    buys: Vec<Listing>,
    sells: Vec<Listing>,
}

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

fn get_material_ids_from_database (db:&Connection) -> Result<Vec<String>> {
    let sql_get_material_ids = "SELECT item_id FROM items";
    let mut statement = db.prepare(sql_get_material_ids).unwrap();
    let selected_rows =  statement.query_map([],|row| {Ok(row.get(0)?)}).unwrap();

    let mut list_of_item_ids: Vec<String> = Vec::new();
    for row in selected_rows {
        list_of_item_ids.push(row.unwrap());
    }
    Ok(list_of_item_ids)
}


#[tokio::main]
async fn main() -> Result<()> {
    let materials = fs::read_to_string("materials.json").expect("Unable to read file");
    let materials_list: Vec<Material> =
        serde_json::from_str(&materials).expect("Cannot parse JSON");

    let base_url = "https://api.guildwars2.com/v2";
    let endpoint_selected = "commerce/listings";

    let path = "./material_listings.db";
    let mut db = Connection::open(&path).unwrap();

    // println!("Connected to database");

    // Checking if there is a new material to be added
    // If an item is in the database but not on the materials.json file, there is no problema
    let db_item_ids = get_material_ids_from_database(&db).unwrap();
    for material in &materials_list {
        if !db_item_ids.contains(&material.id) {
            // Add new material to database
            db.execute("INSERT INTO items (item_id, item_name, item_shortcutname, material_type, material_tier) VALUES (?,?,?,?,?)",
             params![material.id,material.name,material.shortcut_name,material.category, material.tier]).unwrap();
        }
    }

    // Start a transaction to the database
    // Transaction is commited only at the end, after all data is ready to be inserted
    // Doesn't waste time with multiple insertions
    let tx = db.transaction().unwrap();

    for material in materials_list {
        let url_request = format!("{}/{}?ids={}", base_url, endpoint_selected, material.id);
        let request_time: DateTime<Local> = Local::now();
        let request_timestamp = request_time.timestamp();
        let response = reqwest::get(&url_request).await.unwrap();

        let listings: Vec<MaterialListing> = response.json::<Vec<MaterialListing>>().await.unwrap();
        // println!("Material: {:?}", material.name);
        // println!("ID: {:?}", listings[0].id);
        // println!("Buy price:{:?}", listings[0].buys[0].unit_price);
        // println!("Sell price:{:?}", listings[0].sells[0].unit_price);
        // println!();

        let sql_insert_item = "INSERT INTO listings (request_time,request_timestamp,item_id,buy_price,buy_number_of_listings,buy_quantity,sell_price,sell_number_of_listings,sell_quantity) VALUES (?,?,?,?,?,?,?,?,?)";
        tx.execute(
            sql_insert_item,
            params![
                request_time.to_rfc3339(),
                request_timestamp,
                material.id,
                listings[0].buys[0].unit_price,
                listings[0].buys[0].listings,
                listings[0].buys[0].quantity,
                listings[0].sells[0].unit_price,
                listings[0].sells[0].listings,
                listings[0].sells[0].quantity,
            ],
        )
        .unwrap();
    }

    tx.commit().unwrap();
    db.close().unwrap();
   // let _debug_point = 1;

   Ok(())
}

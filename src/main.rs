use chrono::{prelude::*};
use error_chain::error_chain;
use reqwest;
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

fn get_name_from_item_id(item_id: String, material_list: &Vec<Material>) -> &String{
    for item in material_list {
        if item.id == item_id {
            return &item.name;
        }
    }
    panic!("Could not find the name for the selected ID")
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


    // Get the TP data for each material and insert them into the database
    // API request is made for all items at once
    // Database inserts are made for each individual item but commited only at the end
    let mut url_request = format!("{}/{}?ids=", base_url, endpoint_selected);
    let request_time: DateTime<Local> = Local::now();
    let request_timestamp = request_time.timestamp();

    // Generating the URL for the API request
    for material in &materials_list {
        url_request.push_str(&material.id);
        url_request.push(',');
    }

    //let time_before_API_request :DateTime<Local> = Local::now();
    let response = reqwest::get(&url_request).await?;
    //let time_after_API_request :DateTime<Local> = Local::now();

    //let time_before_API_response_processing = Local::now();
    let listings: Vec<MaterialListing> = response.json::<Vec<MaterialListing>>().await.unwrap();
    //let time_after_API_response_processing = Local::now();

    //print!("API response received");

    //let time_before_SQL_insert :DateTime<Local> = Local::now();
    for material_listing in listings {
        // println!("Material: {:?}", get_name_from_item_id(material_listing.id.to_string(), &materials_list));
        // println!("ID: {:?}", material_listing.id);
        // println!("Buy price:{:?}", material_listing.buys[0].unit_price);
        // println!("Sell price:{:?}", material_listing.sells[0].unit_price);
        // println!();

        let sql_insert_item = "INSERT INTO listings (request_time,request_timestamp,item_id,buy_price,buy_number_of_listings,buy_quantity,sell_price,sell_number_of_listings,sell_quantity) VALUES (?,?,?,?,?,?,?,?,?)";
        tx.execute(
            sql_insert_item,
            params![
                request_time.to_rfc3339(),
                request_timestamp,
                material_listing.id,
                material_listing.buys[0].unit_price,
                material_listing.buys[0].listings,
                material_listing.buys[0].quantity,
                material_listing.sells[0].unit_price,
                material_listing.sells[0].listings,
                material_listing.sells[0].quantity,
            ],
        )
        .unwrap();
    }
    //let time_after_SQL_insert :DateTime<Local> = Local::now();


    //let time_before_SQL_commit :DateTime<Local> = Local::now();
    tx.commit().unwrap();
    //let time_after_SQL_commit :DateTime<Local> = Local::now();

    db.close().unwrap();


    // let request_duration = time_after_API_request - time_before_API_request;
    // let request_processing_duration = time_after_API_response_processing - time_before_API_response_processing;
    // let insert_duration = time_after_SQL_insert - time_before_SQL_insert;
    // let commit_duration = time_after_SQL_commit - time_before_SQL_commit;

    // println!("API Request duration: {}",request_duration.num_milliseconds());
    // println!("API Request processing duration: {}",request_processing_duration.num_milliseconds());
    // println!("SQL insert loop duration: {}",insert_duration.num_milliseconds());
    // println!("SQL commit duration: {}",commit_duration.num_milliseconds());

   Ok(())
}

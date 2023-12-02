use crate::food_item::{Error, FoodItem, FoodItemPayload};
use crate::food_item_manager::{do_insert_food_item, get_food_item_from_storage, FOOD_ID_COUNTER};
use ic_cdk::api::time;

#[ic_cdk::query]
pub fn get_food_item(id: u64) -> Result<FoodItem, Error> {
    match _get_food_item(&id) {
        Some(item) => Ok(item),
        None => Err(Error::NotFound {
            msg: format!("a food item with id={} not found", id),
        }),
    }
}

// _get_food_item Function:
pub fn _get_food_item(id: &u64) -> Option<FoodItem> {
    get_food_item_from_storage(*id)
}

// add_food_item Function:
#[ic_cdk::update]
pub fn add_food_item(item: FoodItemPayload) -> Option<FoodItem> {
    let id = FOOD_ID_COUNTER
        .with(|counter| {
            let current_value = *counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .expect("cannot increment id counter for food items");
    let food_item = FoodItem {
        id,
        name: item.name,
        quantity: item.quantity,
        created_date: time(),
        expiration_date: time() + 86_400_000_000_000,
    };
    do_insert_food_item(&food_item);
    Some(food_item)
}

// update_food_item Function:
#[ic_cdk::update]
pub fn update_food_item(id: u64, item: FoodItemPayload) -> Result<FoodItem, Error> {
    match get_food_item_from_storage(id) {
        Some(mut food_item) => {
            food_item.name = item.name;
            food_item.quantity = item.quantity;
            food_item.created_date = time();
            food_item.expiration_date;
            do_insert_food_item(&food_item);
            Ok(food_item)
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't update a food item with id={}. item not found", id),
        }),
    }
}

// delete_food_item Function:
#[ic_cdk::update]
pub fn delete_food_item(id: u64) -> Result<FoodItem, Error> {
    match get_food_item_from_storage(id) {
        Some(food_item) => Ok(food_item),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a food item with id={}. item not found.",
                id
            ),
        }),
    }
}

#[test]
fn generate_candid() {
    use std::fs::File;
    use std::io::Write;
    use std::path::PathBuf;

    const DIST_DIR: &str = "";
    const BACKEND_DID: &str = "backend.did";

    candid::export_service!();
    let backend = __export_service();

    File::create(
        PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").unwrap())
            .join(DIST_DIR)
            .join(BACKEND_DID)
            .as_path(),
    )
    .unwrap()
    .write_all(&backend.as_bytes())
    .expect("Unable to write candid file");
}

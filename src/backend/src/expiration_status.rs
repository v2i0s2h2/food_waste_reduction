use crate::food_item_operations::_get_food_item;
use ic_cdk::api::time;
use crate::food_item::{Error, FoodItem};
use crate::food_item_manager::FOOD_STORAGE;


#[ic_cdk::query]
pub fn check_expiration_status(id: u64) -> Result<String, Error> {
    match _get_food_item(&id) {
        Some(food_item) => {
            let current_timestamp = time();
            if current_timestamp > food_item.expiration_date {
                Ok("Expired".to_string())
            } else {
                Ok("Not Expired".to_string())
            }
        }
        None => Err(Error::NotFound {
            msg: format!("a food item with id={} not found", id),
        }),
    }
}

// List All Food Items
#[ic_cdk::query]
pub fn list_all_food_items() -> Vec<FoodItem> {
    FOOD_STORAGE.with(|service| {
        let storage = service.borrow_mut();
        storage.iter().map(|(_, item)| item.clone()).collect()
    })
}

// Search Food Items by Name
#[ic_cdk::query]
pub fn search_food_items_by_name(name: String) -> Vec<FoodItem> {
    FOOD_STORAGE.with(|service| {
        let storage = service.borrow_mut();
        storage
            .iter()
            .filter(|(_, item)| item.name == name)
            .map(|(_, item)| item.clone())
            .collect()
    })
}

// Get Total Quantity of All Food Items
#[ic_cdk::query]
pub fn get_total_food_quantity() -> u32 {
    FOOD_STORAGE.with(|service| {
        let storage = service.borrow_mut();
        storage.iter().map(|(_, item)| item.quantity).sum()
    })
}

// To generate the Candid interface definitions for our canister

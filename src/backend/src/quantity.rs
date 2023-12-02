use ic_cdk::api::time;

use crate::{food_item_manager::FOOD_STORAGE, FoodItem};

// Function to retrieve food items with a quantity above a specified threshold
#[ic_cdk::query]
pub fn get_food_items_above_quantity(threshold: u32) -> Vec<FoodItem> {
    FOOD_STORAGE.with(|service| {
        let storage = service.borrow_mut();
        storage
            .iter()
            .filter(|(_, item)| item.quantity > threshold)
            .map(|(_, item)| item.clone())
            .collect()
    })
}

// Function to retrieve food items with a quantity below a specified threshold
#[ic_cdk::query]
pub fn get_food_items_below_quantity(threshold: u32) -> Vec<FoodItem> {
    FOOD_STORAGE.with(|service| {
        let storage = service.borrow_mut();
        storage
            .iter()
            .filter(|(_, item)| item.quantity < threshold)
            .map(|(_, item)| item.clone())
            .collect()
    })
}

// Function to get the average quantity of all food items
#[ic_cdk::query]
pub fn get_average_food_quantity() -> f64 {
    FOOD_STORAGE.with(|service| {
        let storage = service.borrow_mut();
        let total_items = storage.len() as f64;
        let total_quantity: f64 = storage.iter().map(|(_, item)| item.quantity as f64).sum();
        if total_items > 0.0 {
            total_quantity / total_items
        } else {
            0.0
        }
    })
}

// Function to clear expired food items from storage
#[ic_cdk::update]
pub fn clear_expired_food_items() {
    let current_timestamp = time();
    FOOD_STORAGE.with(|service| {
        let mut storage = service.borrow_mut();
        let expired_items: Vec<u64> = storage
            .iter()
            .filter(|(_, item)| current_timestamp > item.expiration_date)
            .map(|(id, _)| id)
            .collect();

        for id in expired_items {
            storage.remove(&id);
        }
    });
}


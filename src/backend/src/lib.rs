
mod expiration_status;
mod food_item;
mod quantity;
mod food_item_manager;
mod food_item_operations;
// Re-export the public API for the library
pub use expiration_status::{
    check_expiration_status, get_total_food_quantity, list_all_food_items,
    search_food_items_by_name,
};
pub use food_item::{FoodItem, FoodItemPayload};
pub use food_item_operations::{add_food_item, delete_food_item, get_food_item, update_food_item, };
pub use quantity::{clear_expired_food_items, get_average_food_quantity, get_food_items_above_quantity, get_food_items_below_quantity};


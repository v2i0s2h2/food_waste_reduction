#[macro_use]
extern crate serde;
use candid::{Decode, Encode};
use ic_cdk::api::time;
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, Cell, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;
// ... (existing imports and types)

use ic_cdk::api::time;

// Constants
const ONE_DAY_IN_MICROSECONDS: u64 = 86_400_000_000_000;


#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
struct FoodItem {
    id: u64,
    name: String,
    quantity: u32,
    created_date: u64,
    expiration_date: u64, // Timestamp for expiration date
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct FoodItemPayload {
    name: String,
    quantity: u32,
}

// Implementing Storable and BoundedStorable traits for FoodItem
impl Storable for FoodItem {
    fn to_bytes(&self) -> std::borrow::Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: std::borrow::Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FoodItem {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

// ... (existing thread-local variables and payload structure)

thread_local! {
    static FOOD_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    static FOOD_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(FOOD_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter for food items")
    );

    static FOOD_STORAGE: RefCell<StableBTreeMap<u64, FoodItem, Memory>> =
        RefCell::new(StableBTreeMap::init(
            FOOD_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Helper method to perform insert for FoodItem
fn do_insert_food_item(item: &FoodItem) {
    FOOD_STORAGE.with(|service| service.borrow_mut().insert(item.id, item.clone()));
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
struct ExcessFoodSharePayload {
    food_id: u64,
    quantity: u32,
}


// Managing Food Items
// In this section, we'll implement the core logic for managing food items within our canister.

// get_food_item Function:
#[ic_cdk::query]
fn get_food_item(id: u64) -> Result<FoodItem, Error> {
    match _get_food_item(&id) {
        Some(item) => Ok(item),
        None => Err(Error::NotFound {
            msg: format!("a food item with id={} not found", id),
        }),
    }
}

// _get_food_item Function:
fn _get_food_item(id: &u64) -> Option<FoodItem> {
    FOOD_STORAGE.with(|s| s.borrow().get(id))
}

// add_food_item Function:
#[ic_cdk::update]
fn add_food_item(item: FoodItemPayload) -> Result<FoodItem, Error> {
    let id = FOOD_ID_COUNTER
        .with(|counter| {
            let current_value = counter.borrow().get();
            counter.borrow_mut().set(current_value + 1)
        })
        .map_err(|_| Error::CounterIncrementError)?;

    let food_item = FoodItem {
        id,
        name: item.name,
        quantity: item.quantity,
        created_date: time(),
        expiration_date: time() + ONE_DAY_IN_MICROSECONDS,
    };
    do_insert_food_item(&food_item);
    Ok(food_item)
}

// update_food_item Function:
#[ic_cdk::update]
fn update_food_item(id: u64, item: FoodItemPayload) -> Result<FoodItem, Error> {
    match FOOD_STORAGE.with(|service| service.borrow_mut().get_mut(&id)) {
        Some(food_item) => {
            food_item.name = item.name;
            food_item.quantity = item.quantity;
            food_item.created_date = time();
            food_item.expiration_date = time() + ONE_DAY_IN_MICROSECONDS;
            do_insert_food_item(food_item);
            Ok(food_item.clone())
        }
        None => Err(Error::NotFound {
            msg: format!("couldn't update a food item with id={}. item not found", id),
        }),
    }
}

// delete_food_item Function:
#[ic_cdk::update]
fn delete_food_item(id: u64) -> Result<FoodItem, Error> {
    match FOOD_STORAGE.with(|service| service.borrow_mut().remove(&id)) {
        Some(food_item) => Ok(food_item),
        None => Err(Error::NotFound {
            msg: format!(
                "couldn't delete a food item with id={}. item not found.",
                id
            ),
        }),
    }
}

// enum Error:
#[derive(candid::CandidType, Deserialize, Serialize)]
enum Error {
    NotFound { msg: String },
    CounterIncrementError,
}


#[ic_cdk::query]
fn check_expiration_status(id: u64) -> Result<String, Error> {
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
fn list_all_food_items() -> Vec<FoodItem> {
    FOOD_STORAGE.with(|service| {
        let storage = service.borrow_mut();
        storage.iter().map(|(_, item)| item.clone()).collect()
    })
}

// Search Food Items by Name
#[ic_cdk::query]
fn search_food_items_by_name(name: String) -> Vec<FoodItem> {
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
fn get_total_food_quantity() -> u32 {
    FOOD_STORAGE.with(|service| {
        let storage = service.borrow_mut();
        storage.iter().map(|(_, item)| item.quantity).sum()
    })
}

// Function to retrieve food items with a quantity above a specified threshold
#[ic_cdk::query]
fn get_food_items_above_quantity(threshold: u32) -> Vec<FoodItem> {
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
fn get_food_items_below_quantity(threshold: u32) -> Vec<FoodItem> {
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
fn get_average_food_quantity() -> f64 {
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
fn clear_expired_food_items() {
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

// To generate the Candid interface definitions for our canister
ic_cdk::export_candid!();

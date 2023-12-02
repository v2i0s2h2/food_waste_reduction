use ic_stable_structures::{ Cell, DefaultMemoryImpl, StableBTreeMap};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory} ;


use std::{cell::RefCell, thread_local};
use crate::food_item::FoodItem;

type Memory = VirtualMemory<DefaultMemoryImpl>;
type IdCell = Cell<u64, Memory>;

thread_local! {
    static FOOD_MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(
        MemoryManager::init(DefaultMemoryImpl::default())
    );

    pub static FOOD_ID_COUNTER: RefCell<IdCell> = RefCell::new(
        IdCell::init(FOOD_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(0))), 0)
            .expect("Cannot create a counter for food items")
    );

    pub static FOOD_STORAGE: RefCell<StableBTreeMap<u64, FoodItem, Memory>> =
        RefCell::new(StableBTreeMap::init(
            FOOD_MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1)))
    ));
}

// Helper method to perform insert for FoodItem
 pub fn do_insert_food_item(item: &FoodItem) {
    FOOD_STORAGE.with(|service| service.borrow_mut().insert(item.id, item.clone()));
}

pub fn get_food_item_from_storage(id: u64) -> Option<FoodItem> {
    FOOD_STORAGE.with(|service| service.borrow().get(&id))
}

// Other methods for mana

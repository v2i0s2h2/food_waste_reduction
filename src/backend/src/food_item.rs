use candid::{Decode, Encode};

use ic_stable_structures::{BoundedStorable, Storable};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(candid::CandidType, Clone, Serialize, Deserialize, Default)]
pub struct FoodItem {
    pub id: u64,
    pub name: String,
    pub quantity: u32,
    pub created_date: u64,
    pub expiration_date: u64, // Timestamp for expiration date
}

// Implementing Storable and BoundedStorable traits for FoodItem
impl Storable for FoodItem {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for FoodItem {
    const MAX_SIZE: u32 = 1024;
    const IS_FIXED_SIZE: bool = false;
}

#[derive(candid::CandidType, Deserialize, Serialize)]
pub enum Error {
    NotFound { msg: String },
}

#[derive(candid::CandidType, Serialize, Deserialize, Default)]
pub struct FoodItemPayload {
    pub name: String,
    pub quantity: u32,
}


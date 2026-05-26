#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype,
    symbol_short, Address, Env, String,
};

#[contracttype]
#[derive(Clone)]
pub struct MedicineBatch {
    pub batch_id: String,
    pub medicine_name: String,
    pub manufacturer: String,
    pub manufacture_date: String,
    pub expiry_date: String,
    pub registered_by: Address,
    pub is_recalled: bool,
}

#[contracttype]
pub enum DataKey {
    Batch(String),
    BatchCount,
}

#[contract]
pub struct MedChain;

#[contractimpl]
impl MedChain {

    // REGISTER BATCH (Freighter signs this)
    pub fn register_batch(
        env: Env,
        registrar: Address,
        batch_id: String,
        medicine_name: String,
        manufacturer: String,
        manufacture_date: String,
        expiry_date: String,
    ) -> bool {

        // ONLY ONE AUTH CHECK (correct)
        registrar.require_auth();

        // prevent overwrite
        if env.storage().instance()
            .get::<DataKey, MedicineBatch>(&DataKey::Batch(batch_id.clone()))
            .is_some()
        {
            return false;
        }

        let batch = MedicineBatch {
            batch_id: batch_id.clone(),
            medicine_name,
            manufacturer,
            manufacture_date,
            expiry_date,
            registered_by: registrar.clone(),
            is_recalled: false,
        };

        env.storage().instance().set(&DataKey::Batch(batch_id.clone()), &batch);

        let count: u32 = env.storage()
            .instance()
            .get(&DataKey::BatchCount)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::BatchCount, &(count + 1));

        env.events().publish(
            (symbol_short!("REGISTER"), registrar),
            batch_id,
        );

        true
    }

    // VERIFY
    pub fn verify_batch(
        env: Env,
        batch_id: String,
    ) -> Option<MedicineBatch> {

        env.storage()
            .instance()
            .get(&DataKey::Batch(batch_id))
    }

    // RECALL
    pub fn recall_batch(
        env: Env,
        registrar: Address,
        batch_id: String,
    ) -> bool {

        registrar.require_auth();

        let mut batch = match env.storage()
            .instance()
            .get::<DataKey, MedicineBatch>(&DataKey::Batch(batch_id.clone()))
        {
            None => return false,
            Some(b) => b,
        };

        if batch.registered_by != registrar {
            return false;
        }

        batch.is_recalled = true;

        env.storage()
            .instance()
            .set(&DataKey::Batch(batch_id.clone()), &batch);

        env.events().publish(
            (symbol_short!("RECALL"), registrar),
            batch_id,
        );

        true
    }

    pub fn get_batch_count(env: Env) -> u32 {
        env.storage()
            .instance()
            .get(&DataKey::BatchCount)
            .unwrap_or(0)
    }
}
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
#[derive(Clone)]
pub enum DataKey {
    Batch(String),
    BatchCount,
    Manufacturer(Address),
}

#[contract]
pub struct MedChain;

#[contractimpl]
impl MedChain {

    // Register a new medicine batch
    pub fn register_batch(
        env: Env,
        registrar: Address,
        batch_id: String,
        medicine_name: String,
        manufacturer: String,
        manufacture_date: String,
        expiry_date: String,
    ) -> bool {

        registrar.require_auth();

        // Check if batch already exists
        let exists = env
            .storage()
            .instance()
            .get::<DataKey, MedicineBatch>(&DataKey::Batch(batch_id.clone()))
            .is_some();

        if exists {
            return false;
        }

        let batch = MedicineBatch {
            batch_id: batch_id.clone(),
            medicine_name: medicine_name.clone(),
            manufacturer: manufacturer.clone(),
            manufacture_date,
            expiry_date,
            registered_by: registrar.clone(),
            is_recalled: false,
        };

        env.storage()
            .instance()
            .set(&DataKey::Batch(batch_id.clone()), &batch);

        let count: u32 = env
            .storage()
            .instance()
            .get::<DataKey, u32>(&DataKey::BatchCount)
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::BatchCount, &(count + 1));

        env.events().publish(
            (symbol_short!("REGISTER"), registrar),
            (batch_id, medicine_name, manufacturer),
        );

        true
    }

    // Verify medicine batch
    pub fn verify_batch(
        env: Env,
        batch_id: String,
    ) -> Option<MedicineBatch> {

        env.storage()
            .instance()
            .get::<DataKey, MedicineBatch>(&DataKey::Batch(batch_id))
    }

    // Recall a medicine batch
    pub fn recall_batch(
        env: Env,
        registrar: Address,
        batch_id: String,
    ) -> bool {

        registrar.require_auth();

        let batch_opt = env
            .storage()
            .instance()
            .get::<DataKey, MedicineBatch>(&DataKey::Batch(batch_id.clone()));

        match batch_opt {
            None => false,

            Some(mut batch) => {
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
        }
    }

    // Get total batches
    pub fn get_batch_count(env: Env) -> u32 {

        env.storage()
            .instance()
            .get::<DataKey, u32>(&DataKey::BatchCount)
            .unwrap_or(0)
    }
}
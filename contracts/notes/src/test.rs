#[cfg(test)]
mod tests {
    use super::*;
    use soroban_sdk::{Env, String, Address};

    fn create_env() -> Env {
        Env::default()
    }

    #[test]
    fn test_register_batch() {
        let env = create_env();
        let contract = MedChain;

        let user = Address::generate(&env);

        let result = contract.register_batch(
            env.clone(),
            user.clone(),
            String::from_str(&env, "B001"),
            String::from_str(&env, "Paracetamol"),
            String::from_str(&env, "Pfizer"),
            String::from_str(&env, "2025-01-01"),
            String::from_str(&env, "2027-01-01"),
        );

        assert!(result);
    }

    #[test]
    fn test_verify_batch() {
        let env = create_env();
        let contract = MedChain;
        let user = Address::generate(&env);

        contract.register_batch(
            env.clone(),
            user.clone(),
            String::from_str(&env, "B002"),
            String::from_str(&env, "Ibuprofen"),
            String::from_str(&env, "Bayer"),
            String::from_str(&env, "2025-01-01"),
            String::from_str(&env, "2027-01-01"),
        );

        let batch = contract.verify_batch(
            env.clone(),
            String::from_str(&env, "B002"),
        );

        assert!(batch.is_some());
    }

    #[test]
    fn test_recall_batch() {
        let env = create_env();
        let contract = MedChain;
        let user = Address::generate(&env);

        contract.register_batch(
            env.clone(),
            user.clone(),
            String::from_str(&env, "B003"),
            String::from_str(&env, "Amoxicillin"),
            String::from_str(&env, "GSK"),
            String::from_str(&env, "2025-01-01"),
            String::from_str(&env, "2027-01-01"),
        );

        let result = contract.recall_batch(
            env.clone(),
            user.clone(),
            String::from_str(&env, "B003"),
        );

        assert!(result);
    }

    #[test]
    fn test_batch_count() {
        let env = create_env();
        let contract = MedChain;
        let user = Address::generate(&env);

        contract.register_batch(
            env.clone(),
            user.clone(),
            String::from_str(&env, "B004"),
            String::from_str(&env, "DrugA"),
            String::from_str(&env, "CompanyA"),
            String::from_str(&env, "2025-01-01"),
            String::from_str(&env, "2027-01-01"),
        );

        let count = contract.get_batch_count(env.clone());
        assert_eq!(count, 1);
    }

    #[test]
    fn test_duplicate_batch_fails() {
        let env = create_env();
        let contract = MedChain;
        let user = Address::generate(&env);

        let _ = contract.register_batch(
            env.clone(),
            user.clone(),
            String::from_str(&env, "B005"),
            String::from_str(&env, "DrugX"),
            String::from_str(&env, "CompanyX"),
            String::from_str(&env, "2025-01-01"),
            String::from_str(&env, "2027-01-01"),
        );

        let result = contract.register_batch(
            env.clone(),
            user.clone(),
            String::from_str(&env, "B005"),
            String::from_str(&env, "DrugX"),
            String::from_str(&env, "CompanyX"),
            String::from_str(&env, "2025-01-01"),
            String::from_str(&env, "2027-01-01"),
        );

        assert_eq!(result, false);
    }
}

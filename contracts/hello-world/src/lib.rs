#![no_std]

use soroban_sdk::{
    contract, contractimpl, log, map, symbol_short, Address, Env, Map, String, Symbol,
};

#[contract]
pub struct KudoBankContract;

const KUDOS: Symbol = symbol_short!("KUDOS");

#[contractimpl]
impl KudoBankContract {
    /// 🎁 Gửi 1 Kudo (lời khen) từ 'from' đến 'to'
    pub fn give_kudos(env: Env, from: Address, to: Address) {
        from.require_auth(); // Yêu cầu 'from' phải ký tên

        let mut kudo_map: Map<Address, u32> = env
            .storage()
            .persistent()
            .get(&KUDOS)
            .unwrap_or(Map::new(&env));

        let current_kudos = kudo_map.get(to.clone()).unwrap_or(0);
        let new_kudos = current_kudos + 1;
        kudo_map.set(to.clone(), new_kudos);

        env.storage().persistent().set(&KUDOS, &kudo_map);

        log!(&env, "Kudo given from {:?} to {:?}", from, to);
    }

    /// 🔵 Lấy tổng số Kudo của 1 user
    pub fn get_kudos(env: Env, user: Address) -> u32 {
        let kudo_map: Map<Address, u32> = env
            .storage()
            .persistent()
            .get(&KUDOS)
            .unwrap_or(Map::new(&env));

        kudo_map.get(user).unwrap_or(0)
    }
}

#[cfg(test)]
mod test {
    use super::*; // Import code contract ở trên
    use soroban_sdk::testutils::{Address as _, Logs}; // Import công cụ test
    use soroban_sdk::Env;

    #[test]
    fn test_kudo_bank() {
        // 1. Setup: Tạo môi trường ảo và 2 user
        let env = Env::default();
        let user_1 = Address::random(&env);
        let user_2 = Address::random(&env);

        // 2. Deploy: "Triển khai" contract trong môi trường test
        let contract_id = env.register_contract(None, KudoBankContract);
        let client = KudoBankContractClient::new(&env, &contract_id);

        // 3. Run: Gọi hàm 'give_kudos'
        // User 1 gửi kudo cho User 2
        // .set_source_account() giả lập 'user_1' ký tên
        env.as_contract(&contract_id, || {
            env.set_source_account(&user_1);
            client.give_kudos(&user_1, &user_2);
        });

        // 4. Assert: Kiểm tra kết quả
        // Check xem User 2 có 1 kudo không
        assert_eq!(client.get_kudos(&user_2), 1);

        // Check xem User 1 có 0 kudo không
        assert_eq!(client.get_kudos(&user_1), 0);

        // User 1 gửi thêm 1 kudo nữa cho User 2
        env.as_contract(&contract_id, || {
            env.set_source_account(&user_1);
            client.give_kudos(&user_1, &user_2);
        });

        // Check xem User 2 có 2 kudo không
        assert_eq!(client.get_kudos(&user_2), 2);

        // 5. Check Log (nếu cần)
        let log = env.logger().all().last().unwrap().clone();
        assert_eq!(
            log,
            "Kudo given from Address(user_1_address...) to Address(user_2_address...)"
                .to_string(&env)
        );
    }
}

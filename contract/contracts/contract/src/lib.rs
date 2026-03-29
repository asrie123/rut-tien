#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, symbol_short, Address, Env, Symbol};

// 1. Định nghĩa cấu trúc lưu trữ dữ liệu (State)
#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Balance(Address), // Lưu số dư của từng người dùng: Balance -> Address
}

// 2. Định nghĩa các hằng số Event (Sự kiện) để theo dõi giao dịch
const DEPOSIT: Symbol = symbol_short!("deposit");
const WITHDRAW: Symbol = symbol_short!("withdraw");

#[contract]
pub struct WalletContract;

#[contractimpl]
impl WalletContract {
    
    /// --- HÀM NẠP TIỀN (DEPOSIT) ---
    pub fn deposit(env: Env, user: Address, token_address: Address, amount: i128) {
        // Xác thực người dùng (Bắt buộc phải ký giao dịch)
        user.require_auth();

        if amount <= 0 {
            panic!("So luong nap phai lon hon 0");
        }

        // Khởi tạo client để tương tác với Token (ví dụ: XLM, USDC...)
        let token_client = token::Client::new(&env, &token_address);
        
        // Chuyển tiền từ ví người dùng vào Smart Contract
        token_client.transfer(&user, &env.current_contract_address(), &amount);

        // Cập nhật số dư trong bộ nhớ (Persistent Storage)
        let key = DataKey::Balance(user.clone());
        let mut balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);
        balance += amount;
        env.storage().persistent().set(&key, &balance);

        // Phát sự kiện để Frontend nhận biết
        env.events().publish((DEPOSIT, user), amount);
    }

    /// --- HÀM RÚT TIỀN (WITHDRAW) ---
    pub fn withdraw(env: Env, user: Address, token_address: Address, amount: i128) {
        // Xác thực người dùng
        user.require_auth();

        if amount <= 0 {
            panic!("So luong rut phai lon hon 0");
        }

        // Kiểm tra số dư hiện tại của người dùng trong Contract
        let key = DataKey::Balance(user.clone());
        let mut balance: i128 = env.storage().persistent().get(&key).unwrap_or(0);

        if balance < amount {
            panic!("So du khong du de rut");
        }

        // Cập nhật số dư mới (Trừ tiền trước khi thực hiện transfer để bảo mật)
        balance -= amount;
        env.storage().persistent().set(&key, &balance);

        // Thực hiện chuyển trả Token từ Contract về ví người dùng
        let token_client = token::Client::new(&env, &token_address);
        token_client.transfer(&env.current_contract_address(), &user, &amount);

        // Phát sự kiện rút tiền
        env.events().publish((WITHDRAW, user), amount);
    }

    /// --- TRUY VẤN: Lấy số dư của một người dùng ---
    pub fn get_user_balance(env: Env, user: Address) -> i128 {
        let key = DataKey::Balance(user);
        env.storage().persistent().get(&key).unwrap_or(0)
    }
    
    /// --- TRUY VẤN: Lấy tổng số tiền Contract đang giữ ---
    pub fn get_contract_total_balance(env: Env, token_address: Address) -> i128 {
        let token_client = token::Client::new(&env, &token_address);
        token_client.balance(&env.current_contract_address())
    }
}
stellar contract invoke \
  --id CCSACJN7IQY5P6JMM35WW7KD35QKARTLOK2QWUWFCNLFDLZ4Y2Y6UQN7 \
  --source-account student \
  --network testnet \
  -- \
  deposit --user student --token_address CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC --amount 200000000

  -- \
  withdraw \
  --user student \
  --token_address CDLZFC3SYJYDZT7K67VZ75HPJVIEUVNIXF47ZG2FB2RMQQVU2HHGCYSC \
  --amount 100000000
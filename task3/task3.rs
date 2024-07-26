trait Account {
    fn deposit(&mut self, amount: u32);
    fn withdraw(&mut self, amount: u32);
    fn balance(&self) -> u32;
}

struct BankAccount {
    account_number: u32,
    holder_name: String,
    balance: u32,
}

impl Account for BankAccount {
    fn deposit(&mut self, amount: u32) {
        self.balance += amount;
    }

    fn withdraw(&mut self, amount: u32) {
        if amount > self.balance {
            println!("Insufficient funds for withdrawal.");
        } else {
            self.balance -= amount;
        }
    }

    fn balance(&self) -> u32 {
        self.balance
    }
}

fn main() {
    let account1 = BankAccount {
        account_number: 12345,
        holder_name: "alice".to_string(),
        balance: 100,
    };
    let account2 = BankAccount {
        account_number: 54321,
        holder_name: "bob".to_string(),
        balance: 200,
    };

    account1.deposit(50);
    account2.withdraw(100);

    println!("alice's balance: {}", account1.balance());
    println!("bob's balance: {}", account2.balance());
}
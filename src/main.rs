trait Account {
    fn deposit(&mut self, amount: u64);
    fn withdraw(&mut self, amount: u64);
    fn balance(&self) -> u64;
}

struct BankAccount {
    account_number: u64,
    holder_name: String,
    balance: u64,
}
impl Account for BankAccount {
    fn deposit(&mut self, amount: u64) {
        self.balance += amount;
    }
    fn withdraw(&mut self, amount: u64) {
        if amount <= self.balance {
            self.balance -= amount;
        }
    }
    fn balance(&self) -> u64 {
        self.balance
    }
}
fn main() {
    let mut first_account = BankAccount {
        account_number: 123456,
        holder_name: String::from("John Doe"),
        balance: 100,
    };
    let mut second_account = BankAccount {
        account_number: 654321,
        holder_name: String::from("Jane Doe"),
        balance: 200,
    };
    first_account.deposit(20);
    second_account.withdraw(50);
    println!("First account balance: {}", first_account.balance());
    println!("Second account balance: {}", second_account.balance());
}

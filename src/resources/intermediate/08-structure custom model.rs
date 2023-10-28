struct BankAccount {
    balance: i32,
    verified: bool,
    owner: String,
}

fn main() {
    let my_account = BankAccount {
        balance: 100,
        verified: true,
        owner: String::from("John Doe"),
    };

    println!("Account balance: {:?}", my_account.balance);
    println!("Account verified: {:?}", my_account.verified);
    println!("Account owner: {:?}", my_account.owner);
}

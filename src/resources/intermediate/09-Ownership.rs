struct BankAccount {
    balance: i32,
    verified: bool,
    owner: String,
}

fn print_balance(account: &BankAccount) {
    println!("The balance is: {:?}", account.balance);
}

fn print_owner(account: &BankAccount) {
    println!("The owner is: {:?}", account.owner);
}

fn print_verified(account: &BankAccount) {
    println!("The verified status is: {:?}", account.verified);
}

fn main() {
    let my_account = BankAccount {
        balance: 100,
        verified: true,
        owner: String::from("John Doe"),
    };

    print_balance(&my_account);

    // in this point, rust clean all the data, to prevent memory leaks
    // the data is dropped

    //You need add the & to pass the reference and preserve the data
    print_owner(&my_account);
    print_verified(&my_account);
}

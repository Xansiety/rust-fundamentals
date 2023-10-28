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

fn is_verified(account: &BankAccount) -> Result<bool, bool> {
    return match account.verified {
        true => Ok(true),
        false => Err(false),
    };
}

fn main() {
    let my_account = BankAccount {
        balance: 100,
        verified: false,
        owner: String::from("John Doe"),
    };

    // if not is unwrapped, it will return a Result type Ok(true) or Err(false)
    // but unwrapping cant return a error, so it will panic
    // for that reason, you need use expect
    // let verification_status = is_verified(&my_account).unwrap();
    let verification_status = is_verified(&my_account).expect(
        "The account is not verified, please verify the account to continue",
    );

    print_balance(&my_account);
    print_owner(&my_account);
    print_verified(&my_account);
    println!("Is verified? {:?}", verification_status)
}

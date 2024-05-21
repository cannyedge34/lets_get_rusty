/// A savings account

pub struct SavingsAccount {
    balance: i32
}

impl SavingsAccount {
    /// Creates a `SavingsAccount` with a balance of 0
    ///
    /// # Examples
    ///
    /// ```
    /// use bank::SavingsAccount;
    /// let account = SavingsAccount::new();
    /// assert_eq!(account.get_balance(), 0);
    /// ```
    pub fn new() -> SavingsAccount {
        SavingsAccount {
            balance: 0
        }
    }

    pub fn get_balance(&self) -> i32 { self.balance }

    pub fn deposit(&mut self, amount: i32) {
        if amount < 0 {
            /*
                this is not a good practice
                in a real program we return a Result type instead of panic
                because panic it would crash the program
            */
            panic!("Can not deposit a negative amount!")
        }
        self.balance += amount
    }

    pub fn transfer(&self, acc_number: u32, amount: i32) -> Result<String, String> {
        Ok(format!("Transferred ${amount} to {acc_number}"))
    }
}


#[cfg(test)]
mod tests {
    // we import all the items from the parent module
    use super::*;

    #[test]
    fn it_has_a_starting_balance_of_zero() {
        let account = SavingsAccount::new();

        assert_eq!(account.get_balance(), 0);
    }

    #[test]
    fn it_is_able_to_deposit() {
        let mut account = SavingsAccount::new();

        account.deposit(100);
        assert_eq!(account.get_balance(), 100, "Balance is expected to be 100!");
        /*
            assert_ne!(account.get_balance(), 0);
            assert!(account.get_balance() == 100);
        */
    }

    #[test]
    #[should_panic]
    fn should_panic_if_deposit_is_negative() {
        let mut account = SavingsAccount::new();

        account.deposit(-1);
    }

    #[test]
    fn it_transfers_money() -> Result<(), String> {
        let mut account = SavingsAccount::new();

        account.deposit(100);
        account.transfer(123456, 100)?; // question mark operator to propagate errors
        Ok(())
    }
}

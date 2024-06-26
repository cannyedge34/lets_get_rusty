use bank::SavingsAccount;

mod utils;

#[test]
fn it_has_a_starting_balance_of_zero() {
  utils::common_setup();

  let account = SavingsAccount::new();

  assert_eq!(account.get_balance(), 0);
}
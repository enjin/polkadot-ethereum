use super::*;
use crate::{Error, mock::*};
use sp_runtime::TokenError;
use sp_core::U256;
use frame_support::{assert_ok, assert_noop, traits::Currency};

fn last_event() -> mock::Event {
	frame_system::Pallet::<Test>::events().pop().expect("Event expected").event
}

#[test]
fn minting_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));

		assert_ok!(Assets::do_issue(0, &1, 100.into()));
		assert_eq!(Assets::balance(0, &1), 100.into());

		assert_ok!(Assets::do_issue(0, &1, 20.into()));
		assert_eq!(Assets::balance(0, &1), 120.into());

		assert_ok!(Assets::do_issue(0, &2, 100.into()));
		assert_eq!(Assets::balance(0, &2), 100.into());

		assert_eq!(Assets::supply(0), 220.into());
	});
}

#[test]
fn burning_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));
		assert_ok!(Assets::do_issue(0, &1, 100.into()));

		assert_ok!(Assets::do_burn(0, &1, 20.into()));
		assert_eq!(Assets::balance(0, &1), 80.into());

		assert_ok!(Assets::do_burn(0, &1, 20.into()));
		assert_eq!(Assets::balance(0, &1), 60.into());

		assert_eq!(Assets::supply(0), 60.into());
	});
}

#[test]
fn transfers_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));
		assert_ok!(Assets::do_issue(0, &1, 100.into()));

		assert_ok!(Assets::do_transfer(0, &1, &2, 50.into()));
		assert_eq!(Assets::balance(0, &1), 50.into());
		assert_eq!(Assets::balance(0, &2), 50.into());
		assert_eq!(Assets::supply(0), 100.into());
	});
}


#[test]
fn transferring_amount_more_than_available_balance_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));
		assert_ok!(Assets::do_issue(0, &1, 100.into()));

		assert_noop!(Assets::do_transfer(0, &1, &2, 1000), Error::<Test>::BalanceLow);
		assert_noop!(Assets::transfer(Origin::signed(2), 0, 1, 51), Error::<Test>::BalanceLow);
	});
}


#[test]
fn account_lifecycle_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));

		assert_ok!(Assets::do_issue(0, &1, 100.into()));
		assert_ok!(Assets::do_issue(0, &2, 100.into()));
		assert_ok!(Assets::do_issue(0, &3, 100.into()));
		assert_eq!(Asset::<Test>::get(0).unwrap().accounts, 3);
		assert_eq!(Account::<Test>::iter_prefix(0).count(), 3);

		assert_ok!(Assets::do_transfer(0, &1, &3, 100.into()));
		assert_ok!(Assets::do_transfer(0, &2, &3, 100.into()));
		assert_eq!(Asset::<Test>::get(0).unwrap().accounts, 1);
		assert_eq!(Account::<Test>::iter_prefix(0).count(), 1);

		assert_ok!(Assets::do_transfer(0, &3, &4, 100.into()));
		assert_eq!(Asset::<Test>::get(0).unwrap().accounts, 2);
		assert_eq!(Account::<Test>::iter_prefix(0).count(), 2);

		assert_ok!(Assets::do_burn(0, &3, 200.into()));
		assert_ok!(Assets::do_burn(0, &4, 100.into()));
		assert_eq!(Asset::<Test>::get(0).unwrap().accounts, 0);
		assert_eq!(Account::<Test>::iter_prefix(0).count(), 0);
	});
}

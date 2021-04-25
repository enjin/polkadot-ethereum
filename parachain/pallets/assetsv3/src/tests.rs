use super::*;
use crate::{Error, mock::*};
use sp_runtime::TokenError;
use sp_core::U256;
use frame_support::{
	assert_ok, assert_noop,
	dispatch::DispatchResult,
};

fn last_event() -> mock::Event {
	frame_system::Pallet::<Test>::events().pop().expect("Event expected").event
}

#[test]
fn create_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));
		assert!(Asset::<Test>::contains_key(0));

		assert_noop!(Assets::do_create(0), Error::<Test>::InUse);
	});
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
fn minting_with_unknown_asset_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_noop!(Assets::do_issue(0, &1, 100.into()), TokenError::UnknownAsset);
	});
}

#[test]
fn minting_too_much_should_not_work() {
	new_test_ext().execute_with(|| {
		// test overflow of total supply
		assert_ok!(Assets::do_create(0));
		assert_ok!(Assets::do_issue(0, &1, U256::MAX));
		assert_noop!(Assets::do_issue(0, &2, 1.into()), TokenError::Overflow);

		// test overflow of account balance
		assert_ok!(Assets::do_create(1));
		assert_ok!(Assets::do_issue(1, &1, U256::MAX));
		Asset::<Test>::try_mutate(1, |maybe_details| -> DispatchResult {
			let details = maybe_details.as_mut().unwrap();
			details.supply = U256::zero();
			Ok(())
		}).unwrap();
		assert_eq!(Assets::supply(1), U256::zero());
		assert_noop!(Assets::do_issue(1, &1, 1.into()), TokenError::Overflow);
	});
}

#[test]
fn minting_zero_amount_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));
		assert_ok!(Assets::do_issue(0, &1, 0.into()));
		assert_eq!(Assets::supply(0), U256::zero());
		assert_eq!(Assets::balance(0, &1), U256::zero());
		assert_eq!(
			last_event(),
			mock::Event::assets(crate::Event::Issued(0, 1, U256::zero())),
		);
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
fn burning_with_unknown_asset_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_noop!(Assets::do_burn(0, &1, 100.into()), TokenError::UnknownAsset);
	});
}

#[test]
fn burning_too_much_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));
		assert_ok!(Assets::do_issue(0, &1, 1000.into()));
		assert_noop!(Assets::do_burn(0, &1, 1001.into()), TokenError::Underflow);

		assert_ok!(Assets::do_issue(0, &2, 100.into()));
		assert_noop!(Assets::do_burn(0, &2, 101.into()), TokenError::NoFunds);
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

		assert_eq!(
			last_event(),
			mock::Event::assets(crate::Event::Transferred(0, 1, 2, 50.into())),
		);
	});
}

#[test]
fn transfers_with_unknown_asset_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));
		assert_ok!(Assets::do_issue(0, &1, 1000.into()));
		assert_ok!(Assets::do_issue(0, &2, 100.into()));

		assert_noop!(Assets::do_transfer(0, &2, &3, 101.into()), TokenError::NoFunds);
	});
}


#[test]
fn transferring_amount_more_than_balance_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));
		assert_ok!(Assets::do_issue(0, &1, 1000.into()));
		assert_ok!(Assets::do_issue(0, &2, 100.into()));

		assert_noop!(Assets::do_transfer(0, &2, &3, 101.into()), TokenError::NoFunds);
	});
}

#[test]
fn transferring_amount_more_than_supply_should_not_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));
		assert_ok!(Assets::do_issue(0, &1, 100.into()));

		assert_noop!(Assets::do_transfer(0, &1, &2, 101.into()), TokenError::Underflow);
	});
}

#[test]
fn transferring_whole_balance_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));
		assert_ok!(Assets::do_issue(0, &1, 100.into()));

		assert_ok!(Assets::do_transfer(0, &1, &2, 100.into()));
	});
}

#[test]
fn transferring_zero_amount_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));
		assert_ok!(Assets::do_issue(0, &1, 100.into()));
		assert_ok!(Assets::do_transfer(0, &1, &2, 0.into()));
		assert_eq!(Assets::balance(0, &1), 100.into());
		assert_eq!(Assets::balance(0, &2), 0.into());
		assert_eq!(Assets::supply(0), 100.into());
		assert_eq!(
			last_event(),
			mock::Event::assets(crate::Event::Transferred(0, 1, 2, U256::zero())),
		);
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

#[test]
fn set_metadata_should_work() {
	new_test_ext().execute_with(|| {
		assert_ok!(Assets::do_create(0));

		assert_ok!(Assets::set_metadata(0, vec![0, 1, 2], vec![0, 1, 2], 18));
		assert!(Metadata::<Test>::contains_key(0));

		// cannot add oversized metadata
		assert_noop!(
			Assets::set_metadata(0, vec![0u8; 100], vec![0, 1, 2], 18),
			Error::<Test>::BadMetadata,
		);
	});
}

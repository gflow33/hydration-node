use crate::tests::mock::*;
use crate::types::PoolInfo;
use crate::{Error, Pools};
use frame_support::{assert_noop, assert_ok};
use sp_runtime::traits::BlockNumberProvider;
use sp_runtime::Permill;
use std::num::NonZeroU16;

#[test]
fn update_amplification_should_work_when_correct_params_are_provided() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;
	let pool_id: AssetId = 100;

	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, asset_a, 200 * ONE), (ALICE, asset_b, 200 * ONE)])
		.with_registered_asset("pool".as_bytes().to_vec(), pool_id)
		.with_registered_asset("one".as_bytes().to_vec(), asset_a)
		.with_registered_asset("two".as_bytes().to_vec(), asset_b)
		.build()
		.execute_with(|| {
			assert_ok!(Stableswap::create_pool(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				vec![asset_a, asset_b],
				100,
				Permill::from_percent(10),
				Permill::from_percent(20),
			));

			System::set_block_number(2);
			let b = System::current_block_number();
			dbg!(b);

			assert_ok!(Stableswap::update_amplification(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				1000,
				10,
				1000,
			));

			assert_eq!(
				<Pools<Test>>::get(pool_id).unwrap(),
				PoolInfo {
					assets: vec![asset_a, asset_b].try_into().unwrap(),
					initial_amplification: NonZeroU16::new(100).unwrap(),
					final_amplification: NonZeroU16::new(1000).unwrap(),
					initial_block: 10,
					final_block: 1000,
					trade_fee: Permill::from_percent(10),
					withdraw_fee: Permill::from_percent(20)
				}
			);
		});
}

#[test]
fn update_amplification_should_fail_when_end_block_is_before_current_block() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;
	let pool_id: AssetId = 100;

	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, asset_a, 200 * ONE), (ALICE, asset_b, 200 * ONE)])
		.with_registered_asset("pool".as_bytes().to_vec(), pool_id)
		.with_registered_asset("one".as_bytes().to_vec(), asset_a)
		.with_registered_asset("two".as_bytes().to_vec(), asset_b)
		.build()
		.execute_with(|| {
			assert_ok!(Stableswap::create_pool(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				vec![asset_a, asset_b],
				100,
				Permill::from_percent(10),
				Permill::from_percent(20),
			));

			System::set_block_number(5000);

			assert_noop!(
				Stableswap::update_amplification(RuntimeOrigin::signed(ALICE), pool_id, 1000, 10, 1000),
				Error::<Test>::InvalidBlock
			);
		});
}

#[test]
fn update_amplification_should_fail_when_end_block_is_smaller_than_start_block() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;
	let pool_id: AssetId = 100;

	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, asset_a, 200 * ONE), (ALICE, asset_b, 200 * ONE)])
		.with_registered_asset("pool".as_bytes().to_vec(), pool_id)
		.with_registered_asset("one".as_bytes().to_vec(), asset_a)
		.with_registered_asset("two".as_bytes().to_vec(), asset_b)
		.build()
		.execute_with(|| {
			assert_ok!(Stableswap::create_pool(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				vec![asset_a, asset_b],
				100,
				Permill::from_percent(10),
				Permill::from_percent(20),
			));

			System::set_block_number(5000);

			assert_noop!(
				Stableswap::update_amplification(RuntimeOrigin::signed(ALICE), pool_id, 1000, 20_000, 10_000),
				Error::<Test>::InvalidBlock
			);
		});
}

#[test]
fn update_amplification_should_fail_when_start_block_before_current_block() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;
	let pool_id: AssetId = 100;

	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, asset_a, 200 * ONE), (ALICE, asset_b, 200 * ONE)])
		.with_registered_asset("pool".as_bytes().to_vec(), pool_id)
		.with_registered_asset("one".as_bytes().to_vec(), asset_a)
		.with_registered_asset("two".as_bytes().to_vec(), asset_b)
		.build()
		.execute_with(|| {
			assert_ok!(Stableswap::create_pool(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				vec![asset_a, asset_b],
				100,
				Permill::from_percent(10),
				Permill::from_percent(20),
			));

			System::set_block_number(5000);

			assert_noop!(
				Stableswap::update_amplification(RuntimeOrigin::signed(ALICE), pool_id, 1000, 4000, 10_000),
				Error::<Test>::InvalidBlock
			);
		});
}

#[test]
fn update_amplification_should_work_when_current_change_has_not_completed() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;
	let pool_id: AssetId = 100;

	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, asset_a, 200 * ONE), (ALICE, asset_b, 200 * ONE)])
		.with_registered_asset("pool".as_bytes().to_vec(), pool_id)
		.with_registered_asset("one".as_bytes().to_vec(), asset_a)
		.with_registered_asset("two".as_bytes().to_vec(), asset_b)
		.build()
		.execute_with(|| {
			assert_ok!(Stableswap::create_pool(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				vec![asset_a, asset_b],
				100,
				Permill::from_percent(10),
				Permill::from_percent(20),
			));

			System::set_block_number(1);

			assert_ok!(Stableswap::update_amplification(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				1000,
				10,
				1000,
			));

			assert_eq!(
				<Pools<Test>>::get(pool_id).unwrap(),
				PoolInfo {
					assets: vec![asset_a, asset_b].try_into().unwrap(),
					initial_amplification: NonZeroU16::new(100).unwrap(),
					final_amplification: NonZeroU16::new(1000).unwrap(),
					initial_block: 10,
					final_block: 1000,
					trade_fee: Permill::from_percent(10),
					withdraw_fee: Permill::from_percent(20)
				}
			);
			System::set_block_number(500);
			assert_noop!(
				Stableswap::update_amplification(RuntimeOrigin::signed(ALICE), pool_id, 5000, 5010, 6000,),
				Error::<Test>::AmplificationChangeNotCompleted
			);

			System::set_block_number(5000);
			assert_ok!(Stableswap::update_amplification(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				5000,
				5010,
				6000,
			));

			assert_eq!(
				<Pools<Test>>::get(pool_id).unwrap(),
				PoolInfo {
					assets: vec![asset_a, asset_b].try_into().unwrap(),
					initial_amplification: NonZeroU16::new(1000).unwrap(),
					final_amplification: NonZeroU16::new(5000).unwrap(),
					initial_block: 5010,
					final_block: 6000,
					trade_fee: Permill::from_percent(10),
					withdraw_fee: Permill::from_percent(20)
				}
			);
		});
}

#[test]
fn update_amplification_should_fail_when_new_value_is_same_as_previous_one() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;
	let pool_id: AssetId = 100;

	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, asset_a, 200 * ONE), (ALICE, asset_b, 200 * ONE)])
		.with_registered_asset("pool".as_bytes().to_vec(), pool_id)
		.with_registered_asset("one".as_bytes().to_vec(), asset_a)
		.with_registered_asset("two".as_bytes().to_vec(), asset_b)
		.build()
		.execute_with(|| {
			assert_ok!(Stableswap::create_pool(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				vec![asset_a, asset_b],
				100,
				Permill::from_percent(10),
				Permill::from_percent(20),
			));

			System::set_block_number(5000);

			assert_noop!(
				Stableswap::update_amplification(RuntimeOrigin::signed(ALICE), pool_id, 100, 5000, 10_000),
				Error::<Test>::SameAmplification,
			);
		});
}

#[test]
fn update_amplification_should_fail_when_new_value_is_zero_or_outside_allowed_range() {
	let asset_a: AssetId = 1;
	let asset_b: AssetId = 2;
	let pool_id: AssetId = 100;

	ExtBuilder::default()
		.with_endowed_accounts(vec![(ALICE, asset_a, 200 * ONE), (ALICE, asset_b, 200 * ONE)])
		.with_registered_asset("pool".as_bytes().to_vec(), pool_id)
		.with_registered_asset("one".as_bytes().to_vec(), asset_a)
		.with_registered_asset("two".as_bytes().to_vec(), asset_b)
		.build()
		.execute_with(|| {
			assert_ok!(Stableswap::create_pool(
				RuntimeOrigin::signed(ALICE),
				pool_id,
				vec![asset_a, asset_b],
				100,
				Permill::from_percent(10),
				Permill::from_percent(20),
			));

			System::set_block_number(5000);

			assert_noop!(
				Stableswap::update_amplification(RuntimeOrigin::signed(ALICE), pool_id, 0, 5000, 10_000),
				Error::<Test>::InvalidAmplification,
			);

			assert_noop!(
				Stableswap::update_amplification(RuntimeOrigin::signed(ALICE), pool_id, 1, 5000, 10_000),
				Error::<Test>::InvalidAmplification,
			);

			assert_noop!(
				Stableswap::update_amplification(RuntimeOrigin::signed(ALICE), pool_id, 20_000, 5000, 10_000),
				Error::<Test>::InvalidAmplification,
			);
		});
}

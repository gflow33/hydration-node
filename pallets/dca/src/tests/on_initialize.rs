// This file is part of HydraDX.

// Copyright (C) 2020-2022  Intergalactic, Limited (GIB).
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use frame_support::traits::OnInitialize;
use std::io::empty;
use std::ops::RangeInclusive;

use crate::tests::*;
use crate::{assert_balance, AssetId, BlockNumber, Bond, Event, Order, Recurrence, Schedule, ScheduleId, Trade};
use frame_support::{assert_noop, assert_ok};
use frame_system::pallet_prelude::BlockNumberFor;
use orml_traits::MultiCurrency;
use orml_traits::MultiReservableCurrency;
use pretty_assertions::assert_eq;
use sp_runtime::traits::ConstU32;
use sp_runtime::DispatchError;
use sp_runtime::DispatchError::BadOrigin;
use sp_runtime::{BoundedVec, FixedU128};

#[test]
fn complete_buy_dca_schedule_should_be_executed_with_fixed_recurrence() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 10000 * ONE),
			(LP2, BTC, 5000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);

			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Fixed(5))
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Buy {
					asset_in: HDX,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));
			assert_balance!(ALICE, BTC, 0);
			assert_eq!(3000000, Currencies::reserved_balance(HDX.into(), &ALICE.into()));

			//Act
			proceed_to_blocknumber(501, 901);

			//Assert
			assert_balance!(ALICE, BTC, 5 * ONE);
			assert_eq!(0, Currencies::reserved_balance(HDX.into(), &ALICE.into()));
			assert!(DCA::bond(1).is_none());
		});
}

#[test]
fn complete_buy_dca_schedule_should_be_executed_with_fixed_recurrence_when_nonnative_currency_set_for_user() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, DAI, 10000 * ONE),
			(LP2, BTC, 5000 * ONE),
		])
		.with_fee_asset(vec![(ALICE, DAI)])
		.with_registered_asset(BTC)
		.with_registered_asset(DAI)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);

			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Fixed(5))
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Buy {
					asset_in: DAI,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));
			assert_balance!(ALICE, BTC, 0);
			assert_eq!(6000000, Currencies::reserved_balance(DAI.into(), &ALICE.into()));

			//Act
			proceed_to_blocknumber(501, 901);

			//Assert
			assert_balance!(ALICE, BTC, 5 * ONE);
			assert_eq!(0, Currencies::reserved_balance(DAI.into(), &ALICE.into()));
			assert!(DCA::bond(1).is_none());
		});
}

#[test]
fn buy_dca_schedule_should_be_ongoing_with_perpetual_recurrence() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 10000 * ONE),
			(LP2, BTC, 5000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);

			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Perpetual)
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Buy {
					asset_in: HDX,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));
			assert_balance!(ALICE, BTC, 0);
			assert_eq!(3000000, Currencies::reserved_balance(HDX.into(), &ALICE.into()));

			//Act
			proceed_to_blocknumber(501, 901);

			//Assert
			assert_balance!(ALICE, BTC, 5 * ONE);
			assert_eq!(3000000, Currencies::reserved_balance(HDX.into(), &ALICE.into()));
			assert!(DCA::bond(1).is_some());
		});
}

#[test]
fn complete_sell_dca_schedule_should_be_executed_with_fixed_recurrence() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 10000 * ONE),
			(LP2, BTC, 5000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);
			assert_balance!(ALICE, BTC, 0);

			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Fixed(5))
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Sell {
					asset_in: HDX,
					asset_out: BTC,
					amount_in: 1000 * ONE,
					min_limit: Balance::MIN,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Act
			proceed_to_blocknumber(501, 901);

			//Assert
			assert_balance!(ALICE, BTC, 1_438_848_920_863_307);
		});
}

#[test]
fn full_sell_dca_schedule_should_be_ongoing_with_perpetual_recurrence() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 100000 * ONE),
			(LP2, BTC, 5000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);
			assert_balance!(ALICE, BTC, 0);

			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Perpetual)
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Sell {
					asset_in: HDX,
					asset_out: BTC,
					amount_in: 1000 * ONE,
					min_limit: Balance::MIN,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Act
			proceed_to_blocknumber(501, 1501);

			//Assert
			assert_balance!(ALICE, BTC, 1_602_330_662_782_224);
		});
}

#[test]
fn nothing_should_happen_when_no_schedule_in_storage_for_block() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 10000 * ONE),
			(LP2, BTC, 5000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Act
			proceed_to_blocknumber(1, 500);

			//Assert
			assert_balance!(ALICE, BTC, 0);
			let schedule_id = 1;
			assert!(DCA::schedules(schedule_id).is_none());
		});
}

#[test]
fn schedule_is_executed_in_block_when_user_has_fixed_schedule_planned() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 10000 * ONE),
			(LP2, BTC, 5000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);

			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Fixed(5))
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Buy {
					asset_in: HDX,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Act
			set_to_blocknumber(501);

			//Assert
			assert_balance!(ALICE, BTC, ONE);
			let schedule_id = 1;
			assert_eq!(DCA::remaining_recurrences(schedule_id).unwrap(), 4);

			let scheduled_ids_for_next_planned_block = DCA::schedule_ids_per_block(601).unwrap();
			let expected_scheduled_ids_for_next_block = create_bounded_vec_with_schedule_ids(vec![1]);
			assert_eq!(
				scheduled_ids_for_next_planned_block,
				expected_scheduled_ids_for_next_block
			);
		});
}

#[test]
fn schedule_is_planned_with_period_when_block_has_already_planned_schedule() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 10000 * ONE),
			(LP2, BTC, 5000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Fixed(5))
				.with_period(ONE_HUNDRED_BLOCKS)
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::Some(601)));

			proceed_to_blocknumber(1, 500);
			let schedule_2 = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Perpetual)
				.with_period(ONE_HUNDRED_BLOCKS)
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule_2, Option::None));

			//Act
			set_to_blocknumber(501);

			//Assert
			let scheduled_ids_for_next_planned_block = DCA::schedule_ids_per_block(601).unwrap();
			let expected_scheduled_ids_for_next_block = create_bounded_vec_with_schedule_ids(vec![1, 2]);
			assert_eq!(
				scheduled_ids_for_next_planned_block,
				expected_scheduled_ids_for_next_block
			);
		});
}

#[test]
fn fixed_schedule_is_suspended_in_block_when_user_has_not_enough_balance() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 5000 * ONE),
			(LP2, BTC, 5000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);

			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Fixed(5))
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Buy {
					asset_in: DAI,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Act
			set_to_blocknumber(501);

			//Assert
			assert_balance!(ALICE, BTC, 0);
			let schedule_id = 1;
			assert_eq!(DCA::remaining_recurrences(schedule_id).unwrap(), 5);
			assert!(DCA::suspended(schedule_id).is_some());

			expect_events(vec![Event::Suspended {
				id: schedule_id,
				who: ALICE,
			}
			.into()]);
		});
}

//TODO: add similar case and impl for the case where bond is slashed, but we should not slash storage bond. So if there is price changes, etc in a foreign asset, then we should slash the leftover, so keeping storage bond
//TODO: also check the smae with non native
#[test]
fn user_bond_should_be_slashed_when_not_enough_balance_for_schedule_execution() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 5000 * ONE),
			(LP2, BTC, 5000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);

			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Fixed(5))
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Buy {
					asset_in: DAI,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Act
			set_to_blocknumber(501);

			//Assert
			assert!(DCA::bond(1).is_some());
			assert_eq!(
				DCA::bond(1).unwrap(),
				Bond {
					asset: HDX,
					amount: StorageBondInNativeCurrency::get()
				}
			);

			assert_eq!(
				StorageBondInNativeCurrency::get(),
				Currencies::reserved_balance(HDX.into(), &ALICE.into())
			);

			assert_balance!(TreasuryAccount::get(), HDX, ExecutionBondInNativeCurrency::get());
		});
}

#[test]
fn user_execution_bond_should_not_be_slashed_when_when_total_stored_bond_is_less_than_current_storage_bond() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, DAI, 5000 * ONE),
			(LP2, BTC, 10000 * ONE),
			(LP2, DAI, 10000 * ONE),
		])
		.with_fee_asset(vec![(ALICE, DAI)])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);

			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Fixed(5))
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Buy {
					asset_in: HDX,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Act
			assert_ok!(Tokens::transfer(
				Origin::signed(LP2),
				Omnipool::protocol_account(),
				DAI,
				4000 * ONE
			));
			let dai_in_treasury_before_schedule_execution = 498214558854;
			assert_balance!(TreasuryAccount::get(), DAI, dai_in_treasury_before_schedule_execution);

			set_to_blocknumber(501);

			//Assert
			let total_bond = 6000000;
			assert!(DCA::bond(1).is_some());
			assert_eq!(
				DCA::bond(1).unwrap(),
				Bond {
					asset: DAI,
					amount: total_bond
				}
			);

			assert_eq!(total_bond, Currencies::reserved_balance(DAI.into(), &ALICE.into()));

			assert_balance!(TreasuryAccount::get(), DAI, dai_in_treasury_before_schedule_execution);
		});
}

#[test]
fn user_execution_bond_should_not_be_slashed_fully_when_spot_price_changes_slightly() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, DAI, 5000 * ONE),
			(LP2, BTC, 10000 * ONE),
			(LP2, DAI, 10000 * ONE),
		])
		.with_fee_asset(vec![(ALICE, DAI)])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);

			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Fixed(5))
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Buy {
					asset_in: HDX,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Act
			assert_ok!(Tokens::transfer(
				Origin::signed(LP2),
				Omnipool::protocol_account(),
				DAI,
				200 * ONE
			));
			let dai_in_treasury = 498214558854;
			assert_balance!(TreasuryAccount::get(), DAI, dai_in_treasury);

			set_to_blocknumber(501);

			//Assert
			let total_bond = 6000000;
			let not_full_execution_bond = 1200000;

			assert!(DCA::bond(1).is_some());
			assert_eq!(
				DCA::bond(1).unwrap(),
				Bond {
					asset: DAI,
					amount: total_bond - not_full_execution_bond
				}
			);

			assert_eq!(
				total_bond - not_full_execution_bond,
				Currencies::reserved_balance(DAI.into(), &ALICE.into())
			);

			assert_balance!(TreasuryAccount::get(), DAI, dai_in_treasury + not_full_execution_bond);
		});
}

#[test]
fn user_execution_bond_should_not_be_slashed_with_native_when_storage_bond_config_is_greatly_increased() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 5000 * ONE),
			(LP2, BTC, 10000 * ONE),
			(LP2, DAI, 10000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);

			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Fixed(2))
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Buy {
					asset_in: DAI,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Act
			set_storage_bond_config(*OriginalStorageBondInNative * 10);

			assert_balance!(TreasuryAccount::get(), HDX, 0);

			set_to_blocknumber(501);

			//Assert
			let total_bond = 3000000;
			assert!(DCA::bond(1).is_some());
			assert_eq!(
				DCA::bond(1).unwrap(),
				Bond {
					asset: HDX,
					amount: total_bond
				}
			);

			assert_eq!(total_bond, Currencies::reserved_balance(HDX.into(), &ALICE.into()));

			assert_balance!(TreasuryAccount::get(), HDX, 0);
		});
}

#[test]
fn user_execution_bond_should_not_be_slashed_fullly_with_native_when_storage_bond_config_is_slightly_increased() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 5000 * ONE),
			(LP2, BTC, 10000 * ONE),
			(LP2, DAI, 10000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);

			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Fixed(2))
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Buy {
					asset_in: DAI,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Act
			set_storage_bond_config(*OriginalStorageBondInNative * 11 / 10);

			assert_balance!(TreasuryAccount::get(), HDX, 0);

			set_to_blocknumber(501);

			//Assert
			let total_bond = 3000000;
			let not_full_execution_bond = 800000;
			assert!(DCA::bond(1).is_some());
			assert_eq!(
				DCA::bond(1).unwrap(),
				Bond {
					asset: HDX,
					amount: total_bond - not_full_execution_bond
				}
			);

			assert_eq!(
				total_bond - not_full_execution_bond,
				Currencies::reserved_balance(HDX.into(), &ALICE.into())
			);

			assert_balance!(TreasuryAccount::get(), HDX, not_full_execution_bond);
		});
}

#[test]
fn perpetual_schedule_is_suspended_in_block_when_user_has_not_enough_balance() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 5000 * ONE),
			(LP2, BTC, 5000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);

			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Perpetual)
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Buy {
					asset_in: DAI,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Act
			set_to_blocknumber(501);

			//Assert
			assert_balance!(ALICE, BTC, 0);
			let schedule_id = 1;
			assert!(DCA::suspended(schedule_id).is_some());
		});
}

#[test]
fn schedule_is_executed_in_block_when_user_has_perpetual_schedule_planned() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 10000 * ONE),
			(LP2, BTC, 5000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);
			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Perpetual)
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Buy {
					asset_in: HDX,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Act
			set_to_blocknumber(501);

			//Assert
			assert_balance!(ALICE, BTC, ONE);
			let schedule_id = 1;
			assert!(DCA::remaining_recurrences(schedule_id).is_none());

			let scheduled_ids_for_next_planned_block = DCA::schedule_ids_per_block(601).unwrap();
			let expected_scheduled_ids_for_next_block = create_bounded_vec_with_schedule_ids(vec![1]);
			assert_eq!(
				scheduled_ids_for_next_planned_block,
				expected_scheduled_ids_for_next_block
			);
		});
}

#[test]
fn schedule_should_not_be_planned_again_when_there_is_no_more_recurrences() {
	ExtBuilder::default()
		.with_endowed_accounts(vec![
			(Omnipool::protocol_account(), DAI, 1000 * ONE),
			(Omnipool::protocol_account(), HDX, NATIVE_AMOUNT),
			(ALICE, HDX, 10000 * ONE),
			(LP2, BTC, 5000 * ONE),
		])
		.with_registered_asset(BTC)
		.with_token(BTC, FixedU128::from_float(0.65), LP2, 2000 * ONE)
		.with_initial_pool(FixedU128::from_float(0.5), FixedU128::from(1))
		.build()
		.execute_with(|| {
			//Arrange
			proceed_to_blocknumber(1, 500);
			let schedule = ScheduleBuilder::new()
				.with_recurrence(Recurrence::Fixed(1))
				.with_period(ONE_HUNDRED_BLOCKS)
				.with_order(Order::Buy {
					asset_in: HDX,
					asset_out: BTC,
					amount_out: ONE,
					max_limit: Balance::MAX,
					route: empty_vec(),
				})
				.build();

			assert_ok!(DCA::schedule(Origin::signed(ALICE), schedule, Option::None));

			//Act
			set_to_blocknumber(501);

			//Assert
			assert_balance!(ALICE, BTC, ONE);
			let schedule_id = 1;
			assert!(DCA::remaining_recurrences(schedule_id).is_none());

			assert!(
				DCA::schedule_ids_per_block(601).is_none(),
				"There should be no schedule for the block, but there is"
			);
		});
}

fn create_bounded_vec(trades: Vec<Trade>) -> BoundedVec<Trade, ConstU32<5>> {
	let bounded_vec: BoundedVec<Trade, sp_runtime::traits::ConstU32<5>> = trades.try_into().unwrap();
	bounded_vec
}

fn create_bounded_vec_with_schedule_ids(schedule_ids: Vec<ScheduleId>) -> BoundedVec<ScheduleId, ConstU32<5>> {
	let bounded_vec: BoundedVec<ScheduleId, sp_runtime::traits::ConstU32<5>> = schedule_ids.try_into().unwrap();
	bounded_vec
}

pub fn proceed_to_blocknumber(from: u64, to: u64) {
	for block_number in RangeInclusive::new(from, to) {
		System::set_block_number(block_number);
		DCA::on_initialize(block_number);
	}
}

pub fn set_to_blocknumber(to: u64) {
	System::set_block_number(to);
	DCA::on_initialize(to);
}

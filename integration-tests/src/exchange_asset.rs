#![cfg(test)]

use crate::polkadot_test_net::*;
use cumulus_primitives_core::ParaId;
use frame_support::weights::Weight;
use frame_support::{
	assert_ok,
	pallet_prelude::*,
	sp_runtime::{FixedU128, Permill},
	traits::Contains,
};
use hex_literal::hex;
use orml_traits::currency::MultiCurrency;
use polkadot_xcm::{latest::prelude::*, v3::WeightLimit, VersionedMultiAssets, VersionedXcm};
use pretty_assertions::assert_eq;
use sp_core::H256;
use sp_runtime::traits::{AccountIdConversion, BlakeTwo256, Hash};
use xcm_emulator::TestExt;

use frame_support::dispatch::GetDispatchInfo;

fn craft_exchange_asset_xcm<M: Into<MultiAssets>, RC: Decode + GetDispatchInfo>(give: M, want: M) -> VersionedXcm<RC> {
	use polkadot_runtime::xcm_config::BaseXcmWeight;
	use sp_runtime::traits::ConstU32;
	use xcm_builder::FixedWeightBounds;
	use xcm_executor::traits::WeightBounds;

	type Weigher<RC> = FixedWeightBounds<BaseXcmWeight, RC, ConstU32<100>>;

	let dest = MultiLocation::new(1, Parachain(HYDRA_PARA_ID));
	let beneficiary = Junction::AccountId32 { id: BOB, network: None }.into();
	let assets = give.into();
	let max_assets = assets.len() as u32;
	let context = GlobalConsensus(NetworkId::Polkadot).into();
	let fees = assets
		.get(0)
		.expect("should have at least 1 asset")
		.clone()
		.reanchored(&dest, context)
		.expect("should reanchor");
	let give: MultiAssetFilter = assets.clone().into();
	let want = want.into();
	let weight_limit = {
		let fees = fees.clone();
		let mut remote_message = Xcm(vec![
			ReserveAssetDeposited::<RC>(assets.clone()),
			ClearOrigin,
			BuyExecution {
				fees,
				weight_limit: Limited(Weight::zero()),
			},
			// ExchangeAsset {
			// 	give: give.clone(),
			// 	want: want.clone(),
			// 	maximal: true,
			// },
			DepositAsset {
				assets: Wild(AllCounted(max_assets)),
				beneficiary,
			},
		]);
		// use local weight for remote message and hope for the best.
		let remote_weight = Weigher::weight(&mut remote_message).expect("weighing should not fail");
		Limited(remote_weight)
	};

	let xcm = Xcm(vec![
		BuyExecution { fees, weight_limit },
		// ExchangeAsset {
		// 	give,
		// 	want,
		// 	maximal: true,
		// },
		DepositAsset {
			assets: Wild(AllCounted(max_assets)),
			beneficiary,
		},
	]);
	let message = Xcm(vec![
		SetFeesMode { jit_withdraw: true },
		TransferReserveAsset { assets, dest, xcm },
	]);
	VersionedXcm::V3(message)
}

#[test]
fn hydra_should_swap_assets_when_receiving_from_acala() {
	//Arrange
	TestNet::reset();

	dbg!("before hydra 1");
	Hydra::execute_with(|| {
		assert_ok!(hydradx_runtime::AssetRegistry::set_location(
			hydradx_runtime::RuntimeOrigin::root(),
			1,
			hydradx_runtime::AssetLocation(MultiLocation::parent())
		));
	});
	dbg!("after hydra 1");

	dbg!("before acala");
	Acala::execute_with(|| {
		dbg!("execute acala");
		let xcm = craft_exchange_asset_xcm::<_, hydradx_runtime::RuntimeCall>(
			MultiAsset::from((GeneralIndex(0), 100 * UNITS)),
			MultiAsset::from((Here, 300 * UNITS)),
		);
		//Act
		let res = hydradx_runtime::PolkadotXcm::execute(
			hydradx_runtime::RuntimeOrigin::signed(ALICE.into()),
			Box::new(xcm),
			Weight::MAX,
		);
		assert_ok!(res);

		//Assert
		assert_eq!(
			hydradx_runtime::Balances::free_balance(AccountId::from(ALICE)),
			ALICE_INITIAL_NATIVE_BALANCE_ON_OTHER_PARACHAIN - 100 * UNITS
		);
		//TODO: continue here, probably some other error with conversion, debug it
		// assert_eq!(
		// 	hydradx_runtime::Balances::free_balance(&ParaId::from(HYDRA_PARA_ID).into_account_truncating()),
		// 	100 * UNITS
		// );
		expect_hydra_events(vec![
			cumulus_pallet_xcmp_queue::Event::XcmpMessageSent {
				message_hash: Some([
					9, 0, 8, 33, 54, 22, 19, 20, 3, 152, 149, 31, 97, 142, 128, 167, 186, 128, 27, 162, 115, 18, 183,
					5, 49, 22, 165, 146, 39, 125, 144, 142,
				]),
			}
			.into(),
			pallet_xcm::Event::Attempted(Outcome::Complete(Weight::from_parts(200000000, 0))).into(),
		]);
		dbg!("end execute acala");
	});
	dbg!("after acala");

	let fees = 400641025641;
	dbg!("before hydra 2");
	Hydra::execute_with(|| {
		assert_eq!(
			hydradx_runtime::Tokens::free_balance(1, &AccountId::from(BOB)),
			BOB_INITIAL_NATIVE_BALANCE + 100 * UNITS - fees
		);
		assert_eq!(
			hydradx_runtime::Tokens::free_balance(1, &hydradx_runtime::Treasury::account_id()),
			fees
		);
	});
}

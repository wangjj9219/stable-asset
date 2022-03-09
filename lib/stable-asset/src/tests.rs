// This file is part of NUTS Finance.

// Copyright (C) 2017-2021 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::{mock::*, Error, StableAssetPoolInfo};
use frame_support::assert_noop;
use frame_support::assert_ok;
use frame_support::dispatch::DispatchError;
use frame_support::traits::fungibles::{Inspect, Mutate};

pub const BALANCE_OFF: u128 = 1;

fn last_event() -> Event {
	frame_system::pallet::Pallet::<Test>::events()
		.pop()
		.expect("Event expected")
		.event
}

fn create_pool() -> (i64, i64, i64, u64) {
	let coin0 = TestAssets::create_asset().expect("asset should be created");
	let coin1 = TestAssets::create_asset().expect("asset should be created");
	let pool_asset = TestAssets::create_asset().expect("asset should be created");
	let amount: Balance = 100_000_000;
	assert_ok!(TestAssets::mint_into(coin0, &1, amount));
	assert_ok!(TestAssets::mint_into(coin1, &1, amount));
	assert_ok!(StableAsset::create_pool(
		Origin::signed(1),
		pool_asset,
		vec![coin0, coin1],
		vec![10000000000u128, 10000000000u128],
		10000000u128,
		20000000u128,
		50000000u128,
		10000u128,
		2,
		1,
		1000000000000000000u128,
	));
	(coin0, coin1, pool_asset, 8319403528785522541u64)
}

#[test]
fn create_pool_successful() {
	new_test_ext().execute_with(|| {
		let balances = vec![100000000u128, 200000000u128];
		let a = 100u128;
		let current_d = StableAsset::get_d(&balances, a).unwrap();
		let input = 0;
		let output = 1;
		let amount = 1000345u128;
		let balances_new = vec![100000000u128, 200000000u128 - amount];
		let y = StableAsset::get_y(&balances_new, 0, current_d, a).unwrap();
		println!("y {:#?}", y);
		let dy = y - balances_new[input] - 1;

		let balances_two = vec![100000000u128 + dy, 200000000u128];
		let y_2 = StableAsset::get_y(&balances_two, 1, current_d, a).unwrap();
		println!("y2 {:#?} balances_new[output] {:#?}", y_2, balances_two[output]);
		let dy_2 = balances_two[output] - y_2 - 1;
		println!("current_d {:#?} dy {:#?} dy_2 {:#?}", current_d, dy, dy_2);
	});
}

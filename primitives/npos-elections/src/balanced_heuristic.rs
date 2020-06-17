// This file is part of Substrate.

// Copyright (C) 2020 Parity Technologies (UK) Ltd.
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

// TODO

#![allow(unused)]

use crate::{
	IdentifierT, ElectionResult, ExtendedBalance, setup_inputs, VoteWeight, Candidate, Assignment,
	Voter, CandidatePtr,
};
use sp_arithmetic::{PerThing, InnerOf};
use sp_std::{prelude::*, collections::btree_map::BTreeMap};

pub fn balanced_heuristic<AccountId: IdentifierT, P: PerThing>(
	to_elect: usize,
	initial_candidates: Vec<AccountId>,
	initial_voters: Vec<(AccountId, VoteWeight, Vec<AccountId>)>,
) -> Option<ElectionResult<AccountId, P>>
	where
		P: sp_std::ops::Mul<ExtendedBalance, Output = ExtendedBalance>,
		ExtendedBalance: From<InnerOf<P>>
{
	unimplemented!()
}

/// TODO:
pub(crate) fn calculate_max_score<AccountId: IdentifierT, P: PerThing>(
	candidates: &Vec<CandidatePtr<AccountId>>,
	voters: &Vec<Voter<AccountId>>,
) -> (Candidate<AccountId>, VoteWeight) {
	unimplemented!()
}

/// TODO:
pub(crate) fn apply_elected<AccountId: IdentifierT, P: PerThing>(
	elected: &Candidate<AccountId>,
	score: VoteWeight,
	assignments: &mut Vec<Assignment<AccountId, P>>,
) {
	unimplemented!()
}

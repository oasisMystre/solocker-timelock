// Copyright (c) 2021 Ivan Jelincic <parazyd@dyne.org>
//
// This file is part of streamflow-timelock
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License version 3
// as published by the Free Software Foundation.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program. If not, see <https://www.gnu.org/licenses/>.

//! streamflow-timelock is the code providing timelock primitives
//! used by [streamflow.finance](https://streamflow.finance).
#[cfg(feature = "anchor-support")]
use anchor_lang::prelude::*;
#[cfg(feature = "anchor-support")]
declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

/// Functions related to SPL tokens
pub mod associated_token;
/// Structs and data
pub mod state;
/// Utility functions
pub mod utils;

// Copyright (c) The Libra Core Contributors
// SPDX-License-Identifier: Apache-2.0

#![forbid(unsafe_code)]

//! The core Move VM logic.
//!
//! It is a design goal for the Move VM to be independent of the Libra blockchain, so that
//! other blockchains can use it as well. The VM isn't there yet, but hopefully will be there
//! soon.

#[macro_use]
extern crate mirai_annotations;

pub mod data_cache;
mod data_operations;
mod interpreter;
mod loader;
pub mod move_vm;
mod native_functions;
mod runtime;
pub mod session;
#[macro_use]
mod tracing;

#[cfg(test)]
mod unit_tests;

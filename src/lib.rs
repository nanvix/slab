// Copyright (c) The Maintainers of Nanvix.
// Licensed under the MIT license.

//==================================================================================================
// Configuration
//==================================================================================================

#![deny(clippy::all)]
#![cfg_attr(not(test), no_std)]
#![feature(ptr_sub_ptr)] // Slab::deallocate() uses this.

//==================================================================================================
// Modules
//==================================================================================================

mod slab;

//==================================================================================================
// Exports
//==================================================================================================

pub use slab::*;
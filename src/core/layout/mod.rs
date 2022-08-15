//! User definable window arrangements for a Workspace.
//!
//! Layouts are maintained per monitor and allow for independent management of the two parameters
//! (`max_main`, `ratio`) that are used to modify layout logic. Layout functions are only called
//! when there is a need to re-layout a given screen and will always be given a full list of
//! [Clients][1] that the [WindowManager][2] considers tiled. There are no restrictions as to
//! whether or not windows may overlap or that they provide a total covering of the available
//! screen space. Gaps and borders will be added to the [Regions][3] that are specified by layout
//! functions by eating into the regions specified, so there is no need to account for this when
//! writing a layout function.
//!
//! # Writing a simple layout function
//!
//! Lets start with a very basic layout that ignores the two parameters (`max_main` and `ratio`)
//! and instead, simply arranges the Clients it is given as evenly spaced rows:
//! ```
//! use penrose::core::{
//!     client::Client,
//!     data_types::{Change, Region, ResizeAction},
//!     xconnection::Xid,
//! };
//!
//! pub fn rows(
//!     clients: &[&Client],
//!     _focused: Option<Xid>,
//!     monitor_region: &Region,
//!     _max_main: u32,
//!     _ratio: f32,
//! ) -> Vec<ResizeAction> {
//!     monitor_region
//!         .as_rows(clients.len() as u32)
//!         .iter()
//!         .zip(clients)
//!         .map(|(r, c)| (c.id(), Some(*r)))
//!         .collect()
//! }
//! ```
//!
//! Here we are making use of the [as_rows][4] method on `Region` to split the region we are given
//! (the total available space on the current screen) into evenly sized rows. (There are a number of
//! utility methods on `Region` to aid in writing layout functions.) We then pair each client with
//! `Some(region)` to indicate that this is where the client should be placed by the
//! `WindowManager`. If we provide `None` for any of the clients, that client will then instead be
//! hidden.
//!
//! *Note, windows are positioned and mapped in order, meaning that later clients will overlap
//! those that have already been positioned if any of the Regions overlap one another.*
//!
//! This simple `rows` layout is a sub-set of the behaviour provided by the built in
//! [side_stack][5] layout (in effect, clamping `max_main` at 0).
//!
//! [1]: crate::core::client::Client
//! [2]: crate::core::manager::WindowManager
//! [3]: crate::core::data_types::Region
//! [4]: crate::core::data_types::Region::as_rows
//! [5]: crate::core::layout::side_stack

#[allow(clippy::module_inception)]
mod layout;
pub use layout::*;

/// Layout functions that are provided by the WindowManager.
pub mod layouts;

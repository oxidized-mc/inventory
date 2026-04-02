//! Inventory system: item stacks, data components, and container menus.
//!
//! This crate implements the core Minecraft inventory model including:
//! - [`ItemStack`] with [`DataComponentPatch`] (1.20.5+ format)
//! - [`MenuType`] enum for all container types
//! - [`item_ids`] item ID mapping backed by the vanilla item registry
//!
//! Higher-level constructs such as `PlayerInventory` (ECS component with
//! 41 slots and protocol slot mapping) remain in `oxidized-game`.

#![warn(missing_docs)]
#![deny(unsafe_code)]

pub mod container;
pub mod item_ids;
pub mod item_stack;

pub use container::{ContainerStateId, MenuType};
pub use item_ids::{item_id_to_name, item_name_to_id, max_stack_size_by_name};
pub use item_stack::{DataComponentPatch, ItemError, ItemId, ItemStack};

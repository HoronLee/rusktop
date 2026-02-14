//! Optional integrations with third-party libraries.

#[cfg(feature = "sea-orm-integration")]
pub mod sea_orm;

#[cfg(feature = "sea-orm-integration")]
pub use sea_orm::configure_sea_orm;

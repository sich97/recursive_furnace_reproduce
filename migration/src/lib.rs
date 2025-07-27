#![allow(elided_lifetimes_in_paths)]
#![allow(clippy::wildcard_imports)]
pub use sea_orm_migration::prelude::*;
mod m20220101_000001_users;

mod m20250727_124025_unit_categories;
mod m20250727_124405_unit_bases;
mod m20250727_124705_units;
mod m20250727_124953_global_materials;
mod m20250727_125201_global_recipes;
mod m20250727_125350_create_join_table_global_recipes_and_global_materials;
pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20220101_000001_users::Migration),
            Box::new(m20250727_124025_unit_categories::Migration),
            Box::new(m20250727_124405_unit_bases::Migration),
            Box::new(m20250727_124705_units::Migration),
            Box::new(m20250727_124953_global_materials::Migration),
            Box::new(m20250727_125201_global_recipes::Migration),
            Box::new(m20250727_125350_create_join_table_global_recipes_and_global_materials::Migration),
            // inject-above (do not remove this comment)
        ]
    }
}
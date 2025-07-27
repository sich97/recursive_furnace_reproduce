use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "units",
            &[
            
            ("id", ColType::PkAuto),
            
            ("multiplier_name_prefix", ColType::TextUniq),
            ("multiplier_symbol_prefix", ColType::TextNull),
            ],
            &[
            ("user", "created_by"),
            ("unit_base", "unit_base"),
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "units").await
    }
}

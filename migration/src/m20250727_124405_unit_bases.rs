use loco_rs::schema::*;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, m: &SchemaManager) -> Result<(), DbErr> {
        create_table(m, "unit_bases",
            &[
            
            ("id", ColType::PkAuto),
            
            ("name_base", ColType::TextUniq),
            ("name_plural", ColType::TextUniq),
            ("symbol", ColType::TextNull),
            ],
            &[
            ("user", "created_by"),
            ("unit_categorie", "unit_category"),
            ]
        ).await
    }

    async fn down(&self, m: &SchemaManager) -> Result<(), DbErr> {
        drop_table(m, "unit_bases").await
    }
}

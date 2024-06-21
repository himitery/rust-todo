use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Todo::Table)
                    .if_not_exists()
                    .col(ColumnDef::new(Todo::Id).uuid().not_null().primary_key().extra("DEFAULT gen_random_uuid()"))
                    .col(ColumnDef::new(Todo::CreatedAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP"))
                    .col(ColumnDef::new(Todo::UpdatedAt).timestamp_with_time_zone().not_null().extra("DEFAULT CURRENT_TIMESTAMP"))
                    .col(ColumnDef::new(Todo::Title).string().not_null())
                    .col(ColumnDef::new(Todo::Content).string().not_null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Todo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum Todo {
    Table,
    #[sea_orm(iden = "id")]
    Id,
    #[sea_orm(iden = "created_at")]
    CreatedAt,
    #[sea_orm(iden = "updated_at")]
    UpdatedAt,
    #[sea_orm(iden = "title")]
    Title,
    #[sea_orm(iden = "content")]
    Content,
}

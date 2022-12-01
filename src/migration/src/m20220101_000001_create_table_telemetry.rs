use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Telemetry::Table)
                    .if_not_exists()
                    .col(
                        ColumnDef::new(Telemetry::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(ColumnDef::new(Telemetry::RequestId).integer().not_null())
                    .col(ColumnDef::new(Telemetry::Code).string().not_null())
                    .col(ColumnDef::new(Telemetry::Key).string().not_null())
                    .col(ColumnDef::new(Telemetry::Value).string())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Telemetry::Table).to_owned())
            .await
    }
}

#[derive(Iden)]
enum Telemetry {
    Table,
    Id,
    RequestId,
    Code,
    Key,
    Value,
}

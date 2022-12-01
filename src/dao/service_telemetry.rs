use migration::DbErr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, QuerySelect, Set, TransactionTrait,
};

use crate::entity::telemetry;
use crate::{controller::controller_storage::KeyValue, DB_POOL};
pub async fn store_data_db(data: Vec<KeyValue>) -> bool {
    let db = DB_POOL.get().await;

    db.transaction::<_, bool, DbErr>(|txn| {
        Box::pin(async move {
            let max_opt = telemetry::Entity::find()
                .select_only()
                .column_as(telemetry::Column::RequestId.max(), "value")
                .into_model::<SingleNumeric>()
                .one(txn)
                .await
                .unwrap_or(Some(SingleNumeric { value: 0 }));

            let max = max_opt.unwrap().value + 1;

            for item in data {
                let active_model = telemetry::ActiveModel {
                    code: Set(item.code),
                    key: Set(item.key),
                    value: Set(item.value),
                    request_id: Set(max),
                    ..Default::default()
                };

                let result = active_model.insert(txn).await;
                if result.is_err() {
                    return Ok(false);
                }
            }
            return Ok(true);
        })
    })
    .await
    .unwrap_or(false)
}

#[derive(FromQueryResult)]
struct SingleNumeric {
    value: i32,
}

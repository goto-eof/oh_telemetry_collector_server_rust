use chrono::Utc;
use migration::DbErr;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, FromQueryResult, QuerySelect, Set, TransactionTrait,
};
use std::collections::HashMap;

use crate::{entity::telemetry, DB_POOL};
pub async fn store_data_db(data: HashMap<String, HashMap<String, Option<String>>>) -> bool {
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

            let date_time = Utc::now().naive_utc();

            for (code, key_value_map) in data.into_iter() {
                for (key, value) in key_value_map.into_iter() {
                    let active_model = telemetry::ActiveModel {
                        code: Set(code.to_owned()),
                        key: Set(key),
                        value: Set(value),
                        request_id: Set(max),
                        created_at: Set(date_time),
                        ..Default::default()
                    };

                    let result = active_model.insert(txn).await;
                    if result.is_err() {
                        return Ok(false);
                    }
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

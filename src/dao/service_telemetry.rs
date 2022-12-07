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
                        property: Set(key),
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

// #[derive(FromQueryResult, Debug)]
// struct MultipleKeyValue {
//     n_pcs: i32,
//     value: String,
// }

// pub async fn computers_needs_ram_upgrade() -> () {
//     let db = DB_POOL.get().await;
//     let gb1 = Expr::col(telemetry::Column::Property);
//     let gb1: IntoIterator = gb1.into();
//     let result = telemetry::Entity::find()
//         .select_only()
//         .column(telemetry::Column::Value)
//         .column_as(telemetry::Column::Value.count(), "n_pcs")
//         .filter(
//             Condition::any().add(
//                 telemetry::Column::RequestId.in_subquery(
//                     Query::select()
//                         .expr(telemetry::Column::RequestId.max())
//                         .from(telemetry::Entity)
//                         .and_where(telemetry::Column::Property.eq("hw_cpu_idientifier"))
//                         .add_group_by(gb1)
//                         .add_group_by(Expr::col(telemetry::Column::Value).into())
//                         .to_owned(),
//                 ),
//             ),
//         )
//         .group_by(telemetry::Column::Code)
//         .group_by(telemetry::Column::Property)
//         .group_by(telemetry::Column::Value)
//         .into_model::<MultipleKeyValue>()
//         .all(db)
//         .await;

//     println!("{:?}", result);
// }

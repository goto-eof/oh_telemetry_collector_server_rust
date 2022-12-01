use sea_orm::{entity, ActiveModelTrait, Set};

use crate::entity::telemetry;
use crate::{controller::controller_storage::KeyValue, DB_POOL};
pub async fn store_data_db(data: Vec<KeyValue>) -> bool {
    let db = DB_POOL.get().await;

    for item in data {
        let active_model = telemetry::ActiveModel {
            code: Set(item.code),
            key: Set(item.key),
            value: Set(item.value),
            request_id: Set(1),
            ..Default::default()
        };

        let result = active_model.insert(db).await;
        if result.is_err() {
            return false;
        }
    }
    return true;
}

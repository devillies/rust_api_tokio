use crate::{model,ItemDb,Result};
use warp::{http::StatusCode,reply,Reply};

pub async fn get_shopping_list_item(item_db: ItemDb) -> Result<impl Reply>{
let local_db = item_db.lock().await;
let local_db:Vec<model::ShoppingListItem> = local_db.values().cloned().collect();
Ok(reply::with_status(reply::json(&local_db),StatusCode::OK))

}
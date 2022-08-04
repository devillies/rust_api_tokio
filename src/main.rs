use std::{collections::HashMap,convert::Infallible,sync::Arc};
use tokio::sync::Mutex;
use warp::{Filter, Rejection};



mod model;
mod handler;


type ItemDb =   Arc<Mutex<HashMap<usize,model::ShoppingListItem>>>;
type Result<T> = std::result::Result<T,Rejection> ;



#[tokio::main]
async fn main() {
    let items_db:ItemDb = Arc::new(Mutex::new(HashMap::new()));
    let root = warp::path::end().map(|| "Welcome to my warp server");
    let shopping_list_items_route = warp::path("shopping_list_items").and(warp::get()).and(with_items_db(items_db.clone())).and_then(handler::get_shopping_list_item);
    
    let routes = root.or(shopping_list_items_route).with(warp::cors().allow_any_origin());

    
    warp::serve(routes).run(([127,0,0,1],5000)).await
}



fn with_items_db(item_db: ItemDb) -> impl Filter<Extract = (ItemDb,),Error = Infallible> + Clone {
    warp::any().map(move || item_db.clone())
}
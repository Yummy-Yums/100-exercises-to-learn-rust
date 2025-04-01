use outro_08::{TicketDraft, TicketId, TicketPatch, TicketStore};
use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::sync::Mutex;
use warp::{Filter, Rejection, Reply};
// Use your existing types


#[tokio::main]
async fn main() {
    let store:  Arc<Mutex<TicketStore>> = Arc::new(Mutex::new(TicketStore::new()));

    // Create routes
    let create_ticket = warp::path!("tickets")
        .and(warp::post())
        .and(json_body::<TicketDraft>())
        .and(with_store(store.clone()))
        .and_then(create_ticket_handler);

    let get_ticket = warp::path!("tickets" / u64)
        .and(warp::get())
        .and(with_store(store.clone()))
        .and_then(get_ticket_handler);

    let update_ticket = warp::path!("tickets" / u64)
        .and(warp::patch())
        .and(json_body::<TicketDraft>())
        .and(with_store(store.clone()))
        .and_then(update_ticket_handler);

    let routes = create_ticket
        .or(get_ticket)
        .or(update_ticket)
        .with(warp::cors().allow_any_origin());

    warp::serve(routes).run(([127, 0, 0, 1], 3030)).await;
}

// Handler implementations
async fn create_ticket_handler(
    draft: TicketDraft,
    store:  Arc<Mutex<TicketStore>>,
) -> Result<impl Reply, Rejection> {
    let mut store = store.lock().await;
    let id = store.add_ticket(draft);
    Ok(warp::reply::json(&id))
}

async fn get_ticket_handler(
    id: u64,
    store:  Arc<Mutex<TicketStore>>,
) -> Result<impl Reply, Rejection> {
    let store = store.lock().await;
    match store.get(TicketId(id)) {
        Some(ticket) => {
            let ticket = ticket.read().unwrap();
            Ok(warp::reply::json(&*ticket))
        }
        None => Err(warp::reject::not_found()),
    }
}

async fn update_ticket_handler(
    id: u64,
    patch: TicketPatch,
    store:  Arc<Mutex<TicketStore>>,
) -> Result<impl Reply, Rejection> {
    let mut store = store.lock().await;
    match store.get(TicketId(id)) {
        Some(ticket) => {
            let mut ticket = ticket.write().unwrap();
            if let Some(title) = patch.title {
                ticket.title = title;
            }
            if let Some(description) = patch.description {
                ticket.description = description;
            }
            if let Some(status) = patch.status {
                ticket.status = status;
            }
            Ok(warp::reply::json(&*ticket))
        }
        None => Err(warp::reject::not_found()),
    }
}


// Helper functions
fn with_store(store: Arc<Mutex<TicketStore>>) -> impl Filter<Extract = (TicketStore,), Error = std::convert::Infallible> + Clone {
    warp::any().map(move || store.clone())
}

fn json_body<T: serde::de::DeserializeOwned + Send>() -> impl Filter<Extract = (T,), Error = Rejection> + Clone {
    warp::body::content_length_limit(1024 * 16).and(warp::body::json())
}
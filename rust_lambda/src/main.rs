use lambda_runtime::Error;
use serde::{Serialize, Deserialize};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let h = lambda_runtime::handler_fn(handler);
    lambda_runtime::run(h).await?;

    Ok(())
}

#[derive(Deserialize)]
struct Event {
    first_name: String,
    last_name: String
}

#[derive(Serialize)]
struct Output {
    message: String,
    request_id: String
}

async fn handler(event: Event, context: lambda_runtime::Context) -> Result <Output, Error> {
    let message: String = format!("Good day {} {}", event.first_name, event.last_name);

    Ok(Output {
        message,
        request_id: context.request_id
    })
}

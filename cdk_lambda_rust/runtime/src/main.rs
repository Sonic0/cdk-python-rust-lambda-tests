use lambda_http::{service_fn, Error, IntoResponse, Request, RequestExt, Response};
use log::{LevelFilter};
use serde::Deserialize;
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
    SimpleLogger::new()
        .with_level(LevelFilter::Info)
        .init()
        .unwrap();

    let func = service_fn(handler);
    lambda_http::run(func).await?;
    Ok(())
}

#[derive(Deserialize, Debug)]
// #[serde(rename_all = "camelCase")]
struct EventBody {
    message: String,
    name: String,
}

async fn handler(event: Request) -> Result<impl IntoResponse, Error> {
    // Query string
    let query_strings = event.query_string_parameters();
    let param1 = query_strings
        .first("parameter1")
        .unwrap_or("parameter not found");
    let param2 = query_strings
        .first("parameter2")
        .unwrap_or("parameter not found");
    log::info!("Query-string parameter1 value -> {}", param1);
    log::info!("Query-string parameter2 value -> {}", param2);

    // Body
    let body: EventBody = match event.payload() {
        Ok(serialized_body) => serialized_body.unwrap_or(EventBody {
            message: "No message".to_string(),
            name: "Anon".to_string(),
        }),
        Err(err) => panic!("{}", err.to_string()),
    };
    log::info!("Input/default name -> {}", body.name);
    log::info!("Input/default message -> {}", body.message);

    let message_response = format!(
        "Hello, {}! Your message for us is: {}",
        body.name, body.message
    );
    log::info!("Response from the Lambda: {}", message_response);

    Ok(Response::builder()
        .status(200)
        .body(message_response)
        .expect("failed to render response"))
}

use lambda_runtime::{service_fn, LambdaEvent, Error};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use simple_logger::SimpleLogger;

#[tokio::main]
async fn main() -> Result<(), Error> {
  SimpleLogger::new()
      .with_level(LevelFilter::Info)
      .init()
      .unwrap();

  let func = service_fn(handler);
  lambda_runtime::run(func).await?;
  Ok(())
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct Event {
  message: Option<String>,
  first_name: Option<String>,
}

#[derive(Serialize)]
struct Output {
  response: String,
}

async fn handler(event: LambdaEvent<Event>) -> Result<Output, Error> {
  let (event, _context) = event.into_parts();
  let message = event.message.unwrap_or("world".to_string());
  log::info!("{}", message);
  let first_name = event.first_name.unwrap_or("Anonymous".to_string());
  log::debug!("{}", first_name);

  let message_response = format!("Hello, {}! Your name is {}", message, first_name);
  log::info!("{}", message_response);

  Ok(Output {
    response: message_response,
  })
}

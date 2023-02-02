use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::model::AttributeValue;
use aws_sdk_dynamodb::Client;
use lambda_runtime::{service_fn, Error, LambdaEvent};
use opentelemetry::global;
// use opentelemetry::{sdk::export::trace::stdout, trace::Tracer};
use opentelemetry_aws::trace::XrayPropagator;
// use opentelemetry_http::HeaderInjector;
use serde::Deserialize;
use serde_json::{json, Value};
use tracing::{
    *
    // debug, 
    // error, 
    // info, 
    // instrument, 
    // span, 
    // Level
};
use tracing_subscriber::{
    // layer::SubscriberExt,
    filter::EnvFilter,
    FmtSubscriber,
    //Registry,
};
use uuid::Uuid;

#[tokio::main]
async fn main() -> Result<(), Error> {
    global::set_text_map_propagator(XrayPropagator::default());
    //let tracer = stdout::new_pipeline().install_simple();
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(EnvFilter::from_default_env())
        .finish();
    tracing::subscriber::set_global_default(subscriber)
        .expect("setting default subscriber failed");
    
    //let mut req = hyper::Request::builder().uri("http://127.0.0.1:3000");
    //tracer.in_span("doing_work", |cx| {
    //    info!("Holla Holla!");
    //});

    let func = service_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

#[derive(Deserialize)]
struct User {
    first_name: String,
    last_name: String,
}

#[instrument]
async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
    let (event, _context) = event.into_parts();

    let uuid = Uuid::new_v4().to_string();
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    let body = base64::decode(event["body"].as_str().ok_or("invalid request body")?)?;
    let body = std::str::from_utf8(&body)?;
    info!("Got event body: {}", body);
    let user = serde_json::from_str::<User>(&body).map_err(|_e| "couldn't read user data from req body")?;

    // let first_name = event["first_name"].as_str().unwrap_or_default();
    // let last_name = event["last_name"].as_str().unwrap_or_default();

    //let first_name = event["first_name"].as_str().ok_or("invalid first_name provided")?;
    //let last_name = event["last_name"].as_str().ok_or("invalid last_name provided")?;

    let request = client
        .put_item()
        .table_name("users")
        .item("uid", AttributeValue::S(String::from(uuid)))
        .item("first_name", AttributeValue::S(user.first_name))
        .item("last_name", AttributeValue::S(user.last_name));

    request.send().await?;

    Ok(json!({"message": "Record written"}))
}

//async fn func(event: LambdaEvent<Value>) -> Result<Value, Error> {
//    let (event, _context) = event.into_parts();
//    let first_name = event["firstName"].as_str().unwrap_or("world");
//
//    Ok(json!({ "message": format!("Hello, {}!", first_name) }))
//}

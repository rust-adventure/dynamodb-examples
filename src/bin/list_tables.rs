use aws_sdk_dynamodb::{Client, Error};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let res = client.list_tables().send().await?;
    dbg!(res.table_names);
    Ok(())
}

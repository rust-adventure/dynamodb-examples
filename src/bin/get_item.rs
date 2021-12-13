use aws_sdk_dynamodb::{
    model::AttributeValue, Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let item = client
        .get_item()
        .table_name("my-table")
        .key(
            "name",
            AttributeValue::S("bulbasaur".to_string()),
        )
        .send()
        .await?;

    dbg!(item.item);
    Ok(())
}

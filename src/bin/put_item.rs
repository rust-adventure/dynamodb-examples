use aws_sdk_dynamodb::{
    model::AttributeValue, Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let request = client
        .put_item()
        .table_name("my-table")
        .item(
            "name",
            AttributeValue::S(String::from(
                "bulbasaur".to_string(),
            )),
        )
        .item(
            "pokemon_type",
            AttributeValue::S(String::from(
                "grass".to_string(),
            )),
        );

    request.send().await?;

    Ok(())
}

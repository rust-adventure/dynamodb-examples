use aws_sdk_dynamodb::{
    model::AttributeValue, Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let req = client
        .query()
        .table_name("my-other-table")
        .key_condition_expression(
            "id = :hashKey and createdAt > :rangeKey",
        )
        .expression_attribute_values(
            ":hashKey",
            AttributeValue::S("123".to_string()),
        )
        .expression_attribute_values(
            ":rangeKey",
            AttributeValue::N(20150101.to_string()),
        )
        .send()
        .await?;
    dbg!(req.items);
    Ok(())
}

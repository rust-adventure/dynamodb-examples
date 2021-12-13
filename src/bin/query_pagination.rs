use aws_sdk_dynamodb::{
    model::AttributeValue, Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let mut results = vec![];
    let mut exclusive_start_key = None;
    loop {
        let req = client
            .query()
            .table_name("my-table")
            .limit(10)
            .set_exclusive_start_key(exclusive_start_key)
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

        if let Some(items) = req.items {
            results.extend(items);
            match req.last_evaluated_key {
                Some(last_evaluated_key) => {
                    exclusive_start_key =
                        Some(last_evaluated_key.clone());
                }
                None => {
                    break;
                }
            }
        } else {
            break;
        }
    }
    dbg!(results);
    Ok(())
}

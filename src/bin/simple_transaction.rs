use aws_sdk_dynamodb::{
    model::{
        AttributeValue, Put, TransactWriteItem, Update,
    },
    Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let req = client
        .transact_write_items()
        .transact_items(
            TransactWriteItem::builder()
                .put(
                    Put::builder()
                        .item(
                            "id",
                            AttributeValue::S(
                                "1".to_string(),
                            ),
                        )
                        .item(
                            "count",
                            AttributeValue::N(
                                1.to_string(),
                            ),
                        )
                        .build(),
                )
                .build(),
        )
        .transact_items(
            TransactWriteItem::builder()
                .update(
                    Update::builder()
                        .condition_expression(
                            "#count > :zeroValue",
                        )
                        .expression_attribute_names(
                            "#count", "count",
                        )
                        .expression_attribute_values(
                            "value",
                            AttributeValue::N(
                                1.to_string(),
                            ),
                        )
                        .expression_attribute_values(
                            ":zeroValue",
                            AttributeValue::N(
                                0.to_string(),
                            ),
                        )
                        .key(
                            "id",
                            AttributeValue::S(
                                "123".to_string(),
                            ),
                        )
                        .table_name("ItemsTable")
                        .update_expression(
                            "SET #count = :count - :value",
                        )
                        .build(),
                )
                .build(),
        )
        .send()
        .await?;
    dbg!(req);
    Ok(())
}

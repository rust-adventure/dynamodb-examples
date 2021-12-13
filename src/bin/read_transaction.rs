use aws_sdk_dynamodb::{
    model::{AttributeValue, Get, TransactGetItem},
    Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let req = client
        .transact_get_items()
        .transact_items(
            TransactGetItem::builder()
                .get(
                    Get::builder()
                        .table_name("my-table")
                        .key(
                            "name",
                            AttributeValue::S(
                                "bulbasaur".to_string(),
                            ),
                        )
                        .build(),
                )
                .build(),
        )
        .transact_items(
            TransactGetItem::builder()
                .get(
                    Get::builder()
                        .table_name("my-table")
                        .key(
                            "name",
                            AttributeValue::S(
                                "charmander".to_string(),
                            ),
                        )
                        .build(),
                )
                .build(),
        )
        .send()
        .await?;
    dbg!(req.responses);
    Ok(())
}

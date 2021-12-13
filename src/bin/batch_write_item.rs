use std::collections::HashMap;

use aws_sdk_dynamodb::{
    model::{AttributeValue, PutRequest, WriteRequest},
    Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    client
        .batch_write_item()
        .request_items(
            "my-table",
            vec![
                WriteRequest::builder()
                    .put_request(
                        PutRequest::builder()
                            .set_item(Some(HashMap::from(
                                [
                                    (
                                        "name".to_string(),
                                        AttributeValue::S(
                                            "bulbasaur"
                                                .to_string(
                                                ),
                                        ),
                                    ),
                                    (
                                        "pokemon_type"
                                            .to_string(),
                                        AttributeValue::S(
                                            "grass"
                                                .to_string(
                                                ),
                                        ),
                                    ),
                                ],
                            )))
                            .build(),
                    )
                    .build(),
                WriteRequest::builder()
                    .put_request(
                        PutRequest::builder()
                            .set_item(Some(HashMap::from(
                                [
                                    (
                                        "name".to_string(),
                                        AttributeValue::S(
                                            "charmander"
                                                .to_string(
                                                ),
                                        ),
                                    ),
                                    (
                                        "pokemon_type"
                                            .to_string(),
                                        AttributeValue::S(
                                            "fire"
                                                .to_string(
                                                ),
                                        ),
                                    ),
                                ],
                            )))
                            .build(),
                    )
                    .build(),
            ],
        )
        .send()
        .await?;
    Ok(())
}

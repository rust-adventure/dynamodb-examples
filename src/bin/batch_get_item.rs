use std::collections::HashMap;

use aws_sdk_dynamodb::{
    model::{AttributeValue, KeysAndAttributes},
    Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let items = client
        .batch_get_item()
        .request_items(
            "my-table",
            KeysAndAttributes::builder()
                .keys(HashMap::from([(
                    "name".to_string(),
                    AttributeValue::S(
                        "bulbasaur".to_string(),
                    ),
                )]))
                .keys(HashMap::from([(
                    "name".to_string(),
                    AttributeValue::S(
                        "charmander".to_string(),
                    ),
                )]))
                .build(),
        )
        .send()
        .await?;

    dbg!(items);
    Ok(())
}

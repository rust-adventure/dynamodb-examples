use aws_sdk_dynamodb::{
    model::{
        AttributeDefinition, BillingMode, KeySchemaElement,
        KeyType, ScalarAttributeType,
    },
    Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let key = "name";
    let pk = AttributeDefinition::builder()
        .attribute_name(key)
        .attribute_type(ScalarAttributeType::S)
        .build();

    let ks = KeySchemaElement::builder()
        .attribute_name(key)
        .key_type(KeyType::Hash)
        .build();

    client
        .create_table()
        .table_name(String::from("my-table"))
        .key_schema(ks)
        .attribute_definitions(pk)
        .billing_mode(BillingMode::PayPerRequest)
        .send()
        .await?;

    Ok(())
}

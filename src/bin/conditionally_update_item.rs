use aws_sdk_dynamodb::{
    model::AttributeValue, Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);

    let request = client
        .update_item()
        .table_name("my-table")
        .key(
            "name",
            AttributeValue::S("joe".to_string()),
        )
        .update_expression("set firstName = :firstName")
        .expression_attribute_values(
            ":firstName",
            AttributeValue::S("John McNewname".to_string()),
        )
        .expression_attribute_values( ":company",
                    AttributeValue::S("Apple".to_string())
                ).condition_expression("attribute_not_exists(deletedAt) and company = :company");

    request.send().await?;
    Ok(())
}

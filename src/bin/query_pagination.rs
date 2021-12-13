use aws_sdk_dynamodb::{
    model::AttributeValue, Client, Error,
};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let mut results = vec![];
    loop {
        let req = client
        .query()
        .table_name("InfrastructureStack-CoursesTable3F79D98E-KX1JZVQZ2ANG")
        .limit(1)
        .set_exclusive_start_key(None)
        .key_condition_expression("pk = :pk and begins_with(sk, :course)")
        .expression_attribute_values(
            ":pk",
            AttributeValue::S("rust-adventure".to_string()),
        )
        .expression_attribute_values(":course", AttributeValue::S("course".to_string()))
        .send()
        .await?;
        dbg!(req);
        results.extend(req.items);
    }
    Ok(())
}

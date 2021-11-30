use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::{Client, Error, Region, error::{DescribeTableError, DescribeTableErrorKind}, model::{
        AttributeDefinition, BillingMode, KeySchemaElement, KeyType, ScalarAttributeType,
        TableDescription, TableStatus,
    }, output::DescribeTableOutput};
use tokio::time;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let region_provider = RegionProviderChain::default_provider()
        .or_else(Region::new("us-west-2"));
    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let key = "pk";
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

    let mut interval = time::interval(time::Duration::from_secs(2));

    loop {
        println!("Waiting for database {} to become available", "my-table");
        interval.tick().await;
        let resp = client.describe_table().table_name("my-table").send().await;
        match resp {
            Ok(DescribeTableOutput {
                table:
                    Some(TableDescription {
                        table_status: Some(TableStatus::Active),
                        ..
                    }),
                ..
            }) => {
                break;
            }
            Err(aws_sdk_dynamodb::SdkError::ServiceError {
                err:
                    DescribeTableError {
                        kind: DescribeTableErrorKind::ResourceNotFoundException(_),
                        ..
                    },
                raw: _,
            }) => {
                // keep looping, the database metadata may not be populated yet.
            }
            e => {
                // keep looping if it's not an error, otherwise return
                e?;
            }
        }
    }
    Ok(())
}

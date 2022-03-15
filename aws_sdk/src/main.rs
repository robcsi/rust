use rusoto_core::Region;
use rusoto_s3::{CreateBucketConfiguration, CreateBucketRequest, S3Client, S3};

#[tokio::main]
async fn main()
{
    let region: Region = Default::default();
    // println!("{:?}", region.clone());

    let s3_client = S3Client::new(region);
    let bucket_configuration = CreateBucketConfiguration
    {
        location_constraint: Some(String::from("eu-central-1")),
    };

    let bucket_request = CreateBucketRequest
    {
        bucket: "softingbucket".to_string(),
        create_bucket_configuration: Some(bucket_configuration),
        ..Default::default()
    };

    match s3_client.create_bucket(bucket_request).await
    {
        Ok(output) => match output.location
        {
            Some(bucket_location_value) =>
            {
                println!("Bucket location is: {:?}", bucket_location_value);
            }
            None => println!("Unable to create bucket!"),
        },
        Err(error) =>
        {
            println!("Error: {:?}", error);
        }
    }
}

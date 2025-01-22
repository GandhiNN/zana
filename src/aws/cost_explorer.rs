/*
Reference for Group By Metrics:
https://docs.rs/aws-sdk-costexplorer/latest/aws_sdk_costexplorer/operation/get_cost_and_usage/builders/struct.GetCostAndUsageFluentBuilder.html#method.group_by
*/

use aws_sdk_costexplorer::types::{
    DateInterval, Granularity, GroupDefinition, GroupDefinitionType,
};
use aws_sdk_costexplorer::{Client, Error};
use aws_types::SdkConfig;

pub struct CostExplorer {
    pub client: Client,
}

impl CostExplorer {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn get_cost_and_usage(
        &self,
        start: String,
        end: String,
        granularity: &str,
        metrics: Vec<String>,
        group_by_type: String,
        group_by_key: String,
    ) -> Result<(), Error> {
        // DateInterval is a non-exhaustive struct, we have to use the builder to create it
        let date_interval = DateInterval::builder()
            .start(start)
            .end(end)
            .build()
            .unwrap();
        // Granularity is a non-exhaustive enum, we have to use the builder to create it
        let granularity_input = match granularity {
            "DAILY" => Granularity::Daily,
            "HOURLY" => Granularity::Hourly,
            "MONTHLY" => Granularity::Monthly,
            _ => Granularity::try_parse("NewFeature").unwrap(),
        };
        // metrics is an Option<Vec<String>>, we have to use Some() to wrap it
        let metrics_input = Some(metrics);
        // Create a group_by criteria using the type and key
        // GroupDefinition is a non-exhaustive enum, we have to use matching
        let group_def = match group_by_type.as_str() {
            "DIMENSION" => GroupDefinition::builder()
                .set_key(Some(group_by_key))
                .set_type(Some(GroupDefinitionType::Dimension)),
            "TAG" => GroupDefinition::builder()
                .set_key(Some(group_by_key))
                .set_type(Some(GroupDefinitionType::Tag)),
            "COST CATEGORY" => GroupDefinition::builder()
                .set_key(Some(group_by_key))
                .set_type(Some(GroupDefinitionType::CostCategory)),
            _ => GroupDefinition::builder(), // Default value
        };
        let resp = self
            .client
            .get_cost_and_usage()
            .time_period(date_interval)
            .granularity(granularity_input)
            .set_metrics(metrics_input)
            .group_by(group_def.build())
            .send()
            .await;
        match resp {
            Ok(output) => Ok(println!("{:#?}", output)),
            Err(e) => Err(Into::into(e)),
        }
    }
}

use anyhow::Result;
use aws_sdk_bedrock::{Client, Error};
use aws_types::SdkConfig;
use serde::Serialize;
use tabled::Tabled;

pub struct Bedrock {
    pub client: Client,
}

#[derive(Tabled, Debug, Serialize)]
pub struct FoundationalModelsSummary {
    name: String,
    id: String,
    provider: String,
    input_modalities: String,
    output_modalities: String,
    response_streaming_supported: String,
    customizations_supported: String,
    inference_types_supported: String,
    lifecycle_status: String,
}

#[allow(irrefutable_let_patterns)]
impl Bedrock {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn list_foundational_models(&self) -> Result<Vec<FoundationalModelsSummary>, Error> {
        let fm = self.client.list_foundation_models().send().await;
        let mut fm_summaries: Vec<FoundationalModelsSummary> = Vec::new();
        if let fms = fm.unwrap().model_summaries() {
            for fm in fms {
                fm_summaries.push(FoundationalModelsSummary {
                    name: fm.model_name().unwrap().to_string(),
                    id: fm.model_id().to_string(),
                    provider: fm.provider_name().unwrap().to_string(),
                    input_modalities: fm
                        .input_modalities()
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(", "),
                    output_modalities: fm
                        .output_modalities()
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    response_streaming_supported: fm
                        .response_streaming_supported()
                        .unwrap_or(false)
                        .to_string(),
                    customizations_supported: fm
                        .customizations_supported()
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    inference_types_supported: fm
                        .inference_types_supported()
                        .iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(","),
                    lifecycle_status: fm.model_lifecycle().unwrap().status().to_string(),
                });
            }
        }
        Ok(fm_summaries)
    }
}

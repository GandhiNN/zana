use aws_sdk_bedrockruntime::{primitives::Blob, Client, Error};
use aws_types::SdkConfig;
use serde::Serialize;
use std::str;

pub struct BedrockRuntime {
    pub client: Client,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct PromptBody {
    input_text: String,
}

impl BedrockRuntime {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn invoke_prompt(&self, model: &str, prompt: &str) -> Result<(), Error> {
        let input_prompt = PromptBody {
            input_text: prompt.to_string(),
        };
        let result = self
            .client
            .invoke_model()
            .model_id(model)
            .content_type("application/json")
            .body(Blob::new(serde_json::to_string(&input_prompt).unwrap()))
            .send()
            .await
            .unwrap();

        let output = str::from_utf8(result.body().as_ref()).unwrap();
        println!("{:#?}", output);
        Ok(())
    }
}

use anyhow::Result;
use aws_sdk_sts::Client;
use aws_types::SdkConfig;
use std::fmt;

#[derive(Default, Debug)]
pub struct Account {
    user_id: String,
    account_id: String,
    arn: String,
}

impl fmt::Display for Account {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "\nUser ID: {}\nAccount ID: {}\nARN: {}\n",
            self.user_id, self.account_id, self.arn
        )
    }
}

pub struct STS {
    pub client: Client,
}

impl STS {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn get_account_id(&self) -> Result<Account> {
        let sts_output = self.client.get_caller_identity().send().await;
        let mut account = Account::default();
        sts_output
            .map(|output| {
                account.user_id = output.user_id.unwrap();
                account.account_id = output.account.unwrap();
                account.arn = output.arn.unwrap();
            })
            .unwrap();
        Ok(account)
    }
}

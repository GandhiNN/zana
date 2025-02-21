use anyhow::Result;
use aws_sdk_secretsmanager::Client;
use aws_types::SdkConfig;
use serde::Serialize;
use tabled::Tabled;

#[derive(Tabled, Default, Serialize, Debug)]
pub struct Secret {
    arn: String,
    name: String,
    description: String,
    kms_key_id: String,
    rotation_enabled: String,
    rotation_lambda_arn: String,
    last_rotated_date: String,
    last_changed_date: String,
    last_accessed_date: String,
    deleted_date: String,
    next_rotation_date: String,
    owning_service: String,
    created_date: String,
    primary_region: String,
}

pub struct SecretsManager {
    pub client: Client,
}

impl SecretsManager {
    pub fn new(config: SdkConfig) -> Self {
        let client = Client::new(&config);
        Self { client }
    }

    pub async fn list_secrets(&self) -> Result<Vec<Secret>> {
        let mut out = self.client.list_secrets().into_paginator().send();
        let mut secrets_entries: Vec<Secret> = Vec::new();
        while let Some(output) = out.next().await {
            match output {
                Ok(secrets) => {
                    for secret in secrets.secret_list() {
                        secrets_entries.push(Secret {
                            arn: secret.arn().unwrap_or_default().into(),
                            name: secret.name().unwrap_or_default().into(),
                            description: secret.description().unwrap_or_default().into(),
                            kms_key_id: secret.kms_key_id().unwrap_or_default().into(),
                            rotation_enabled: secret
                                .rotation_enabled()
                                .unwrap_or_default()
                                .to_string(),
                            rotation_lambda_arn: secret
                                .rotation_lambda_arn()
                                .unwrap_or_default()
                                .into(),
                            last_rotated_date: match secret.last_rotated_date() {
                                Some(date) => date.to_string(),
                                None => "N/A".to_string(),
                            },
                            last_changed_date: match secret.last_changed_date() {
                                Some(date) => date.to_string(),
                                None => "N/A".to_string(),
                            },
                            last_accessed_date: match secret.last_changed_date() {
                                Some(date) => date.to_string(),
                                None => "N/A".to_string(),
                            },
                            deleted_date: match secret.deleted_date() {
                                Some(date) => date.to_string(),
                                None => "N/A".to_string(),
                            },
                            next_rotation_date: match secret.next_rotation_date() {
                                Some(date) => date.to_string(),
                                None => "N/A".to_string(),
                            },
                            owning_service: secret.owning_service().unwrap_or_default().into(),
                            created_date: secret.created_date().unwrap().to_string(),
                            primary_region: secret.primary_region().unwrap_or_default().into(),
                        })
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
        Ok(secrets_entries)
    }
}

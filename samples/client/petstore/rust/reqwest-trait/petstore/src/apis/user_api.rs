/*
 * OpenAPI Petstore
 *
 * This is a sample server Petstore server. For this sample, you can use the api key `special-key` to test the authorization filters.
 *
 * The version of the OpenAPI document: 1.0.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use async_trait::async_trait;
#[cfg(feature = "mockall")]
use mockall::automock;
use reqwest;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration};

#[cfg_attr(feature = "mockall", automock)]
#[async_trait]
pub trait UserApi: Send + Sync {
    async fn create_user<'user>(&self, user: models::User) -> Result<(), Error<CreateUserError>>;
    async fn create_users_with_array_input<'user>(&self, user: Vec<models::User>) -> Result<(), Error<CreateUsersWithArrayInputError>>;
    async fn create_users_with_list_input<'user>(&self, user: Vec<models::User>) -> Result<(), Error<CreateUsersWithListInputError>>;
    async fn delete_user<'username>(&self, username: &'username str) -> Result<(), Error<DeleteUserError>>;
    async fn get_user_by_name<'username>(&self, username: &'username str) -> Result<models::User, Error<GetUserByNameError>>;
    async fn login_user<'username, 'password>(&self, username: &'username str, password: &'password str) -> Result<String, Error<LoginUserError>>;
    async fn logout_user<>(&self, ) -> Result<(), Error<LogoutUserError>>;
    async fn update_user<'username, 'user>(&self, username: &'username str, user: models::User) -> Result<(), Error<UpdateUserError>>;
}

pub struct UserApiClient {
    configuration: Arc<configuration::Configuration>
}

impl UserApiClient {
    pub fn new(configuration: Arc<configuration::Configuration>) -> Self {
        Self { configuration }
    }
}



#[async_trait]
impl UserApi for UserApiClient {
    /// This can only be done by the logged in user.
    async fn create_user<'user>(&self, user: models::User) -> Result<(), Error<CreateUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/user", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("api_key", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&user);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<CreateUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// 
    async fn create_users_with_array_input<'user>(&self, user: Vec<models::User>) -> Result<(), Error<CreateUsersWithArrayInputError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/user/createWithArray", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("api_key", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&user);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<CreateUsersWithArrayInputError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// 
    async fn create_users_with_list_input<'user>(&self, user: Vec<models::User>) -> Result<(), Error<CreateUsersWithListInputError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/user/createWithList", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::POST, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("api_key", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&user);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<CreateUsersWithListInputError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This can only be done by the logged in user.
    async fn delete_user<'username>(&self, username: &'username str) -> Result<(), Error<DeleteUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/user/{username}", local_var_configuration.base_path, username=crate::apis::urlencode(username));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::DELETE, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("api_key", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<DeleteUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// 
    async fn get_user_by_name<'username>(&self, username: &'username str) -> Result<models::User, Error<GetUserByNameError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/user/{username}", local_var_configuration.base_path, username=crate::apis::urlencode(username));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<GetUserByNameError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// 
    async fn login_user<'username, 'password>(&self, username: &'username str, password: &'password str) -> Result<String, Error<LoginUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/user/login", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        local_var_req_builder = local_var_req_builder.query(&[("username", &username.to_string())]);
        local_var_req_builder = local_var_req_builder.query(&[("password", &password.to_string())]);
        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            serde_json::from_str(&local_var_content).map_err(Error::from)
        } else {
            let local_var_entity: Option<LoginUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// 
    async fn logout_user<>(&self, ) -> Result<(), Error<LogoutUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/user/logout", local_var_configuration.base_path);
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::GET, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("api_key", local_var_value);
        };

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<LogoutUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

    /// This can only be done by the logged in user.
    async fn update_user<'username, 'user>(&self, username: &'username str, user: models::User) -> Result<(), Error<UpdateUserError>> {
        let local_var_configuration = &self.configuration;

        let local_var_client = &local_var_configuration.client;

        let local_var_uri_str = format!("{}/user/{username}", local_var_configuration.base_path, username=crate::apis::urlencode(username));
        let mut local_var_req_builder = local_var_client.request(reqwest::Method::PUT, local_var_uri_str.as_str());

        if let Some(ref local_var_user_agent) = local_var_configuration.user_agent {
            local_var_req_builder = local_var_req_builder.header(reqwest::header::USER_AGENT, local_var_user_agent.clone());
        }
        if let Some(ref local_var_apikey) = local_var_configuration.api_key {
            let local_var_key = local_var_apikey.key.clone();
            let local_var_value = match local_var_apikey.prefix {
                Some(ref local_var_prefix) => format!("{} {}", local_var_prefix, local_var_key),
                None => local_var_key,
            };
            local_var_req_builder = local_var_req_builder.header("api_key", local_var_value);
        };
        local_var_req_builder = local_var_req_builder.json(&user);

        let local_var_req = local_var_req_builder.build()?;
        let local_var_resp = local_var_client.execute(local_var_req).await?;

        let local_var_status = local_var_resp.status();
        let local_var_content = local_var_resp.text().await?;

        if !local_var_status.is_client_error() && !local_var_status.is_server_error() {
            Ok(())
        } else {
            let local_var_entity: Option<UpdateUserError> = serde_json::from_str(&local_var_content).ok();
            let local_var_error = ResponseContent { status: local_var_status, content: local_var_content, entity: local_var_entity };
            Err(Error::ResponseError(local_var_error))
        }
    }

}

/// struct for typed errors of method [`create_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUserError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_users_with_array_input`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUsersWithArrayInputError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`create_users_with_list_input`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CreateUsersWithListInputError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`delete_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DeleteUserError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`get_user_by_name`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum GetUserByNameError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`login_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoginUserError {
    Status400(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`logout_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LogoutUserError {
    DefaultResponse(),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`update_user`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum UpdateUserError {
    Status400(),
    Status404(),
    UnknownValue(serde_json::Value),
}


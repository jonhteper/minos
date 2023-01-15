//! This module allows you to convert authorizations to jwt and validate them.
use crate::core::authorization::{Authorization, Permission};
use crate::errors::MinosError;
use crate::utils::none_as_empty_string;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use non_empty_string::NonEmptyString;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
#[cfg(feature = "jwt")]
pub struct AuthorizationClaims {
    pub(crate) permissions: Vec<String>,
    pub(crate) agent_id: String,
    pub(crate) resource_id: String,
    pub(crate) resource_type: String,
    pub(crate) exp: u64,
}

impl AuthorizationClaims {
    pub fn new(
        permissions: Vec<String>,
        agent_id: NonEmptyString,
        resource_id: NonEmptyString,
        resource_type: String,
        exp: u64,
    ) -> Self {
        Self {
            permissions,
            agent_id: agent_id.to_string(),
            resource_id: resource_id.to_string(),
            resource_type,
            exp,
        }
    }

    pub fn permissions(&self) -> &Vec<String> {
        &self.permissions
    }

    pub fn user_id(&self) -> &str {
        &self.agent_id
    }

    pub fn resource_id(&self) -> &str {
        &self.resource_id
    }

    pub fn resource_type(&self) -> &str {
        &self.resource_type
    }

    pub fn expiration(&self) -> u64 {
        self.exp
    }

    fn string_permissions_to_vec_permissions(&self) -> Vec<Permission> {
        self.permissions
            .clone()
            .into_iter()
            .map(|p| Permission::from(p.as_str()))
            .collect()
    }

    pub fn as_authorization(&self) -> Result<Authorization, MinosError> {
        Ok(Authorization {
            permissions: self.string_permissions_to_vec_permissions(),
            agent_id: NonEmptyString::try_from(self.agent_id.as_str())?,
            resource_id: NonEmptyString::try_from(self.resource_id.as_str())?,
            resource_type: NonEmptyString::try_from(self.resource_type.as_str()).ok(),
            expiration: self.exp,
        })
    }

    fn permissions_as_vec_string(permissions: &[Permission]) -> Vec<String> {
        permissions.iter().map(|p| p.to_string()).collect()
    }
}

impl From<Authorization> for AuthorizationClaims {
    fn from(auth: Authorization) -> Self {
        AuthorizationClaims {
            permissions: AuthorizationClaims::permissions_as_vec_string(&auth.permissions),
            agent_id: auth.agent_id.to_string(),
            resource_id: auth.resource_id.to_string(),
            resource_type: none_as_empty_string(auth.resource_type.clone()),
            exp: auth.expiration,
        }
    }
}

#[cfg(feature = "jwt")]
pub struct TokenServer {
    header: Header,
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
    algorithm: Algorithm,
}

impl TokenServer {
    pub fn new(
        header: Header,
        encoding_key: EncodingKey,
        decoding_key: DecodingKey,
        algorithm: Algorithm,
    ) -> Self {
        Self {
            header,
            encoding_key,
            decoding_key,
            algorithm,
        }
    }

    pub fn generate_token(&self, claims: &AuthorizationClaims) -> Result<String, MinosError> {
        Ok(encode(&self.header, &claims, &self.encoding_key)?)
    }

    pub fn get_claims_by_token(&self, token: &str) -> Result<AuthorizationClaims, MinosError> {
        let validation = Validation::new(self.algorithm);
        let token_data = decode::<AuthorizationClaims>(token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}

use crate::authorization::{Authorization, Permission};
use crate::errors::{ErrorKind, MinosError};
use crate::resources::ResourceType;
use crate::utils;
use crate::utils::string_as_datetime;
use chrono::NaiveDateTime;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[allow(non_snake_case)]
#[cfg(feature = "jwt")]
pub struct AuthorizationClaims {
    pub(crate) permissions: Vec<String>,
    pub(crate) userId: String,
    pub(crate) resourceId: String,
    pub(crate) resourceType: String,
    pub(crate) expiration: String,
    exp: i64,
}

impl AuthorizationClaims {
    pub fn new(
        permissions: Vec<String>,
        user_id: String,
        resource_id: String,
        resource_type: String,
        expiration: NaiveDateTime,
    ) -> Self {
        Self {
            permissions,
            userId: user_id,
            resourceId: resource_id,
            resourceType: resource_type,
            expiration: expiration.to_string(),
            exp: expiration.timestamp(),
        }
    }
    pub fn permissions(&self) -> &Vec<String> {
        &self.permissions
    }
    pub fn user_id(&self) -> &str {
        &self.userId
    }
    pub fn resource_id(&self) -> &str {
        &self.resourceId
    }
    pub fn resource_type(&self) -> &str {
        &self.resourceType
    }
    pub fn expiration(&self) -> &str {
        &self.expiration
    }

    fn string_permissions_to_vec_permissions(&self) -> Vec<Permission> {
        self.permissions
            .clone()
            .into_iter()
            .map(|p| Permission::from(p.as_str()))
            .collect()
    }

    pub fn as_authorization(
        &self,
        resource_type: &ResourceType,
    ) -> Result<Authorization, MinosError> {
        if &self.resourceType != &resource_type.label {
            return Err(MinosError::new(
                ErrorKind::Io,
                "The resource types not match",
            ));
        }

        Ok(Authorization {
            permissions: self.string_permissions_to_vec_permissions(),
            user_id: self.userId.clone(),
            resource_id: self.resourceId.clone(),
            resource_type: resource_type.clone(),
            expiration: string_as_datetime(&self.expiration)?,
        })
    }

    fn permissions_as_vec_string(permissions: &Vec<Permission>) -> Vec<String> {
        permissions
            .clone()
            .into_iter()
            .map(|p| p.to_string())
            .collect()
    }
}

impl From<Authorization> for AuthorizationClaims {
    fn from(auth: Authorization) -> Self {
        AuthorizationClaims {
            permissions: AuthorizationClaims::permissions_as_vec_string(&auth.permissions),
            userId: auth.user_id,
            resourceId: auth.resource_id,
            resourceType: auth.resource_type.label,
            expiration: auth.expiration.format(utils::DATETIME_FMT).to_string(),
            exp: auth.expiration.timestamp(),
        }
    }
}

impl From<&Authorization> for AuthorizationClaims {
    fn from(auth: &Authorization) -> Self {
        AuthorizationClaims {
            permissions: AuthorizationClaims::permissions_as_vec_string(&auth.permissions),
            userId: auth.user_id.clone(),
            resourceId: auth.resource_id.clone(),
            resourceType: auth.resource_type.label.clone(),
            expiration: auth.expiration.format(utils::DATETIME_FMT).to_string(),
            exp: auth.expiration.timestamp(),
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
        let validation = Validation::new(self.algorithm.clone());
        let token_data = decode::<AuthorizationClaims>(&token, &self.decoding_key, &validation)?;
        Ok(token_data.claims)
    }
}

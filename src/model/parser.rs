use crate::errors::MinosError;
use crate::model::actor::{Actor, ToActor};
use crate::model::assertion::{Assertion, ToAssertions};
use crate::model::permission::{Permission, ToPermissions};
use crate::model::rule::{Rule, RuleBuilder, ToRule};
use fundu::parse_duration;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;
use serde_json::Value::Bool;
use serde_json::{Map, Value};
use std::ops::Deref;
use std::str::FromStr;

pub(crate) const LAST_SYNTAX_VERSION: &str = "0.7";

pub struct JsonParser<'obt> {
    object: &'obt Map<String, Value>,
}

impl ToPermissions for JsonParser<'_> {
    fn to_permissions(&self) -> Result<Vec<Permission>, MinosError> {
        let raw_permissions = self
            .object
            .get("permissions")
            .and_then(|val| val.as_array())
            .ok_or(MinosError::EmptyPermissions)?;

        let permissions = raw_permissions
            .par_iter()
            .map(|permission| {
                let permission = permission.as_array().ok_or(MinosError::EmptyPermissions)?;
                if permission.len() != 2 {
                    return Err(MinosError::PermissionsFormat(format!("{:?}", permission)));
                }

                let name = permission[0].as_str().ok_or_else(|| {
                    MinosError::PermissionNameFormat(format!("{:?}", permission[0]))
                })?;

                let duration_str = permission[1].as_str().ok_or_else(|| {
                    MinosError::PermissionDurationFormat(format!("{:?}", permission[1]))
                })?;
                let milliseconds = parse_duration(duration_str)
                    .map_err(|_| {
                        MinosError::PermissionDurationFormat(format!("{:?}", permission[1]))
                    })?
                    .as_millis();

                Ok(Permission::new(name, milliseconds))
            })
            .collect();

        permissions
    }
}

impl ToAssertions for JsonParser<'_> {
    fn to_assertions(&self) -> Result<Vec<Assertion>, MinosError> {
        let raw_assertions = self.object.get("assertions").and_then(|val| val.as_array());

        if raw_assertions.is_none() {
            return Ok(vec![]);
        }

        let assetions = raw_assertions
            .unwrap()
            .par_iter()
            .map(|val| {
                let raw_assertion = val
                    .as_str()
                    .ok_or_else(|| MinosError::InvalidAssertionSyntax(format!("{:?}", val)))?;

                Assertion::from_str(raw_assertion)
            })
            .collect();

        assetions
    }
}

macro_rules! get_attributes {
    ($object:ident, $key:literal) => {
        $object
            .get("resource")
            .and_then(|val| val.as_object())
            .cloned()
    };
}

impl ToRule for JsonParser<'_> {
    fn to_rule(&self) -> Result<Rule, MinosError> {
        let object = self.object;

        let permissions = self.to_permissions()?;
        let by_owner = object
            .get("by_owner")
            .unwrap_or(&Bool(false))
            .as_bool()
            .unwrap_or_default();

        let actor_attributes: Option<Map<String, Value>> = get_attributes!(object, "actor");
        let resource_attributes: Option<Map<String, Value>> = get_attributes!(object, "resource");
        let environment_attributes: Option<Map<String, Value>> =
            get_attributes!(object, "environment");
        let assertions = self.to_assertions()?;

        /*Ok(RuleBuilder::default()
        .permissions(permissions)
        .by_owner(by_owner)
        .actor_attributes(actor_attributes)
        .resource_attributes(resource_attributes)
        .environment_attributes(environment_attributes)
        .assertions(assertions)
        .build()
        .unwrap())*/

        Err(MinosError::__UnImplemented)
    }
}

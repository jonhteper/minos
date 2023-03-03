use fundu::parse_duration;
use serde_json::Value;
use crate::errors::MinosError;
use crate::model::permission::{Permission, ToPermissions};
use crate::model::rule::ToRule;

pub(crate) const LAST_SINTAXIS_VERSION: &str = "0.6";

pub struct Parser;

impl ToPermissions for Parser {
    fn to_permissions(&self, vec: &Vec<Value>) -> Result<Vec<Permission>, MinosError> {
        let mut permissions = vec![];

        for permission in vec {
            let permission = permission.as_array()
                .ok_or_else(|| MinosError::EmptyPermissions)?;
            if permission.len() != 2 {
                return Err(MinosError::PermissionsFormat(format!("{:?}", permission)));
            }

            let name = permission[0].as_str()
                .ok_or(MinosError::PermissionNameFormat(format!("{:?}", permission[0])))?;

            let duration_str = permission[1].as_str()
                .ok_or(MinosError::PermissionDurationFormat(format!("{:?}", permission[1])))?;
            let milliseconds = parse_duration(duration_str)
                .map_err(|_|{
                    MinosError::PermissionDurationFormat(format!("{:?}", permission[1]))
                })?.as_millis();


            permissions.push(Permission::new(name, milliseconds));
        }


        Ok(permissions)
    }

    fn permissions_to_string(&self, permissions: &Vec<Permission>) -> String {
        todo!()
    }
}




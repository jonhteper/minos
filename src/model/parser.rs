use fundu::parse_duration;
use rayon::prelude::IntoParallelRefIterator;
use serde_json::Value;
use crate::errors::MinosError;
use crate::model::permission::{Permission, ParsePermissions};
use crate::model::rule::ToRule;
use rayon::iter::ParallelIterator;

pub(crate) const LAST_SINTAXIS_VERSION: &str = "0.6";

pub struct Parser;

impl ParsePermissions for Parser {
    fn to_permissions(&self, vec: &[Value]) -> Result<Vec<Permission>, MinosError> {
        let permissions = vec.par_iter().map(|permission| {
            let permission = permission.as_array()
                .ok_or(MinosError::EmptyPermissions)?;
            if permission.len() != 2 {
                return Err(MinosError::PermissionsFormat(format!("{:?}", permission)));
            }

            let name = permission[0].as_str()
                .ok_or_else(|| MinosError::PermissionNameFormat(format!("{:?}", permission[0])))?;

            let duration_str = permission[1].as_str()
                .ok_or_else(||MinosError::PermissionDurationFormat(format!("{:?}", permission[1])))?;
            let milliseconds = parse_duration(duration_str)
                .map_err(|_|{
                    MinosError::PermissionDurationFormat(format!("{:?}", permission[1]))
                })?.as_millis();


            Ok(Permission::new(name, milliseconds))
        }).collect();



        permissions
    }

    fn permissions_to_string(&self, permissions: &[Permission]) -> String {
        todo!()
    }
}




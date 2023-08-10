use delegate::delegate;
use std::ops::Deref;

use crate::language::policy::Permission;



/// Hight level abstraction of [Permission] list.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Permissions(Vec<String>);

impl Permissions {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn has(&self, item: impl ToString) -> bool {
        self.0.contains(&item.to_string())
    }

    pub(crate) fn append_permissions(&mut self, permissions: &[Permission]) {
        self.0.extend(permissions.iter().map(|p| p.0.to_string()));
    }

    delegate! {
        to self.0 {
            pub fn len(&self) -> usize;
            pub fn is_empty(&self) -> bool;
            pub fn iter(&self) -> ::std::slice::Iter<String>;
            pub fn contains(&self, permission: &String) -> bool;
        }
    }
}

impl AsRef<[String]> for Permissions {
    fn as_ref(&self) -> &[String] {
        &self.0
    }
}

impl Deref for Permissions {
    type Target = [String];
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

use crate::actor::Actor;
use crate::authorization::{Authorization, AuthorizationMode, Policy};
use crate::errors::{ErrorKind, MinosError};
use crate::resources::Resource;
use chrono::Utc;

pub struct AuthorizationBuilder<'b, R: Resource> {
    resource: &'b R,
}

impl<'b, R: Resource> AuthorizationBuilder<'b, R> {
    pub fn new(resource: &'b R) -> Self {
        Self { resource }
    }

    fn check_groups<A: Agent>(&self, agent: &A, policy: &Policy) -> Result<(), MinosError> {
        if let Some(possible_ids) = &policy.groups_ids {
            for id in possible_ids {
                if agent.groups().contains(&id) {
                    return Ok(());
                }
            }

            return Err(MinosError::new(
                ErrorKind::Authorization,
                "The user is not in the correct group",
            ));
        }

        Ok(())
    }

    fn by_owner_check<A: Agent>(&self, agent: &A) -> Result<(), MinosError> {
        if self.resource.owner().is_none() {
            return Err(MinosError::new(
                ErrorKind::IncompatibleAuthPolicy,
                "The resource haven't an owner",
            ));
        }

        let owner = &self.resource.owner().unwrap();
        if owner != &agent.id() {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                "The agent is not the owner",
            ));
        }

        Ok(())
    }

    fn define_expiration(seconds: u64) -> u64 {
        (Utc::now().timestamp() + seconds as i64) as u64
    }

    fn policy_check(&self, policy: &Policy) -> Result<(), MinosError> {
        if !&self.resource.policies().contains(&policy) {
            return Err(MinosError::new(
                ErrorKind::IncompatibleAuthPolicy,
                "The policy not corresponds to resource type",
            ));
        }
        Ok(())
    }

    /// Create an [`Authorization`] based in a [`Resource`], a [`Policy`], and [`Agent`]. This function
    /// checks if the policy is malformed.
    ///
    /// # Errors
    /// This function will return an error in two cases:
    /// * [`IncompatibleAuthPolicy`]: The [`Policy`] not corresponds to [`Resource`] or the attribute
    ///   `by_owner` is true, but the [`Resource`] not have an owner.
    /// * [`Authorization`]: The [`Actor`] not have any permissions available.
    ///
    /// [`Actor`]: Actor
    /// [`Policy`]: Policy
    /// [`Resource`]: Resource
    /// [`IncompatibleAuthPolicy`]: ErrorKind::IncompatibleAuthPolicy
    /// [`Authorization`]: ErrorKind::Authorization
    pub fn build_by_policy<A: Actor>(
        &self,
        policy: &Policy,
        agent: &A,
    ) -> Result<Authorization, MinosError> {
        let _ = self.policy_check(&policy)?;
        if policy.by_owner {
            let _ = self.by_owner_check(agent)?;
        } else {
            let _ = self.check_groups(agent, &policy)?;
        }

        Ok(Authorization {
            permissions: policy.permissions.clone(),
            agent_id: agent.id(),
            resource_id: self.resource.id(),
            resource_type: self.resource.resource_type(),
            expiration: Self::define_expiration(policy.duration.clone()),
        })
    }

    /// Create an [`Authorization`] based in a [`Resource`] and an [`Actor`]. Check all policies and assign all
    ///  permissions available to the [`Actor`], but assign the shortest duration found.
    ///
    /// # Errors
    /// This function will return an error in two cases:
    /// * [`IncompatibleAuthPolicy`]: Some [`Policy`] not corresponds to [`Resource`] or the attribute
    ///   `by_owner` is true, but the [`Resource`] not have an owner.
    /// * [`Authorization`]: The [`Actor`] not have any permissions available.
    ///
    /// [`Actor`]: Actor
    /// [`Policy`]: Policy
    /// [`Resource`]: Resource
    /// [`IncompatibleAuthPolicy`]: ErrorKind::IncompatibleAuthPolicy
    /// [`Authorization`]: ErrorKind::Authorization
    pub fn build<A: Agent>(&self, agent: &A) -> Result<Authorization, MinosError> {
        let mut permissions = vec![];
        let mut durations = vec![];
        for policy in &self.resource.policies() {
            match self.build_by_policy(&policy, agent) {
                Ok(mut auth) => {
                    permissions.append(&mut auth.permissions);
                    durations.push(policy.duration);
                }
                Err(err) => {
                    if err.kind() == ErrorKind::IncompatibleAuthPolicy {
                        return Err(err);
                    }
                    continue;
                }
            }
        }

        durations.sort();
        let seconds = *durations
            .get(0)
            .ok_or(MinosError::new(ErrorKind::Authorization, "Not authorized"))?;

        Ok(Authorization {
            permissions,
            agent_id: actor.id(),
            resource_id: self.resource.id(),
            resource_type: self.resource.resource_type(),
            expiration: Self::define_expiration(seconds),
        })
    }
}

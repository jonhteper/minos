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

    fn single_group_check<A: Actor>(&self, actor: &A, policy: &Policy) -> Result<(), MinosError> {
        if policy.groups_ids.is_none() {
            return Err(MinosError::new(
                ErrorKind::IncompatibleAuthPolicy,
                "The policy haven't groups defined",
            ));
        }
        let possible_groups = policy.groups_ids.as_ref().unwrap();
        for group in possible_groups {
            if actor.groups().contains(group) {
                return Ok(());
            }
        }

        Err(MinosError::new(
            ErrorKind::Authorization,
            "The actor is not in the correct group",
        ))
    }

    fn multi_group_check<A: Actor>(&self, actor: &A, policy: &Policy) -> Result<(), MinosError> {
        let error = MinosError::new(
            ErrorKind::IncompatibleAuthPolicy,
            "The policy haven't groups defined",
        );
        if policy.groups_ids.is_none() {
            return Err(error);
        }
        let required_groups = policy.groups_ids.as_ref().unwrap();
        if required_groups.is_empty() {
            return Err(error);
        }
        for group in required_groups {
            if !&actor.groups().contains(group) {
                return Err(MinosError::new(
                    ErrorKind::Authorization,
                    "The actor is not in all required groups",
                ));
            }
        }

        Ok(())
    }

    fn by_owner_check<A: Actor>(&self, actor: &A) -> Result<(), MinosError> {
        let owner = &self.resource.owner().ok_or_else(|| {
            MinosError::new(
                ErrorKind::IncompatibleAuthPolicy,
                "The resource haven't an owner",
            )
        })?;
        if owner != &actor.id() {
            return Err(MinosError::new(
                ErrorKind::Authorization,
                "The actor is not the owner",
            ));
        }

        Ok(())
    }

    fn owner_single_group_check<A: Actor>(
        &self,
        actor: &A,
        policy: &Policy,
    ) -> Result<(), MinosError> {
        self.by_owner_check(actor)?;
        self.single_group_check(actor, policy)
    }

    fn owner_multi_group_check<A: Actor>(
        &self,
        actor: &A,
        policy: &Policy,
    ) -> Result<(), MinosError> {
        self.by_owner_check(actor)?;
        self.multi_group_check(actor, policy)
    }

    fn define_expiration(seconds: u64) -> u64 {
        (Utc::now().timestamp() + seconds as i64) as u64
    }

    fn policy_check(&self, policy: &Policy) -> Result<(), MinosError> {
        if !&self.resource.policies().contains(policy) {
            return Err(MinosError::new(
                ErrorKind::IncompatibleAuthPolicy,
                "The policy not corresponds to resource type",
            ));
        }
        Ok(())
    }

    fn mode_check<A: Actor>(&self, actor: &A, policy: &Policy) -> Result<(), MinosError> {
        match policy.auth_mode {
            AuthorizationMode::Owner => self.by_owner_check(actor),
            AuthorizationMode::SingleGroup => self.single_group_check(actor, policy),
            AuthorizationMode::MultiGroup => self.multi_group_check(actor, policy),
            AuthorizationMode::OwnerSingleGroup => self.owner_single_group_check(actor, policy),
            AuthorizationMode::OwnerMultiGroup => self.owner_multi_group_check(actor, policy),
        }
    }

    /// Create an [`Authorization`] based in a [`Resource`], a [`Policy`], and [`Actor`]. This function
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
        actor: &A,
    ) -> Result<Authorization, MinosError> {
        self.policy_check(policy)?;
        self.mode_check(actor, policy)?;

        Ok(Authorization {
            permissions: policy.permissions.clone(),
            agent_id: actor.id(),
            resource_id: self.resource.id(),
            resource_type: self.resource.resource_type(),
            expiration: Self::define_expiration(policy.duration.get()),
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
    pub fn build<A: Actor>(&self, actor: &A) -> Result<Authorization, MinosError> {
        let mut permissions = vec![];
        let mut durations = vec![];
        for policy in &self.resource.policies() {
            match self.build_by_policy(policy, actor) {
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
            .first()
            .ok_or_else(|| MinosError::new(ErrorKind::Authorization, "Not authorized"))?;

        Ok(Authorization {
            permissions,
            agent_id: actor.id(),
            resource_id: self.resource.id(),
            resource_type: self.resource.resource_type(),
            expiration: Self::define_expiration(seconds.get()),
        })
    }
}

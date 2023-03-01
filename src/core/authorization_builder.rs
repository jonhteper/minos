/*use crate::core::actor::Actor;
use crate::core::authorization::{Authorization, AuthorizationMode, Permission, Policy};
use crate::core::resources::Resource;
use crate::errors::{ErrorKind, MinosError};
use chrono::Utc;
use std::num::NonZeroU64;

#[derive(Clone)]
pub struct AuthorizationBuilder<'b, R: Resource, A: Actor> {
    resource: &'b R,
    actor: &'b A,
}

impl<'b, R: Resource, A: Actor> AuthorizationBuilder<'b, R, A> {
    pub fn new(actor: &'b A, resource: &'b R) -> Self {
        Self {
            actor,
            resource,
        }
    }

    /// Check if the [Actor] is member at least one of the authorized groups in [Policy].
    /// # Errors
    /// * [MinosError::EmptyGroupsPolicy] if the [Policy] don't have any group.
    /// * [MinosError::MissingGroup] if the [Actor] is not member for any authorized group.
    fn single_group_check(&self, actor: &A, policy: &Policy) -> Result<(), MinosError> {
        if policy.groups.is_none() {
            return Err(MinosError::EmptyGroupsPolicy);
        }
        let possible_groups = policy.groups.as_ref().unwrap();
        for group in possible_groups {
            if actor.groups().contains(group) {
                return Ok(());
            }
        }

        Err(MinosError::MissingGroup)
    }

    /// Check if the [Actor] is member of all authorized groups in [Policy].
    /// # Errors
    /// * [MinosError::EmptyGroupsPolicy] if the [Policy] don't have any group.
    /// * [MinosError::MissingGroup] if the [Actor] is not member of all groups in [Policy].
    fn multi_group_check(&self, actor: &A, policy: &Policy) -> Result<(), MinosError> {
        if policy.groups.is_none() {
            return Err(MinosError::EmptyGroupsPolicy);
        }
        let required_groups = policy.groups.as_ref().unwrap();
        if required_groups.is_empty() {
            return Err(MinosError::EmptyGroupsPolicy);
        }
        for group in required_groups {
            if !&actor.groups().contains(group) {
                return Err(MinosError::MissingGroup);
            }
        }

        Ok(())
    }

    /// Check if the [Actor] is the owner of the [Resource].
    /// # Errors
    /// * [MinosError::ResourceWithoutOwner] if the [Resource] not have an owner.
    /// * [MinosError::InvalidOwner] if the [Actor] is not the owner.
    fn by_owner_check(&self, actor: &A) -> Result<(), MinosError> {
        let owner = &self
            .resource
            .owner()
            .ok_or(MinosError::ResourceWithoutOwner)?;
        if owner != &actor.id() {
            return Err(MinosError::InvalidOwner);
        }

        Ok(())
    }

    /// Check if the [Actor] is the owner of the [Resource] and if is member at least one
    /// of the authorized groups in [Policy].
    /// # Errors
    /// * [MinosError::ResourceWithoutOwner] if the [Resource] not have an owner.
    /// * [MinosError::InvalidOwner] if the [Actor] is not the owner.
    /// * [MinosError::EmptyGroupsPolicy] if the [Policy] don't have any group.
    /// * [MinosError::MissingGroup] if the [Actor] is not member for any authorized group.
    fn owner_single_group_check(
        &self,
        actor: &A,
        policy: &Policy,
    ) -> Result<(), MinosError> {
        self.by_owner_check(actor)?;
        self.single_group_check(actor, policy)
    }

    /// Check if the [Actor] is the owner of the [Resource] and if is member of all authorized
    /// groups in [Policy].
    /// # Errors
    /// * [MinosError::ResourceWithoutOwner] if the [Resource] not have an owner.
    /// * [MinosError::InvalidOwner] if the [Actor] is not the owner.
    /// * [MinosError::EmptyGroupsPolicy] if the [Policy] don't have any group.
    /// * [MinosError::MissingGroup] if the [Actor] is not member of all groups in [Policy].
    fn owner_multi_group_check(
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

    /// Check if [Policy] corresponds to `resource type` of [Resource]
    /// # Errors
    /// * [MinosError::InvalidResourceTypePolicy]: if the [Policy] not corresponds to resource type.
    fn policy_check(&self, policy: &Policy) -> Result<(), MinosError> {
        if &self.resource.resource_type() != policy.resource_type.as_ref().unwrap() {
            return Err(MinosError::InvalidResourceTypePolicy);
        }
        Ok(())
    }

    /// Check if the [Actor] satisfies the [Policy]. Uses the policy [AuthorizationMode] to define
    /// the algorithm used.
    /// # Authorization modes
    ///
    /// ## Authorization by owner
    /// Check if the [Actor] is the owner of the [Resource].
    /// ### Errors
    /// * [MinosError::ResourceWithoutOwner] if the [Resource] not have an owner.
    /// * [MinosError::InvalidOwner] if the [Actor] is not the owner.
    ///
    /// ## Authorization by single group
    /// Check if the [Actor] is member at least one of the authorized groups in [Policy].
    /// ### Errors
    /// * [MinosError::EmptyGroupsPolicy] if the [Policy] don't have any group.
    /// * [MinosError::MissingGroup] if the [Actor] is not member for any authorized group.
    ///
    /// ## Authorization by multi group
    /// Check if the [Actor] is member of all authorized groups in [Policy].
    /// ### Errors
    /// * [MinosError::EmptyGroupsPolicy] if the [Policy] don't have any group.
    /// * [MinosError::MissingGroup] if the [Actor] is not member of all groups in [Policy].
    ///
    /// ## Authorization by owner and single group
    /// Check if the [Actor] is the owner of the [Resource] and if is member at least one
    /// of the authorized groups in [Policy].
    /// ### Errors
    /// * [MinosError::ResourceWithoutOwner] if the [Resource] not have an owner.
    /// * [MinosError::InvalidOwner] if the [Actor] is not the owner.
    /// * [MinosError::EmptyGroupsPolicy] if the [Policy] don't have any group.
    /// * [MinosError::MissingGroup] if the [Actor] is not member for any authorized group.
    ///
    /// ## Authorization by owner and multi group
    /// Check if the [Actor] is the owner of the [Resource] and if is member of all authorized
    /// groups in [Policy].
    /// ### Errors
    /// * [MinosError::ResourceWithoutOwner] if the [Resource] not have an owner.
    /// * [MinosError::InvalidOwner] if the [Actor] is not the owner.
    /// * [MinosError::EmptyGroupsPolicy] if the [Policy] don't have any group.
    /// * [MinosError::MissingGroup] if the [Actor] is not member of all groups in [Policy].
    fn check_by_policy(&self, actor: &A, policy: &Policy) -> Result<(), MinosError> {
        match policy.auth_mode {
            AuthorizationMode::Owner => self.by_owner_check(actor),
            AuthorizationMode::SingleGroup => self.single_group_check(actor, policy),
            AuthorizationMode::MultiGroup => self.multi_group_check(actor, policy),
            AuthorizationMode::OwnerSingleGroup => self.owner_single_group_check(actor, policy),
            AuthorizationMode::OwnerMultiGroup => self.owner_multi_group_check(actor, policy),
        }
    }

    /// Create an [Authorization] based in a [Resource], a [Policy], and [Actor]. This function
    /// checks if the policy is malformed.
    ///
    /// # Errors
    /// This function may fails and return one of the next errors:
    /// * [ResourceWithoutOwner] if the [Resource] not have an owner.
    /// * [InvalidOwner] if the [Actor] is not the owner.
    /// * [EmptyGroupsPolicy] if the [Policy] don't have any group.
    /// * [MissingGroup] if the [Actor] is not member for any authorized group
    /// or all authorized groups, according to [AuthorizationMode].
    ///
    /// [ResourceWithoutOwner]: MinosError::ResourceWithoutOwner
    /// [InvalidOwner]: MinosError::InvalidOwner
    /// [EmptyGroupsPolicy]: MinosError::EmptyGroupsPolicy
    /// [MissingGroup]: MinosError::MissingGroup
    pub fn build_by_policy(
        &self,
        policy: &Policy,
        actor: &A,
    ) -> Result<Authorization, MinosError> {
        self.policy_check(policy)?;
        self.check_by_policy(actor, policy)?;

        Ok(Authorization {
            permissions: policy.permissions.clone(),
            actor_id: actor.id(),
            resource_id: self.resource.id(),
            resource_type: self.resource.resource_type(),
            expiration: Self::define_expiration(policy.duration.get()),
        })
    }

    fn extract_permissions_and_durations(
        &self,
        actor: &A,
    ) -> Result<(Vec<Permission>, Vec<NonZeroU64>), MinosError> {
        let mut permissions = vec![];
        let mut durations = vec![];
        for policy in &self.resource.policies() {
            match self.build_by_policy(policy, actor) {
                Ok(mut auth) => {
                    permissions.append(&mut auth.permissions);
                    durations.push(policy.duration);
                }
                Err(err) => {
                    if err.kind() != ErrorKind::Unauthorized {
                        return Err(err);
                    }
                    continue;
                }
            }
        }
        durations.sort();

        Ok((permissions, durations))
    }

    /// Create an [`Authorization`] based in a [`Resource`] and an [`Actor`]. Check all policies
    /// and assign all permissions available to the [`Actor`], but assign the shortest duration
    /// found.
    ///
    /// # Errors
    /// This function may fails and return one of the next errors:
    /// * [ResourceWithoutOwner] if the [Resource] not have an owner.
    /// * [EmptyGroupsPolicy] if the [Policy] don't have any group.
    /// * [MissingPermissions] if the [Actor] does not satisfies any [Policy].
    ///
    ///
    /// [ResourceWithoutOwner]: MinosError::ResourceWithoutOwner
    /// [EmptyGroupsPolicy]: MinosError::EmptyGroupsPolicy
    /// [MissingPermissions]: MinosError::MissingPermissions
    pub fn build(&self, actor: &A) -> Result<Authorization, MinosError> {
        let (permissions, durations) = self.extract_permissions_and_durations(actor)?;
        let seconds = *durations.first().ok_or(MinosError::MissingPermissions)?;

        Ok(Authorization {
            permissions,
            actor_id: actor.id(),
            resource_id: self.resource.id(),
            resource_type: self.resource.resource_type(),
            expiration: Self::define_expiration(seconds.get()),
        })
    }
}
*/
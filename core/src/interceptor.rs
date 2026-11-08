// core/src/interceptor.rs
// Network gate: decides what a node may do, given its Authority.

use crate::authority::{Authority, Capability, Ring};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Decision {
    /// Send now, on the fast path.
    FastPath,
    /// Send, but through the normal queue.
    Normal,
    /// Send, but the node must pass MFA first.
    RequireMfa,
    /// Send, but rate-limited.
    RateLimited,
    /// Do not send.
    Deny,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    /// Call a kernel-level function (e.g. paramattha_execute on core).
    KernelCall,
    /// Send a message to another node.
    Send,
}

/// Decides whether an action may proceed for the given authority.
pub fn decide(authority: &Authority, action: Action) -> Decision {
    if authority.ring == Ring::Quarantine {
        return Decision::Deny;
    }
    match action {
        Action::KernelCall => {
            if authority.capabilities.contains(&Capability::KernelAccess) {
                Decision::FastPath
            } else {
                Decision::Deny
            }
        }
        Action::Send => {
            if authority
                .capabilities
                .contains(&Capability::PriorityBroadcast)
            {
                Decision::FastPath
            } else if authority.capabilities.contains(&Capability::RateLimited) {
                Decision::RateLimited
            } else if authority.capabilities.contains(&Capability::MfaRequired) {
                Decision::RequireMfa
            } else {
                Decision::Normal
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::authority::Authority;

    fn auth(ring: Ring, caps: &[Capability]) -> Authority {
        Authority {
            ring,
            capabilities: caps.to_vec(),
            reasons: vec![],
        }
    }

    #[test]
    fn sakka_kernel_call_is_fast_path() {
        let a = auth(
            Ring::Ring0,
            &[Capability::KernelAccess, Capability::PriorityBroadcast],
        );
        assert_eq!(decide(&a, Action::KernelCall), Decision::FastPath);
        assert_eq!(decide(&a, Action::Send), Decision::FastPath);
    }

    #[test]
    fn ring3_cannot_call_kernel_and_is_rate_limited() {
        let a = auth(
            Ring::Ring3,
            &[Capability::MfaRequired, Capability::RateLimited],
        );
        assert_eq!(decide(&a, Action::KernelCall), Decision::Deny);
        assert_eq!(decide(&a, Action::Send), Decision::RateLimited);
    }

    #[test]
    fn quarantine_denies_everything() {
        let a = auth(Ring::Quarantine, &[Capability::RateLimited]);
        assert_eq!(decide(&a, Action::Send), Decision::Deny);
        assert_eq!(decide(&a, Action::KernelCall), Decision::Deny);
    }

    #[test]
    fn ring2_send_requires_mfa() {
        let a = auth(Ring::Ring2, &[Capability::MfaRequired]);
        assert_eq!(decide(&a, Action::Send), Decision::RequireMfa);
    }
}

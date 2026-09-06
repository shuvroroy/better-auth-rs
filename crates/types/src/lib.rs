//! Shared Better Auth response types for Rust servers and WebAssembly clients.
//!
//! This crate contains the wire views, their entity traits, and invitation
//! status without depending on the authentication runtime or database adapters.
//! Client applications can deserialize responses using [`UserView`] and
//! [`SessionView`] with `serde`, including on `wasm32-unknown-unknown`.

pub mod entity;
mod invitation;
pub mod wire;

pub use entity::{
    AuthAccount, AuthApiKey, AuthInvitation, AuthMember, AuthOrganization, AuthPasskey,
    AuthSession, AuthTwoFactor, AuthUser, AuthVerification, MemberUserView,
};
pub use invitation::InvitationStatus;
pub use wire::{
    AccountView, ApiKeyView, InvitationView, OrganizationView, PasskeyView, SessionView, UserView,
    VerificationView,
};

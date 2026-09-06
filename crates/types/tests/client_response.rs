use better_auth_types::{SessionView, UserView};
use serde::Deserialize;

// These envelopes belong to the client application. Only the shared entity
// views are imported, so this target also compiles without the server crates.
#[derive(Deserialize)]
struct SignUpResponse {
    token: String,
    user: UserView,
}

#[derive(Deserialize)]
struct SessionResponse {
    session: SessionView,
    user: UserView,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct RuntimeCapture {
    version: String,
    signup: SignUpResponse,
    get_session: SessionResponse,
    unauthenticated_get_session: Option<SessionResponse>,
}

#[test]
fn client_deserializes_typescript_auth_responses() -> Result<(), serde_json::Error> {
    let captures: Vec<RuntimeCapture> =
        serde_json::from_str(include_str!("fixtures/core_auth_responses.json"))?;
    assert_eq!(captures.len(), 2);

    for capture in captures {
        let user = capture.get_session.user;
        let session = capture.get_session.session;
        assert_eq!(capture.signup.user, user, "{}", capture.version);
        assert_eq!(capture.signup.token, session.token);
        assert_eq!(session.user_id, user.id);
        assert_eq!(user.email.as_deref(), Some("types-fixture@example.com"));
        assert_eq!(user.name.as_deref(), Some("Types Fixture"));
        assert!(!user.email_verified);
        assert!(user.image.is_none());
        assert!(user.username.is_none());
        assert!(!user.two_factor_enabled);
        assert!(!user.banned);
        assert!(session.impersonated_by.is_none());
        assert!(session.active_organization_id.is_none());
        assert_eq!(session.ip_address.as_deref(), Some(""));
        assert_eq!(
            session.expires_at - session.created_at,
            chrono::Duration::days(7)
        );
        assert!(session.created_at >= user.created_at);
        assert!(capture.unauthenticated_get_session.is_none());

        // A client can tolerate additional plugin fields while internal-only
        // values stay out of the shared response representation.
        let mut user_json = serde_json::to_value(&user)?;
        user_json["customPluginField"] = serde_json::json!({"enabled": true});
        user_json["metadata"] = serde_json::json!({"internal": true});
        let parsed_user: UserView = serde_json::from_value(user_json)?;
        assert_eq!(parsed_user, user);
        assert!(serde_json::to_value(parsed_user)?.get("metadata").is_none());

        let mut session_json = serde_json::to_value(&session)?;
        session_json["customPluginField"] = serde_json::json!("extra");
        session_json["active"] = serde_json::json!(true);
        let parsed_session: SessionView = serde_json::from_value(session_json)?;
        assert_eq!(parsed_session, session);
        assert!(
            serde_json::to_value(parsed_session)?
                .get("active")
                .is_none()
        );
    }
    Ok(())
}

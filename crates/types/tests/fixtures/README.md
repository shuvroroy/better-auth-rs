`core_auth_responses.json` contains successful `/sign-up/email` and
authenticated `/get-session` responses, plus the unauthenticated
`/get-session` response, captured from two TypeScript runtimes:

- `better-auth@1.4.19`, installed by `compat-tests/client-tests`, with an
  in-memory SQLite database and email/password authentication enabled.
- The configured `compat-tests/reference-server/server.ts` runtime using
  `better-auth@1.6.29`, including its default plugins.

Each capture signs up `types-fixture@example.com` and passes the returned
session cookie to `/get-session`. Only IDs, session tokens, and the nonempty
user-agent value are replaced with fixed fixture values. Field names, dates,
null values, omitted fields, and the empty user-agent value are unchanged.

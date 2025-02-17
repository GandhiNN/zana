## Notes about SSO token
The tool uses the SSO and OIDC SDK to fetch available accounts and roles for your login and appends the session 
and profile configuration to your `~/.aws/config`.

It also places the authentication information it used itself in the `~/.aws/sso/cache` directory. 
These cached sessions are used by the newer credential provider implementations of AWS SDKs. 

The tool also supports usage of multiple sso sessions (combinations of sso_start_url and sso_region). 

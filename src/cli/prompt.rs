#[allow(dead_code)]
#[non_exhaustive]
pub struct Prompt {}

#[allow(dead_code)]
impl Prompt {
    pub const CONFIGURE_CREDENTIALS: &'static str = "Would you like to configure the credentials?";
    pub const REFRESH_CREDENTIALS: &'static str = "Would you like to refresh the credentials?";
    pub const CONFIGURE_PROFILE: &'static str = "Would you like to configure the default profile?";
    pub const CONFIGURE_REGION: &'static str = "Would you like to configure the default region?";
    pub const SELECT_ACCOUNT: &'static str = "Select the account you would like to use:";
    pub const SELECT_PROFILE: &'static str = "Select the profile you would like to use:";
    pub const INPUT_PROFILE: &'static str = "Enter the profile name you would like to use:";
    pub const SELECT_ROLE: &'static str = "Select the role you would like to assume:";
    pub const CONTINUE_PROGRAM: &'static str = "Would you like to continue?";
    pub const CLEAR_CACHE: &'static str = "Would you like to clear the cache?";
}

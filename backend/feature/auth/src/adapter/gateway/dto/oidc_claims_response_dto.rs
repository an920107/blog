use crate::domain::entity::user::User;

pub struct OidcClaimsResponseDto {
    pub sub: String,
    pub issuer: String,
    pub preferred_username: Option<String>,
    pub email: Option<String>,
}

impl OidcClaimsResponseDto {
    pub fn into_entity(self) -> User {
        User {
            id: -1,
            issuer: self.issuer,
            source_id: self.sub,
            displayed_name: self.preferred_username.unwrap_or_default(),
            email: self.email.unwrap_or_default(),
        }
    }
}

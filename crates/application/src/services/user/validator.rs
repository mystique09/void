use thiserror::Error;
use void_domain::entity::user::NickName;

#[derive(Debug)]
pub struct Request<'a> {
    pub nick_name: &'a str,
}
pub type Response = Result<(), UserInvalidity>;

#[derive(Debug, Error)]
pub enum UserInvalidity {
    #[error(transparent)]
    NickName(#[from] NickNameInvalidity),
}

#[derive(Debug, Error)]
pub enum NickNameInvalidity {
    #[error("Nickname must have at least {min} but got {actual} characters.")]
    MinLength { min: usize, actual: usize },
    #[error("Nickname must have at most {max} but got {actual} characters.")]
    MaxLength { max: usize, actual: usize },
}

pub fn validate_user_fields(request: &Request) -> Response {
    validate_nickname(request.nick_name).map_err(UserInvalidity::NickName)?;
    Ok(())
}

fn validate_nickname(nick_name: &str) -> Result<(), NickNameInvalidity> {
    let actual_length = nick_name.len();
    let min_length = NickName::min_len();

    if actual_length < min_length {
        return Err(NickNameInvalidity::MinLength {
            min: min_length,
            actual: actual_length,
        });
    }

    let max_length = NickName::max_len();
    if actual_length > max_length {
        return Err(NickNameInvalidity::MaxLength {
            max: max_length,
            actual: actual_length,
        });
    }

    Ok(())
}

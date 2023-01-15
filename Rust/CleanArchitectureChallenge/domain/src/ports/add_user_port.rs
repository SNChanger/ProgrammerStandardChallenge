use crate::entities::user_entity::UserEntity;
use crate::errors::add_user_error::AddUserError;

pub trait AddUserPort {
    fn add_user(&selfl lastname: &String, firstname: &String, email: &String) ->  Result<UserEntity, AddUserError>;;
}
#ちょまラジ 今日はいったんここまでにしますか。

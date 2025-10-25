use serenity::model::user::User;

pub fn user_to_tag(user: &User) -> String {
    user.name.clone()
}

use crate::entities::user::User;
use std::collections::HashMap;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct UserListProps {
    pub users: HashMap<u128, User>,
}

#[function_component(UserList)]
pub fn user_list(props: &UserListProps) -> Html {

    html! {
            <ul>
                { for props.users.iter().map(|(id, user)| html! {
                    <li key={id.clone()}>{ format!("{}: {}", id, user.name) }</li>
                }) }
            </ul>
    }
}

use crate::entities::user::{users_from_local_storage, User};
use yew::prelude::*;


#[function_component(UserList)]
pub fn user_list() -> Html {
    let users = users_from_local_storage();

    html! {
            <ul>
                { for users.iter().map(|(id, user)| html! {
                    <li key={id.clone()}>{ format!("{}: {}", id, user.name) }</li>
                }) }
            </ul>
    }
}

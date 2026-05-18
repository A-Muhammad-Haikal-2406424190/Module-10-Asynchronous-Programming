use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
       <div class="bg-gray-800 flex w-screen">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <div class="text-white text-xl font-semibold mb-2">{"YewChat Playground"}</div>
                <div class="text-gray-300 text-sm mb-5">{"Enter your username and start chatting in realtime."}</div>
                <form class="m-4 flex">
                    <input {oninput} class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-gray-200 bg-white" placeholder="Username" />
                    <Link<Route> to={Route::Chat}> <button {onclick} disabled={username.len()<1} class="px-8 rounded-r-lg bg-violet-600	  text-white font-bold p-4 uppercase border-violet-600 border-t border-b border-r" >{"Go Chatting!"}</button></Link<Route>>
                </form>
                <Link<Route> to={Route::About}>
                    <button class="mt-2 px-4 py-2 rounded-md border border-violet-400 text-violet-200 hover:bg-violet-700/30">
                        {"See Creative Notes"}
                    </button>
                </Link<Route>>
            </div>
        </div>
    }
}

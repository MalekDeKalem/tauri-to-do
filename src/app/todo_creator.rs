use yew::prelude::*;

use crate::app::todo_card::Todo;
use web_sys::console;

#[derive(Properties, PartialEq)]
pub struct TodoCreatorProps {
    pub list: Vec<Todo>,
    pub onadd: Callback<MouseEvent>,
    pub oninput: Callback<InputEvent>
}


#[function_component(TodoCreator)]
pub fn todo_creator(props: &TodoCreatorProps) -> Html {
    let show_dialog = use_state(|| false);
    let title = use_state(|| AttrValue::from(""));

    let open_dialog = {
        let show_dialog = show_dialog.clone();
        Callback::from(move |_| show_dialog.set(true))
    };


    let close_dialog = {
        let show_dialog = show_dialog.clone();
        Callback::from(move |_| show_dialog.set(false))
    };

    html! {
        <div>
            <div class="border-2 rounded-md border-gray-600 border-dashed mt-3 w-80 p-6 h-auto min-h-16 relative">
                <div class="flex justify-center">
                    <button
                        class="group cursor-pointer outline-none hover:rotate-90 duration-300"
                        title="Add New"
                        onclick={open_dialog}
                    >
                      <svg
                        class="stroke-slate-400 fill-none group-hover:fill-slate-800 group-active:stroke-slate-200 
                        group-active:fill-slate-800 group-active:duration-0 duration-300"
                        viewBox="0 0 24 24"
                        height="50px"
                        width="50px"
                        xmlns="http://www.w3.org/2000/svg"
                      >
                        <path
                          stroke-width="1.5"
                          d="M12 22C17.5 22 22 17.5 22 12C22 6.5 17.5 2 12 2C6.5 2 2 6.5 2 12C2 17.5 6.5 22 12 22Z"
                        ></path>
                        <path stroke-width="1.5" d="M8 12H16"></path>
                        <path stroke-width="1.5" d="M12 16V8"></path>
                      </svg>
                    </button>

                </div>
            </div>

            <div class="block flex items-center justify-center m-auto">

                {
                
                    if *show_dialog {
                        html! {
                            <div class="absolute size-1/3 bg-gray-800 z-40 rounded-xl ">
                                <div class="m-3">
                                    <button onclick={close_dialog}>{"Close"}</button>
                                </div>

                                <div class="mt-[10%] mx-5 bg-white w-fit text-black">
                                    <input type="text" placeholder="Title of the todo" oninput={props.oninput.clone()}/>
                                </div>


                                <div class="mt-5 mx-5 bg-white w-fit text-black">
                                    <button onclick={props.onadd.clone()}>{"Add"}</button>
                                </div>


                            </div>
                        }        
                    } else {
                        html! {}
                    }
                }

            </div>

        </div>
    }
}

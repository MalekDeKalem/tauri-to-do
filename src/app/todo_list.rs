use yew::prelude::*;

use crate::app::todo_card::Todo;
use crate::app::todo_card::TodoCard;
use crate::app::todo_creator::TodoCreator;


#[function_component(TodoList)]
pub fn todo_list() -> Html {
    let list = use_state(|| vec![
        Todo { id: 1, title: "Buy groceries".into(), done: false},
        Todo { id: 2, title: "Walk the dog".into(), done: false},
        Todo { id: 3, title: "Feed the cat".into(), done: false},
        Todo { id: 3, title: "Go to the mall and buy the dino nuggies".into(), done: false},
    ]);

    let title = use_state(|| AttrValue::from(""));
    let title_add = title.clone();
    let title_input = title.clone();


    let onadd = {
        let list = list.clone();
        Callback::from(move |_| {
            let mut new_list = (*list).clone();
            new_list.push(Todo {
                id: list.len(),
                title: (*title_add).clone(),
                done: false
            });
            list.set(new_list);
        })
    };


    let oninput = {
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                title_input.set(AttrValue::from(input.value()));
            }
        })
    };



    html! {
        <div>
            <div class="flex justify-center w-full">
                <div class="flex flex-col items-center">
                    {
                        for list.iter().map(|todo| html!{
                            <TodoCard todo={todo.clone()} />
                        })
                    } 
                    <TodoCreator list={(*list).clone()} onadd={onadd.clone()} oninput={oninput.clone()} />
                </div>
                
            </div>
            <button onclick={onadd}> </button> 
        </div>
    }
}

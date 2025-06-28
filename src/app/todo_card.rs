use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub struct Todo {
    pub id: usize,
    pub title: AttrValue,
    pub done: bool,
}


#[derive(Properties, PartialEq)]
pub struct TodoCardProps {
    pub todo: Todo,
}

#[function_component(TodoCard)]
pub fn todo_card(props: &TodoCardProps) -> Html {


   html! {
      <div class="border-2 rounded-md border-gray-100 mt-3 w-80 p-6 h-auto min-h-16 relative bg-black">
            <div class="flex justify-center">
                <div class="absolute top-0 left-0 rounded-full flex justify-center items-center w-5 h-5 text-sm font-bold border border-gray-100 p-1 m-2 bg-white">
                    <span class="text-black">{&props.todo.id}</span>
                </div>
                <p class="text-gray-100 text-lg text-center">{&props.todo.title}</p>
            </div>
       </div>
   } 
}

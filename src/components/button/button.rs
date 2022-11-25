use yew::prelude::*;
use stylist::yew::styled_component;
#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    pub name: String,
}

#[styled_component(Button)]
pub fn button(props: &ButtonProps) -> Html {
    html! {
        <div>
        <button className="border-1 border-black text-black font-bold hover:bg-black hover:text-white  py-2 px-8 rounded-full">
            {props.name.clone()}
        </button>
        </div>
    }
}
// #[derive(Debug)]
// struct Button {
//     String name;
// }
// impl Component for Button {
//     type Properties();

    
//     fn view(&self, ctx: &Context<Self>) -> Html {
//         let link = ctx.link();
//         html! {
//             <div>
//                 <button onclick={link.callback(|_| Msg::AddOne)}>{ "+1" }</button>
//                 <p>{ self.value }</p>
//             </div>
//         }
//     }
// }
// const Button = (props: { name: string }) => {
//   return (
//     <div>
//     </div>
//   );
// };

// export default Button;

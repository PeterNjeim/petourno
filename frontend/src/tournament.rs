use yew::prelude::*;

pub enum Msg {
    Update(String),
}

pub struct Text {
    link: ComponentLink<Self>,
    content: String,
}

impl Component for Text {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            content: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Update(content) => self.content = content,
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
                <label for="players">{ "Players (one per line):" }</label>
                <textarea id="players" name="players"
                    oninput=self.link.callback(|event: InputData| Msg::Update(event.value))
                    value=self.content.clone()>
                </textarea>
            </div>
        }
    }
}

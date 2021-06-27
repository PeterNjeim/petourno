use material_yew::text_inputs::TextFieldType;
use material_yew::*;
use petgraph::stable_graph::StableGraph;
use tournament::generation::single_elim::{self, *};
use yew::html;
use yew::prelude::*;

enum Msg {
    TournamentSubmit,
    UpdatePlayers(String),
    UpdateTuple(String),
}

struct Tournament {
    // `ComponentLink` is like a reference to a component.
    // It can be used to send messages to the component
    link: ComponentLink<Self>,
    value: StableGraph<GraphSet, SetEdge>,
    players: String,
    tuple: String,
}

impl Component for Tournament {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: StableGraph::new(),
            players: String::new(),
            tuple: String::new(),
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::TournamentSubmit => {
                self.value = single_elim::new_elim(
                    self.players
                        .lines()
                        .map(|p| p.to_owned())
                        .collect::<Vec<String>>(),
                    self.tuple.parse::<u64>().unwrap(),
                );
                // the value has changed so we need to
                // re-render for it to appear on the page
                true
            }
            Msg::UpdatePlayers(content) => {
                self.players = content;
                true
            }
            Msg::UpdateTuple(content) => {
                self.tuple = content;
                true
            }
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        // Should only return "true" if new properties are different to
        // previously received properties.
        // This component has no properties so we will always return "false".
        true
    }

    fn view(&self) -> Html {
        html! {
            <>
                <MatTopAppBarFixed>{html! {<span slot="title">{ "Petourno" }</span>}}</MatTopAppBarFixed>
                <div style="padding: 16px;">
                <div style="display: flex; flex-direction: column;">
                    <MatTextField oninput=self.link.callback(|event: InputData| Msg::UpdateTuple(event.value)) value=self.tuple.clone() outlined=true label="Tuple" icon="group_work" field_type=TextFieldType::Number />
                    <MatTextField  outlined=true label="Name" icon="create" />
                    <MatTextArea oninput=self.link.callback(|event: InputData| Msg::UpdatePlayers(event.value)) value=self.players.clone() outlined=true label="Players" helper="One per line" helper_persistent=true icon="group_work" />
                    <span onclick=self.link.callback(|_| Msg::TournamentSubmit)>
                        <MatButton label="Generate" />
                    </span>
                </div>
                {for (1..=self.tuple.parse().unwrap_or(1)).map(|bracket| html! {
                <div style="display: grid; border-bottom: 2px black solid;">
                    {for self.value.clone().node_weights_mut().filter(|set| set.bracket == bracket).map(|set| html! {<div style=format!("grid-area: {0} / {2} / {1} / {3}; display: inline-flex; border: 1px solid #d3d3d3; margin: 4px; max-width: 400px; margin-bottom: 8px;", (set.position - 1) * (1 << set.round) + (1 << (set.round - 1)), (set.position - 1) * (1 << set.round) + (1 << (set.round - 1)) + 2, set.round, set.round + 1)>
                    <div style="align-self: center;">
                    <MatList>
                    <MatListItem>{ format!("{}", set.game)}</MatListItem>
                    </MatList>
                    </div>
                    <div style="width: 100%; align-self: center; border-left: 1px solid #d3d3d3;">
                    <MatList>
                    <MatListItem>{ format!("{}", set.placeholders.0) }</MatListItem>
                    <MatListItem>{ format!("{}", set.placeholders.1) }</MatListItem></MatList></div></div>})}
                </div>
                })}
                </div>
            </>
        }
    }
}

fn main() {
    yew::start_app::<Tournament>();
}

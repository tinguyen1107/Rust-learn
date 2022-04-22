use serde_json::Value;
use web_sys::*;
use weblog::*;
use yew::*;

#[derive(PartialEq, Debug, Clone)]
pub struct Pokemon {
    id: usize,
    name: String,
    image_src: String,
}

#[function_component(Icon)]
pub fn icon() -> Html {
    let pokemon_state = use_state_eq::<Option<Pokemon>, _>(|| None);
    let pokemon_state_outer = pokemon_state.clone();

    let onclick = Callback::from(move |_: MouseEvent| {
        console::log_1(&format!("{:?}", pokemon_state).into());

        let pokemon_state = pokemon_state.clone();

        wasm_bindgen_futures::spawn_local(async move {
            let res = reqwest::get("https://pokeapi.co/api/v2/pokemon/22")
                .await
                .unwrap();
            let text = res.text().await.unwrap();
            let v: Value = serde_json::from_str(&text).unwrap();
            let name = v["name"].as_str().unwrap();
            let image_src = v["sprites"]["front_default"].as_str().unwrap();

            let pokemon = Pokemon {
                id: 22,
                name: name.into(),
                image_src: image_src.into(),
            };
            pokemon_state.set(Some(pokemon));
            console::log_2(&name.into(), &image_src.into());
        });
    });

    html! {
        <div>
            <button {onclick}>{"Get pokemon"}</button>
            <ViewPokemon pokemon={(*pokemon_state_outer).clone()} />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct ViewPokemonProps {
    pokemon: Option<Pokemon>,
}

#[function_component(ViewPokemon)]
pub fn view_pokemon(props: &ViewPokemonProps) -> Html {
    let pokemon = match &props.pokemon {
        Some(p) => p,
        None => return html!{}
    };

    let input_ref = NodeRef::default();
    let input_ref_outer = input_ref.clone();
    let onclick = Callback::from(move |_| {
       let input = input_ref.cast::<HtmlInputElement>().unwrap(); 
       let guess = input.value().to_lowercase();
       console::log_1(&format!("{:?}", guess).into());
    });

    html! {
       <div>
           <img src={pokemon.image_src.clone()}/>
           <h3>{pokemon.name.clone()}</h3>
            <div class="input_container">
                <input type="text"
                    placeholder="Please input index pokemon.."
                    ref={input_ref_outer.clone()}
                    />
                <button {onclick}>{"Search"}</button>
            </div>
       </div>
    }
}

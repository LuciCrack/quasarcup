use yew::prelude::*;
use web_sys::window;

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    pub title: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(Layout)]
pub fn layout(props: &LayoutProps) -> Html {
    // Set the page title
    {
        let title = props.title.clone();
        use_effect_with((), move |_| {
            if let Some(doc) = window().and_then(|w| w.document()) {
                doc.set_title(&format!("Quasar Cup - {}", title));
            }
            || ()
        });
    }

    html! {
        <>
            <header> 
                // QUASAR 
                <div class="header-top">
                    <text class="quasar"> { "QUASAR" } </text>
                    <a class="top-left-buttons" href="https://github.com/LuciCrack/quasarcup">{ "GitHub" }</a>
                </div>
                // Github Button
                <div class="header-nav">
                    <nav class="header-nav">
                        <a href="/">{"Home"}</a>
                        {" | "}
                        <a href="/tournament">{"Create Tournament"}</a>
                        {" | "}
                        <a href="/search">{"Search"}</a>
                        {" | "}
                        <a href="https://github.com/LuciCrack/quasarcup">{ "GitHub" }</a>
                    </nav>
                </div>
            </header>
            <main>
                { for props.children.iter() }
            </main>
            <footer>
                // Quasar Cup @ 2025
                // About me 
                // GitHub 
                // Code Licensce
            </footer>
        </>
    }
}


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
              <div class="frame header-a6197eb64f28">
                <div class="shape text q-u-a-s-a-r-a61eb4af3ee5">
                  <svg width="737" xmlns="http://www.w3.org/2000/svg" height="386" id="screenshot-4d728884-4b02-80a2-8006-a61eb4af3ee5" viewBox="3225 -2303 737 386" style="-webkit-print-color-adjust::exact" xlink="http://www.w3.org/1999/xlink" fill="none" version="1.1">
                    <g id="shape-4d728884-4b02-80a2-8006-a61eb4af3ee5">
                      <defs>
                        <filter id="filter-render-405" x="-0.0817391304347826" y="-0.19583333333333333" width="1.2156521739130435" height="1.4666666666666666" filterUnits="objectBoundingBox" color-interpolation-filters="sRGB">
                          <feFlood flood-opacity="0" result="BackgroundImageFix"></feFlood>
                          <feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"></feColorMatrix>
                          <feMorphology radius="5" operator="dilate" in="SourceAlpha" result="filter_4d728884-4b02-80a2-8006-a61ee2d717ef"></feMorphology>
                          <feOffset dx="10" dy="6"></feOffset>
                          <feGaussianBlur stdDeviation="2"></feGaussianBlur>
                          <feColorMatrix type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0"></feColorMatrix>
                          <feBlend mode="normal" in2="BackgroundImageFix" result="filter_4d728884-4b02-80a2-8006-a61ee2d717ef"></feBlend>
                          <feBlend mode="normal" in="SourceGraphic" in2="filter_4d728884-4b02-80a2-8006-a61ee2d717ef" result="shape"></feBlend>
                        </filter>
                        <filter id="filter-shadow-render-405" x="-0.0817391304347826" y="-0.19583333333333333" width="1.2156521739130435" height="1.4666666666666666" filterUnits="objectBoundingBox" color-interpolation-filters="sRGB">
                          <feFlood flood-opacity="0" result="BackgroundImageFix"></feFlood>
                          <feColorMatrix in="SourceAlpha" type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 127 0"></feColorMatrix>
                          <feMorphology radius="5" operator="dilate" in="SourceAlpha" result="filter_4d728884-4b02-80a2-8006-a61ee2d717ef"></feMorphology>
                          <feOffset dx="10" dy="6"></feOffset>
                          <feGaussianBlur stdDeviation="2"></feGaussianBlur>
                          <feColorMatrix type="matrix" values="0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 1 0"></feColorMatrix>
                          <feBlend mode="normal" in2="BackgroundImageFix" result="filter_4d728884-4b02-80a2-8006-a61ee2d717ef"></feBlend>
                          <feBlend mode="normal" in="SourceGraphic" in2="filter_4d728884-4b02-80a2-8006-a61ee2d717ef" result="shape"></feBlend>
                        </filter>
                      </defs>
                      <g transform="matrix(1.000000, 0.000000, 0.000000, 1.000000, 0.000000, 0.000000)" class="text-container" x="3276" y="-2248" width="575" height="240" filter="url(#filter-render-405)" rx="0" ry="0">
                        <defs>
                          <pattern patternUnits="objectBoundingBox" width="574.4000244140625" height="236.5" id="fill-0-render-405-0">
                            <g>
                              <rect width="574.4000244140625" height="236.5" style="fill:#e8ca72;fill-opacity:1"></rect>
                            </g>
                          </pattern>
                        </defs>
                        <g class="fills" id="fills-4d728884-4b02-80a2-8006-a61eb4af3ee5"><text x="0" y="0" dominant-baseline="ideographic" textLength="574.4000244140625" lengthAdjust="spacingAndGlyphs" style="text-transform:none;font-family:&quot;Londrina Solid&quot;;letter-spacing:1px;font-style:normal;font-weight:400;white-space:pre;font-size:200px;text-decoration:rgb(232, 202, 114);direction:ltr;fill:#e8ca72;fill-opacity:1"> { "QUASAR" } </text></g>
                        <g id="strokes-db3c1e32-4f7f-8094-8006-ab2970744ce8-4d728884-4b02-80a2-8006-a61eb4af3ee5" class="strokes">
                          <g class="outer-stroke-shape">
                            <defs>
                              <mask id="outer-stroke-render-405-0-4d728884-4b02-80a2-8006-a61eb4af3ee5-0" x="0" y="0" width="582.885305788301" height="244.98528137423858" maskUnits="userSpaceOnUse">
                                <use href="#stroke-shape-render-405-0-4d728884-4b02-80a2-8006-a61eb4af3ee5-0" style="fill:none;stroke:white;stroke-width:6"></use>
                                <use href="#stroke-shape-render-405-0-4d728884-4b02-80a2-8006-a61eb4af3ee5-0" style="fill:black;stroke:none"></use>
                              </mask><text x="0" y="0" dominant-baseline="ideographic" textLength="574.4000244140625" lengthAdjust="spacingAndGlyphs" style="text-transform:none;font-family:&quot;Londrina Solid&quot;;letter-spacing:1px;font-style:normal;font-weight:400;white-space:pre;font-size:200px;text-decoration:rgb(232, 202, 114);direction:ltr" id="stroke-shape-render-405-0-4d728884-4b02-80a2-8006-a61eb4af3ee5-0"> { "QUASAR" } </text>
                            </defs>
                            <use href="#stroke-shape-render-405-0-4d728884-4b02-80a2-8006-a61eb4af3ee5-0" mask="url(#outer-stroke-render-405-0-4d728884-4b02-80a2-8006-a61eb4af3ee5-0)" style="text-transform:none;font-family:&quot;Londrina Solid&quot;;letter-spacing:1px;font-style:normal;font-weight:400;white-space:pre;font-size:200px;text-decoration:rgb(232, 202, 114);direction:ltr;fill:none;stroke-width:6;stroke:#000000;stroke-opacity:1"></use>
                            <use href="#stroke-shape-render-405-0-4d728884-4b02-80a2-8006-a61eb4af3ee5-0" style="text-transform:none;font-family:&quot;Londrina Solid&quot;;letter-spacing:1px;font-style:normal;font-weight:400;white-space:pre;font-size:200px;text-decoration:rgb(232, 202, 114);direction:ltr;fill:none;fill-opacity:none;stroke-width:3;stroke:none;stroke-opacity:1"></use>
                          </g>
                        </g>
                      </g>
                    </g>
                  </svg>
                </div>
              </div>
                <nav>
                    <a href="/">{"Home"}</a>
                    {" | "}
                    <a href="/tournament">{"Create Tournament"}</a>
                    {" | "}
                    <a href="/search">{"Search"}</a>
                    {" | "}
                    <a href="https://github.com/LuciCrack/quasarcup">{ "GitHub" }</a>
                </nav>
            </header>
            <main>
                { for props.children.iter() }
            </main>
            <footer>
                <div class="frame footer-ab21776dfbfc">
                  <div class="shape text quasar-cup-ab2184d3777d">
                    <div class="text-node-html" id="html-text-node-db3c1e32-4f7f-8094-8006-ab2184d3777d" data-x="3303" data-y="-1290">
                      <div class="root rich-text root-0" xmlns="http://www.w3.org/1999/xhtml">
                        <div class="paragraph-set root-0-paragraph-set-0">
                          <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="ltr"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0"> { "Quasar Cup @ 2025" } </span></p>
                        </div>
                      </div>
                    </div>
                  </div>
                  <div class="shape text about-me-ab227d3d955d">
                    <div class="text-node-html" id="html-text-node-db3c1e32-4f7f-8094-8006-ab227d3d955d" data-x="3554" data-y="-1206">
                      <div class="root rich-text root-0" xmlns="http://www.w3.org/1999/xhtml">
                        <div class="paragraph-set root-0-paragraph-set-0">
                          <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="ltr"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0"> {"About Me" } </span></p>
                        </div>
                      </div>
                    </div>
                  </div>
                  <div class="shape text git-hub-ab2eeb8ae287">
                    <div class="text-node-html" id="html-text-node-db3c1e32-4f7f-8094-8006-ab2eeb8ae287" data-x="3337" data-y="-1206">
                      <div class="root rich-text root-0" xmlns="http://www.w3.org/1999/xhtml">
                        <div class="paragraph-set root-0-paragraph-set-0">
                          <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="ltr"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0"> { "GitHub" } </span></p>
                        </div>
                      </div>
                    </div>
                  </div>
                  <div class="shape text code-licen-ab22685b7320">
                    <div class="text-node-html" id="html-text-node-db3c1e32-4f7f-8094-8006-ab22685b7320" data-x="3888" data-y="-1281">
                      <div class="root rich-text root-0" xmlns="http://www.w3.org/1999/xhtml">
                        <div class="paragraph-set root-0-paragraph-set-0">
                          <p class="paragraph root-0-paragraph-set-0-paragraph-0" dir="ltr"><span class="text-node root-0-paragraph-set-0-paragraph-0-text-0"> { "Code license under.. " } </span></p>
                        </div>
                      </div>
                    </div>
                  </div>
                </div>
            </footer>
        </>
    }
}

use leptos::prelude::*;
use leptos_meta::{provide_meta_context, MetaTags, Stylesheet, Title};
use leptos_router::{
    components::{Outlet, ProtectedParentRoute, Route, Router, Routes, A},
    lazy_route, Lazy, LazyRoute, StaticSegment,
};

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
            <head>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
                <AutoReload options=options.clone()/>
                <HydrationScripts options/>
                <MetaTags/>
            </head>
            <body>
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let auth = Resource::new(|| (), |_| check_auth());
    let auth_signal: RwSignal<Option<bool>> = RwSignal::new(None);
    Effect::new(move |_| {
        if let Some(Ok(value)) = auth.get() {
            auth_signal.set(Some(value));
        }
    });
    let auth_condition = move || {
        let value = auth_signal.get();
        #[cfg(feature = "ssr")]
        eprintln!("[trace-ssr] auth_condition -> {:?}", value);
        #[cfg(not(feature = "ssr"))]
        leptos::logging::log!("[trace-hydrate] auth_condition -> {:?}", value);
        value
    };

    view! {
        <Stylesheet id="leptos" href="/pkg/hydrate-branching-mismatch.css"/>
        <Title text="Hydrate Branching Mismatch"/>

        <Router>
            <main class="app-shell">
                <nav>
                    <A href="/">"Home"</A>
                    <A href="/guarded">"Guarded route"</A>
                </nav>

                <Routes fallback=|| view! { <p>"Not found."</p> } transition=true>
                    <Route path=StaticSegment("") view=HomePage/>
                    <ProtectedParentRoute
                        path=StaticSegment("guarded")
                        view=GuardedLayout
                        condition=auth_condition
                        redirect_path=|| "/"
                        fallback=|| ()
                    >
                        <Route path=StaticSegment("") view={Lazy::<LazyGuardedPage>::new()}/>
                    </ProtectedParentRoute>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <section class="panel">
            <h1>"Hydrate branching mismatch repro"</h1>
            <p>
                "Open "
                <a href="/guarded">"/guarded"</a>
                " directly with DevTools open. This repo delays client hydration, without WASM splitting, so the guarded route can choose a different branch on the client than the server emitted."
            </p>
            <p>
                "Before the branch-marker fix, hydration should panic because the server emitted the pending fallback branch while the delayed client hydration computes the authenticated branch."
            </p>
        </section>
    }
}

#[component]
fn GuardedLayout() -> impl IntoView {
    view! {
        <section class="panel guarded-layout">
            <h1>"Guarded layout"</h1>
            <Outlet/>
        </section>
    }
}

#[component]
fn GuardedPage() -> impl IntoView {
    view! {
        <div id="guarded-page" class="success">
            <h2>"Authenticated branch"</h2>
            <p>"If this hydrates successfully, the client branch replaced or matched the server branch."</p>
            <button on:click=|_| leptos::logging::log!("guarded button clicked")>
                "Hydrated button"
            </button>
        </div>
    }
}

pub struct LazyGuardedPage;

#[lazy_route]
impl LazyRoute for LazyGuardedPage {
    fn data() -> Self {
        Self
    }

    fn view(_this: Self) -> AnyView {
        view! { <GuardedPage/> }.into_any()
    }
}

#[server(CheckAuth, "/api")]
async fn check_auth() -> Result<bool, ServerFnError> {
    tokio::time::sleep(std::time::Duration::from_millis(1200)).await;
    Ok(true)
}

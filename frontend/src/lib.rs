//frontend//src/lib.rs
use leptos::*;
use leptos_router::*;
use wasm_bindgen::prelude::*;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
struct Review {
    review_title: String,
    review_body: String,
    product_id: String,
    review_rating: f32,
}

#[derive(Deserialize, Clone)]
struct SearchHit {
    id: u64,
    score: f32,
    review: Review,
}
#[component]
fn IndexPage(cx: Scope) -> impl IntoView {
    let (title, set_title) = create_signal(cx, String::new());
    let (body, set_body) = create_signal(cx, String::new());
    let (pid, set_pid) = create_signal(cx, String::new());
    let (rating, set_rating) = create_signal(cx, 5.0f32);

    let (status, set_status) = create_signal(cx, String::new());
    let (loading, set_loading) = create_signal(cx, false);

    let submit = move |_| {
        let t = title.get();
        let b = body.get();
        let p = pid.get();
        let r = rating.get();

        set_loading.set(true);
        set_status.set(String::new());

        wasm_bindgen_futures::spawn_local(async move {
            let payload = NewReview {
                review_title: t,
                review_body: b,
                product_id: p,
                review_rating: r, // f32
            };
            let resp = Request::post("/reviews")
                .header("content-type", "application/json")
                .json(&payload).unwrap()
                .send().await;

            match resp {
                Ok(rsp) if rsp.status() == 200 || rsp.status() == 201 => {
                    // small UX: clear after success
                    set_status.set("Inserted!".into());
                }
                Ok(rsp) => {
                    let text = rsp.text().await.unwrap_or_default();
                    set_status.set(format!("Error {}: {}", rsp.status(), text));
                }
                Err(e) => set_status.set(format!("Network error: {e}")),
            }
            set_loading.set(false);
        });
    };

    view! { cx,
      <div class="container" style="max-width:640px;margin:2rem auto;font-family:sans-serif;">
        <h1>"Semantic Search"</h1>
        <h2>"Add a Review"</h2>
        <div style="display:flex;flex-direction:column;gap:.5rem;">
          <input placeholder="Title" on:input=move |e| set_title.set(event_target_value(&e)) />
          <textarea placeholder="Body" rows=6 on:input=move |e| set_body.set(event_target_value(&e)) />
          <input placeholder="Product ID" on:input=move |e| set_pid.set(event_target_value(&e)) />
          <label>
            "Rating: "
            <input type="range" min="0" max="5" step="0.5"
              on:input=move |e| {
                let v = event_target_value(&e).parse().unwrap_or(5.0);
                set_rating.set(v);
              }/>
            <span>{move || format!("{:.1}", rating.get())}</span>
          </label>
          <button on:click=submit disabled=move || loading.get()>
            {move || if loading.get() { "Saving..." } else { "Submit" }}
          </button>
          <p style="color:#555;">{move || status.get()}</p>
        </div>
        <hr />
        <A href="/search">"Go to Search →"</A>
      </div>
    }
}

#[derive(Serialize)]
struct SearchReq { query: String, top_k: u32 }

#[component]
fn SearchPage(cx: Scope) -> impl IntoView {
    let (q, set_q) = create_signal(cx, String::new());
    let (k, set_k) = create_signal(cx, 5u32);
    let (loading, set_loading) = create_signal(cx, false);
    let (error, set_error) = create_signal(cx, String::new());
    let (hits, set_hits) = create_signal::<Vec<SearchHit>>(cx, vec![]);

    let run_search = move |_| {
        let qq = q.get();
        let kk = k.get();
        set_loading.set(true);
        set_error.set(String::new());
        set_hits.set(vec![]);
        wasm_bindgen_futures::spawn_local(async move {
            let resp = Request::post("/search")
                .header("content-type", "application/json")
                .json(&SearchReq { query: qq, top_k: kk }).unwrap()
                .send().await;

            match resp {
                Ok(rsp) if rsp.status() == 200 => {
                    match rsp.json::<Vec<SearchHit>>().await {
                        Ok(list) => set_hits.set(list),
                        Err(e) => set_error.set(format!("Parse error: {e}")),
                    }
                }
                Ok(rsp) => {
                    let text = rsp.text().await.unwrap_or_default();
                    set_error.set(format!("Error {}: {}", rsp.status(), text));
                }
                Err(e) => set_error.set(format!("Network error: {e}")),
            }
            set_loading.set(false);
        });
    };

    view! { cx,
      <div class="container" style="max-width:760px;margin:2rem auto;font-family:sans-serif;">
        <h1>"Search Reviews"</h1>
        <div style="display:flex;gap:.5rem;">
          <input style="flex:1" placeholder="Type your query..."
            on:input=move |e| set_q.set(event_target_value(&e)) />
          <button on:click=run_search disabled=move || loading.get()>
            {move || if loading.get() { "Searching..." } else { "Search" }}
          </button>
        </div>
        <div style="margin:.5rem 0;">
          <label>
            "Top K: "
            <input type="range" min="1" max="50" value=move || k.get().to_string()
              on:input=move |e| set_k.set(event_target_value(&e).parse().unwrap_or(5)) />
            <span style="margin-left:.5rem;">{move || k.get().to_string()}</span>
          </label>
        </div>
        <p style="color:#b00;">{move || error.get()}</p>
        <ul style="list-style:none;padding:0;display:flex;flex-direction:column;gap:1rem;">
          <For
            each=move || hits.get()
            key=|hit| hit.id
            children=move |cx, hit| {
              view! { cx,
                <li style="border:1px solid #ddd;border-radius:8px;padding:12px;">
                  <div style="display:flex;justify-content:space-between;gap:1rem;">
                    <h3 style="margin:0;">{hit.review.review_title.clone()}</h3>
                    <span style="color:#555;">{format!("{:.4}", hit.score)}</span>
                  </div>
                  <p style="margin:.5rem 0 0;color:#333;">
                    { hit.review.review_body.chars().take(160).collect::<String>() }
                     </p>
                  <small style="color:#666;">{"ID: "}{hit.id}</small>
                </li>
              }
            }
          />
        </ul>
        <hr />
        <A href="/">"← Back"</A>
      </div>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    console_error_panic_hook::set_once();
    leptos::mount_to_body(|cx| {
        provide_meta_context(cx);
        view! { cx,
          <Router>
            <Routes>
              <Route path="" view=move |cx| view! { cx, <IndexPage/> }/>
              <Route path="/search" view=move |cx| view! { cx, <SearchPage/> }/>
              <Route path="*" view=move |cx| view! { cx, <p>"Not found"</p> }/>
            </Routes>
          </Router>
        }
    });
}
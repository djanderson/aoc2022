use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::Path2d;

const PIXEL_RATIO: f64 = 5.0;
const HALF_PIXEL_RATIO: f64 = PIXEL_RATIO / 2.0;
const CANVAS_WIDTH: f64 = 800.0;
const CANVAS_HEIGHT: f64 = 800.0;
const SIM_CENTER: f64 = 500.0; // simulation centered around x == 500.0

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context(cx);

    view! { cx,
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        <Meta name="color-scheme" content="dark"/>

        <Title text="AOC 2022 - Day 14"/>

        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn request_animation_frame(f: &Closure<dyn FnMut()>) {
    window()
        .request_animation_frame(f.as_ref().unchecked_ref())
        .expect("should register `requestAnimationFrame` OK");
}

fn simulate(
    rocks: Path2d,
    canvas: HtmlElement<html::Canvas>,
    incr_sand_count: impl Fn() + 'static,
) -> Result<(), JsValue> {
    // https://rustwasm.github.io/docs/wasm-bindgen/examples/request-animation-frame.html
    let f = Rc::new(RefCell::new(None));
    let g = f.clone();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    // Colors
    let slate_gray = JsValue::from_str("rgb(112, 128, 144)");
    let sandy_brown = JsValue::from_str("rgb(244, 164, 96)");

    let x = CANVAS_WIDTH / 2.0;
    let y = 0.0;
    let mut x_offset = 0.0;
    let mut y_offset = 0.0;

    let mut resting_sand = web_sys::Path2d::new()?;
    let falling_sand = web_sys::Path2d::new()?;
    falling_sand.rect(x, y, PIXEL_RATIO, PIXEL_RATIO);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let hit = |x: f64, y: f64| -> bool {
            ctx.is_point_in_path_with_path_2d_and_f64(&rocks, x, y)
                || ctx.is_point_in_path_with_path_2d_and_f64(&resting_sand, x, y)
        };

        // If we hit the bottom edge of the canvas, we're falling into the abyss
        let abyss = y + y_offset + PIXEL_RATIO + HALF_PIXEL_RATIO > CANVAS_HEIGHT;
        if abyss {
            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        // Check straight down
        if hit(
            x + x_offset + HALF_PIXEL_RATIO,
            y + y_offset + PIXEL_RATIO + HALF_PIXEL_RATIO,
        ) {
            // Check down-left
            if hit(
                x + x_offset - HALF_PIXEL_RATIO,
                y + y_offset + PIXEL_RATIO + HALF_PIXEL_RATIO,
            ) {
                // Check down-right
                if hit(
                    x + x_offset + PIXEL_RATIO + HALF_PIXEL_RATIO,
                    y + y_offset + PIXEL_RATIO + HALF_PIXEL_RATIO,
                ) {
                    // Add unit to resting sand
                    incr_sand_count();
                    resting_sand = web_sys::Path2d::new_with_other(&resting_sand).unwrap();
                    resting_sand.rect(x + x_offset, y + y_offset, PIXEL_RATIO, PIXEL_RATIO);
                    y_offset = 0.0;
                    x_offset = 0.0;
                } else {
                    // Fall down-right
                    x_offset += PIXEL_RATIO;
                    y_offset += PIXEL_RATIO;
                }
            } else {
                // Fall down-left
                x_offset -= PIXEL_RATIO;
                y_offset += PIXEL_RATIO;
            }
        } else {
            // Fall straight down
            y_offset += PIXEL_RATIO;
        }

        ctx.clear_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);
        ctx.set_fill_style(&slate_gray);
        ctx.fill_with_path_2d(&rocks);
        ctx.set_fill_style(&sandy_brown);
        ctx.fill_with_path_2d(&resting_sand);
        ctx.save();
        ctx.begin_path();
        _ = ctx.translate(x_offset, y_offset);
        ctx.set_fill_style(&sandy_brown);
        ctx.fill_with_path_2d(&falling_sand);
        ctx.close_path();
        ctx.restore();

        // Schedule ourself for another requestAnimationFrame callback.
        request_animation_frame(f.borrow().as_ref().unwrap());
    }) as Box<dyn FnMut()>));

    request_animation_frame(g.borrow().as_ref().unwrap());

    Ok(())
}

#[server(MyServerFnType, "/api", "GetJson", "input")]
pub async fn get_input() -> Result<String, ServerFnError> {
    use std::fs;
    use std::path::Path;
    let conf = get_configuration(None).await.unwrap();
    let leptos_options = &conf.leptos_options;
    let site_root = &leptos_options.site_root;
    let path = format!("{site_root}/input.txt");
    let p = Path::new(&path);

    Ok(fs::read_to_string(p)?)
}

/// Renders the home page of your application.
#[component]
fn HomePage(cx: Scope) -> impl IntoView {
    let board_canvas_ref = create_node_ref::<html::Canvas>(cx);
    let (count, set_count) = create_signal(cx, 0);
    let incr_sand_count = move || set_count.update(|n| *n += 1);

    // Coordinates are centered around SIM_CENTER in the given coordinate system.
    // We need to scale by the pixel ratio and recenter in the middle of the canvas.
    let transform = |coords: (f64, f64)| -> (f64, f64) {
        let x = ((coords.0 - SIM_CENTER) * PIXEL_RATIO) + (CANVAS_WIDTH / 2.0);
        let y = coords.1 * PIXEL_RATIO;
        (x, y)
    };

    board_canvas_ref.on_load(cx, move |canvas: HtmlElement<html::Canvas>| {
        canvas.set_width(CANVAS_WIDTH as u32);
        canvas.set_height(CANVAS_HEIGHT as u32);
        spawn_local(async move {
            let input = get_input().await.unwrap();

            // Build rock path
            let mut rocks = web_sys::Path2d::new().unwrap();
            for line in input.lines() {
                rocks = web_sys::Path2d::new_with_other(&rocks).unwrap();
                let coords: Vec<(f64, f64)> = line
                    .split(" -> ")
                    .map(|xy| {
                        let (x, y) = xy.split_once(',').unwrap();
                        (x.parse().unwrap(), y.parse().unwrap())
                    })
                    .collect();
                let mut coord_pairs = coords.windows(2);
                while let Some(&[p1, p2]) = coord_pairs.next() {
                    let (x1, y1) = transform(p1);
                    let (x2, y2) = transform(p2);
                    let x = x1.min(x2);
                    let y = y1.min(y2);
                    let width = f64::abs(x2 - x1) + PIXEL_RATIO;
                    let height = f64::abs(y2 - y1) + PIXEL_RATIO;
                    rocks.rect(x, y, width, height);
                }
            }

            _ = simulate(rocks, canvas, incr_sand_count);
        })
    });

    view! { cx,
        <h1>"AOC 2022 - Day 14"</h1>
        <p>Count: {count}</p>
        <canvas id="board" _ref=board_canvas_ref width="{CANVAS_WIDTH}" height="{CANVAS_HEIGHT}"></canvas>
    }
}

/// 404 - Not Found
#[component]
fn NotFound(cx: Scope) -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>(cx);
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! { cx,
        <h1>"Not Found"</h1>
    }
}

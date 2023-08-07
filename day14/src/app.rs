use bitvec::prelude::*;
use leptos::*;
use leptos_meta::{provide_meta_context, Meta, Stylesheet, Title};
use leptos_router::{Route, Router, Routes};
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::Path2d;

const PIXEL_RATIO: f64 = 4.0;
const CANVAS_WIDTH: f64 = 1400.0;
const CANVAS_HEIGHT: f64 = 700.0;
const CANVAS_PIXEL_WIDTH: usize = (CANVAS_WIDTH / PIXEL_RATIO) as usize;
const CANVAS_PIXEL_HEIGHT: usize = (CANVAS_HEIGHT / PIXEL_RATIO) as usize;
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
    mut hit_region: BitBox,
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

    const X_START: usize = CANVAS_PIXEL_WIDTH / 2;
    const Y_START: usize = 0;
    let mut x = X_START;
    let mut y = Y_START;

    let mut blocked: bool = false;
    let resting_sand = web_sys::Path2d::new()?;
    let falling_sand = web_sys::Path2d::new()?;

    falling_sand.rect(x as f64 * PIXEL_RATIO, y as f64 * PIXEL_RATIO, PIXEL_RATIO, PIXEL_RATIO);

    *g.borrow_mut() = Some(Closure::wrap(Box::new(move || {
        let hit = |x: usize, y: usize| -> bool {
            let pixel = y * CANVAS_PIXEL_WIDTH + x;
            //log!("hit pixel: {}, x: {}, y: {}: {}", pixel, x, y, hit_region[pixel]);
            if y > CANVAS_PIXEL_HEIGHT {
                panic!("y out of bounds");
            }
            hit_region[pixel]
        };

        // Draw canvas
        ctx.clear_rect(0.0, 0.0, CANVAS_WIDTH, CANVAS_HEIGHT);
        ctx.set_fill_style(&slate_gray);
        ctx.fill_with_path_2d(&rocks);
        ctx.set_fill_style(&sandy_brown);
        ctx.fill_with_path_2d(&resting_sand);
        ctx.save();
        ctx.begin_path();
        _ = ctx.translate(x as f64 * PIXEL_RATIO, y as f64 * PIXEL_RATIO);
        ctx.set_fill_style(&sandy_brown);
        ctx.fill_with_path_2d(&falling_sand);
        ctx.close_path();
        ctx.restore();

        if blocked {
            // Drop our handle to this closure so that it will get cleaned
            // up once we return.
            let _ = f.borrow_mut().take();
            return;
        }

        loop {
            // Check straight down
            if hit(x, y + 1) {
                // Check down-left
                if hit(x - 1, y + 1) {
                    // Check down-right
                    if hit(x + 1, y + 1) {
                        // Add unit to resting sand
                        incr_sand_count();
                        let canvas_x = x as f64 * PIXEL_RATIO;
                        let canvas_y = y as f64 * PIXEL_RATIO;
                        resting_sand.rect(canvas_x, canvas_y, PIXEL_RATIO, PIXEL_RATIO);

                        let pixel = y * CANVAS_PIXEL_WIDTH + x;
                        hit_region.set(pixel, true);

                        blocked = x == X_START && y == Y_START;
                        y = Y_START;
                        x = X_START;

                        break; // break to request new animation frame
                    } else {
                        // Fall down-right
                        x += 1;
                        y += 1;
                    }
                } else {
                    // Fall down-left
                    x -= 1;
                    y += 1;
                }
            } else {
                // Fall straight down
                y += 1;
            }

        }
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
    const X_SHIFT: isize = -(SIM_CENTER as isize) + (CANVAS_PIXEL_WIDTH as isize) / 2;

    board_canvas_ref.on_load(cx, move |canvas: HtmlElement<html::Canvas>| {
        canvas.set_width(CANVAS_WIDTH as u32);
        canvas.set_height(CANVAS_HEIGHT as u32);
        spawn_local(async move {
            let input = get_input().await.unwrap();

            // Build rock path
            let rocks = web_sys::Path2d::new().unwrap();
            let mut hit_region = bitbox![0; CANVAS_PIXEL_WIDTH * CANVAS_PIXEL_HEIGHT];

            let mut floor = 0;

            for line in input.lines() {
                let coords: Vec<(isize, isize)> = line
                    .split(" -> ")
                    .map(|xy| {
                        let (x, y) = xy.split_once(',').unwrap();
                        (x.parse().unwrap(), y.parse().unwrap())
                    })
                    .collect();
                let mut coord_pairs = coords.windows(2);
                while let Some(&[p1, p2]) = coord_pairs.next() {
                    let (mut x1, y1) = p1;
                    x1 += X_SHIFT;
                    let (mut x2, y2) = p2;
                    x2 += X_SHIFT;
                    let x = x1.min(x2);
                    let y = y1.min(y2);
                    floor = floor.max(y1.max(y2));
                    let width = x1.abs_diff(x2) + 1;
                    let height = y1.abs_diff(y2) + 1;
                    if height == 1 {
                        let offset = y as usize * CANVAS_PIXEL_WIDTH + x as usize;
                        for i in 0..width {
                            hit_region.set(offset + i, true);
                        }
                    } else {
                        for i in 0..height {
                            hit_region.set((y as usize + i) * CANVAS_PIXEL_WIDTH + x as usize, true);
                        }
                    }
                    rocks.rect(
                        x as f64 * PIXEL_RATIO,
                        y as f64 * PIXEL_RATIO,
                        width as f64 * PIXEL_RATIO,
                        height as f64 * PIXEL_RATIO,
                    );
                }
            }

            floor += 2;

            // Draw Part 2 floor
            for i in 0..CANVAS_PIXEL_WIDTH {
                hit_region.set(floor as usize * CANVAS_PIXEL_WIDTH + i, true);
            }
            rocks.rect(0.0, floor as f64 * PIXEL_RATIO, CANVAS_WIDTH, PIXEL_RATIO);

            _ = simulate(rocks, hit_region, canvas, incr_sand_count);
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

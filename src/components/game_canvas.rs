use leptos::*;
use web_sys::{CanvasRenderingContext2d, MouseEvent};
use wasm_bindgen::JsCast;
use crate::game::{Universe, Cell};

const CELL_SIZE: f64 = 6.0;

#[component]
pub fn GameCanvas() -> impl IntoView {
    let width = 80;
    let height = 60;

    // Leptos Signals (狀態管理)
    let (universe, set_universe) = create_signal(Universe::new(width, height));
    let (is_running, set_is_running) = create_signal(true);
    let (speed_ms, set_speed_ms) = create_signal(50u64);

    let canvas_ref = create_node_ref::<html::Canvas>();

    // Canvas 繪圖 logic - 抽出純繪圖邏輯
    let render_to_canvas = move |uni: &Universe| {
        if let Some(canvas) = canvas_ref.get() {
            let ctx = canvas
                .get_context("2d")
                .unwrap()
                .unwrap()
                .dyn_into::<CanvasRenderingContext2d>()
                .unwrap();

            let c_width = canvas.width() as f64;
            let c_height = canvas.height() as f64;

            // 清空畫布 (背景色)
            ctx.set_fill_style_str("#1a1a1a");
            ctx.fill_rect(0.0, 0.0, c_width, c_height);

            // 繪製活細胞
            ctx.set_fill_style_str("#00FF66");
            for row in 0..uni.height() {
                for col in 0..uni.width() {
                    let idx = uni.get_index(row, col);
                    if uni.cells()[idx] == Cell::Alive {
                        ctx.fill_rect(
                            col as f64 * (CELL_SIZE + 1.0) + 1.0,
                            row as f64 * (CELL_SIZE + 1.0) + 1.0,
                            CELL_SIZE,
                            CELL_SIZE,
                        );
                    }
                }
            }
        }
    };

    // ✅ 自動響應繪圖：只要 universe 變更，就會自動重新繪製 Canvas！
    create_effect(move |_| {
        let uni = universe.get(); // 在 Effect 內部讀取 Signal，建立正確的追蹤關係
        render_to_canvas(&uni);
    });

    // 處理 Canvas 滑鼠點擊切換狀態
    let handle_canvas_click = move |e: MouseEvent| {
        if let Some(canvas) = canvas_ref.get() {
            let rect = canvas.get_bounding_client_rect();
            let click_x = e.client_x() as f64 - rect.left();
            let click_y = e.client_y() as f64 - rect.top();

            let col = (click_x / (CELL_SIZE + 1.0)).floor() as u32;
            let row = (click_y / (CELL_SIZE + 1.0)).floor() as u32;

            if row < height && col < width {
                set_universe.update(|u| u.toggle_cell(row, col));
                // 不需要手動呼叫 draw()，Effect 會自動抓到 universe 的改變！
            }
        }
    };

    // 動畫計時器 Effect
    create_effect(move |_| {
        if is_running.get() {
            let interval_ms = speed_ms.get();
            let handle = gloo_timers::callback::Interval::new(interval_ms as u32, move || {
                set_universe.update(|u| u.tick());
                // 不需要手動呼叫 draw()
            });
            on_cleanup(move || drop(handle));
        }
    });

    let canvas_w = ((CELL_SIZE + 1.0) * width as f64 + 1.0) as u32;
    let canvas_h = ((CELL_SIZE + 1.0) * height as f64 + 1.0) as u32;

    view! {
        <div style="display: flex; flex-direction: column; align-items: center; padding: 20px;">
            <h2>"Leptos + WebAssembly - Conway's Game of Life"</h2>

            // 狀態與工具列
            <div style="display: flex; gap: 15px; align-items: center; margin-bottom: 15px;">
                <button on:click=move |_| set_is_running.update(|r| *r = !*r)>
                    {move || if is_running.get() { "Pause ⏸" } else { "Play ▶" }}
                </button>

                <button 
                    disabled=move || is_running.get()
                    on:click=move |_| set_universe.update(|u| u.tick())
                >
                    "Step ⏭"
                </button>

                <button on:click=move |_| set_universe.update(|u| u.random_fill())>
                    "Randomize 🔀"
                </button>

                <button on:click=move |_| set_universe.update(|u| u.clear())>
                    "Clear 🗑"
                </button>

                <label style="display: flex; align-items: center; gap: 5px;">
                    "Interval: " {move || speed_ms.get()} "ms"
                    <input 
                        type="range" min="10" max="200" step="5"
                        value=move || speed_ms.get()
                        on:input=move |e| {
                            if let Ok(val) = event_target_value(&e).parse::<u64>() {
                                set_speed_ms.set(val);
                            }
                        }
                    />
                </label>
            </div>

            // 統計數據
            <div style="display: flex; gap: 20px; margin-bottom: 15px; color: #aaa;">
                <div>"Generation: " <strong>{move || universe.get().generation()}</strong></div>
                <div>"Alive Cells: " <strong>{move || universe.get().live_count()}</strong></div>
            </div>

            // 遊戲 Canvas
            <canvas
                node_ref=canvas_ref
                on:click=handle_canvas_click
                width=canvas_w
                height=canvas_h
                style="border: 1px solid #333; cursor: pointer; border-radius: 4px;"
            />
        </div>
    }
}
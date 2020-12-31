use visioncortex::{ColorImage, PointI32, color_clusters::Runner};
use wasm_bindgen::prelude::*;
use web_sys::{CanvasRenderingContext2d, console};

use crate::{canvas::Canvas, symbol::{Symbol, SymbolType}};

use super::ScanResult;

#[wasm_bindgen]
pub struct RawScanner {}

#[wasm_bindgen]
impl RawScanner {
    /// Initiate scanning, should return whatever info is needed for decoding
    pub fn scan_from_canvas_id(canvas_id: &str) -> String {
        let canvas = &Canvas::new_from_id(canvas_id);
        let clusters = Self::extract_symbol_candidates(
            canvas.get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32)
        );
        let symbols = Self::filter_and_categorize_symbols(clusters, canvas);

        "Finished".into()
    }
}

impl RawScanner {

    fn extract_symbol_candidates(frame: ColorImage) -> Vec<Symbol> {
        // Color clustering requires the use of a Runner (it is taken after run())
        let mut runner = Runner::default();
        runner.init(frame);

        let clusters = runner.run(); // Performing clustering
        let view = clusters.view(); // Obtain the ClustersView (parent of clusters)
        
        view.clusters_output.iter()
            .map(|&cluster_index| view.get_cluster(cluster_index))
            .map(|cluster| {
                //console::log_2(&cluster.rect.center().x.into(), &cluster.rect.center().y.into());
                Symbol {
                    color: cluster.color().to_hsv(),
                    shape: cluster.to_shape(&view),
                    top_left: PointI32 {x: cluster.rect.left, y: cluster.rect.top},
                    bot_right: PointI32 {x: cluster.rect.right, y: cluster.rect.bottom},
                }
            })
            .collect()
    }

    /// Keep only those which look like a symbol.
    ///
    /// Locate the Finder patterns and seperate them from the data-carrying glyphs.
    fn filter_and_categorize_symbols(candidates: Vec<Symbol>, canvas: &Canvas) -> ScanResult {
        let mut result = ScanResult { finders: vec![], glyphs: vec![] };

        for candidate in candidates.into_iter() {
            match candidate.categorize() {
                SymbolType::Finder => result.finders.push(candidate),
                SymbolType::Glyph => result.glyphs.push(candidate),
                SymbolType::Invalid => {}, // Thrown away
            }
        }

        Self::render_symbols(canvas, &result);

        result
    }

    fn render_symbols(canvas: &Canvas, scan_result: &ScanResult) {
        let ctx = canvas.get_rendering_context_2d();
        for finder in scan_result.finders.iter() {
            ctx.set_stroke_style(JsValue::from_str(
                "rgb(255, 0, 0)"
            ).as_ref());
            let x1 = finder.top_left.x as f64;
            let y1 = finder.top_left.y as f64;
            let x2 = finder.bot_right.x as f64;
            let y2 = finder.bot_right.y as f64;
            ctx.begin_path();
            ctx.move_to(x1, y1);
            ctx.line_to(x1, y2);
            ctx.line_to(x2, y2);
            ctx.line_to(x2, y1);
            ctx.line_to(x1, y1);
            ctx.stroke();
        }
    }
}
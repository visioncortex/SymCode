use std::iter::Filter;

use visioncortex::{ColorImage, color_clusters::Runner};
use wasm_bindgen::prelude::*;

use crate::{canvas::Canvas, utils::render_color_image_to_canvas};

use super::{FinderCandidate, TransformFitter, Transformer};

#[wasm_bindgen]
pub struct RawScanner {}

#[wasm_bindgen]
impl RawScanner {
    /// Initiate scanning, should return whatever info is needed for decoding
    pub fn scan_from_canvas_id(canvas_id: &str, debug_canvas_id: &str, transform_error_threshold: f64) -> String {
        let canvas = &Canvas::new_from_id(canvas_id);
        let debug_canvas = &Canvas::new_from_id(debug_canvas_id);

        let raw_frame = canvas.get_image_data_as_color_image(0, 0, canvas.width() as u32, canvas.height() as u32);
        let filter_candidates = Self::extract_finder_candidates(
            &raw_frame,
            canvas,
            debug_canvas
        );
        if let Some(transform) = TransformFitter::from_scan_result(filter_candidates, transform_error_threshold) {
            transform.print_coeffs()
        } else {
            "No candidates are good enough".into()
        }
    }
}

impl RawScanner {
    /// Extract the Finder patterns.
    ///
    /// Decision is made based on the colors and shapes of each candidate.
    fn extract_finder_candidates(frame: &ColorImage, canvas: &Canvas, debug_canvas: &Canvas) -> Vec<FinderCandidate> {
        // Color clustering requires the use of a Runner (it is taken after run())
        let mut runner = Runner::default();
        runner.init(frame.clone());

        let clusters = runner.run(); // Performing clustering
        let view = clusters.view(); // Obtain the ClustersView (parent of clusters)

        let render_result = render_color_image_to_canvas(view.to_color_image(), debug_canvas); // Possibly unhandled exception
        
        let finder_candidates: Vec<FinderCandidate> = view.clusters_output.iter()
            .map(|&cluster_index| FinderCandidate::from_cluster_and_view(view.get_cluster(cluster_index), &view))
            .filter(|option| option.is_some())
            .map(|option| option.unwrap())
            .collect();
        
        Self::render_finder_candidates(canvas, &finder_candidates);

        finder_candidates
    }

    fn render_finder_candidates(canvas: &Canvas, finder_candidates: &[FinderCandidate]) {
        let ctx = canvas.get_rendering_context_2d();
        for finder in finder_candidates.into_iter() {
            ctx.set_stroke_style(JsValue::from_str(
                "rgb(255, 0, 0)"
            ).as_ref());
            let rect = &finder.rect;
            let x1 = rect.left as f64;
            let y1 = rect.top as f64;
            let x2 = rect.right as f64;
            let y2 = rect.bottom as f64;
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
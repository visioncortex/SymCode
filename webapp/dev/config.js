export default {
    SYMCODE_CONFIG: {
        code_width: 400,
        code_height: 400,
        symbol_width: 80,
        symbol_height: 80,
        finder_positions: [
            {x: 200.0, y: 80.0},
            {x: 200.0, y: 200.0},
            {x: 80.0, y: 320.0},
            {x: 320.0, y: 320.0},
        ],
        glyph_anchors: [
            {x: 40.0, y: 40.0},
            {x: 40.0, y: 160.0},
            {x: 160.0, y: 280.0},
            {x: 280.0, y: 160.0},
            {x: 280.0, y: 40.0},
        ],
        canvas: "frame",
        debug_canvas: "debug",
        max_extra_finder_candidates: 3,
        rectify_error_threshold: 20.0,
        stat_tolerance: 0.2,
        max_encoding_difference: 1,
        empty_cluster_threshold: 0.2,
    },
    
    ALPHABET_CONFIG: {
        top_left: {x: 49, y: 49},
        symbol_width: 80,
        symbol_height: 80,
        offset_x: 115,
        offset_y: 116,
        num_columns: 4,
        num_rows: 8,
    }
};
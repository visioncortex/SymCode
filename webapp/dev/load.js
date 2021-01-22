import { AlphabetReaderParams } from "symcode";
import { ALPHABET_CONFIG } from "./config";
import { loadingCompletes } from "./index";

export const loadBuffer = document.getElementById('loadBuffer');
const loadBufferCtx = loadBuffer.getContext('2d');
const img = new Image();

export function loadAllTemplates() {
    loadTemplateByIndex(1);
}

export function loadTemplateByIndex(index) {
    if (index > numTemplates) {
        loadingCompletes();
        return;
    }
    const path = "assets/glyph_templates/" + index + ".jpg";
    img.src = path;
    img.onload = function () {
        loadBuffer.width = img.naturalWidth;
        loadBuffer.height = img.naturalHeight;

        loadBufferCtx.clearRect(0, 0, loadBuffer.width, loadBuffer.height);
        loadBufferCtx.drawImage(img, 0, 0);

        scanner.load_template_from_canvas_id('loadBuffer');

        loadTemplateByIndex(index + 1);
    };
}

export function loadAlphabet(scanner) {
    const path = "assets/alphabet/alphabet2.jpg";
    const params = AlphabetReaderParams.from_json_string(JSON.stringify(ALPHABET_CONFIG));
    img.src = path;
    img.onload = function () {
        loadBuffer.width = img.naturalWidth;
        loadBuffer.height = img.naturalHeight;

        loadBufferCtx.clearRect(0, 0, loadBuffer.width, loadBuffer.height);
        loadBufferCtx.drawImage(img, 0, 0);

        scanner.load_alphabet_from_canvas_id('loadBuffer', params);

        loadingCompletes();
    };
}


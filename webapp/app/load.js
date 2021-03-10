import { loadingCompletes } from "./index";

export const loadBuffer = document.getElementById('loadBuffer');
const loadBufferCtx = loadBuffer.getContext('2d');
const img = new Image();

export function loadAlphabet(scanner) {
    const path = "assets/alphabet/alphabet2.png";
    img.src = path;
    img.onload = function () {
        loadBuffer.width = img.naturalWidth;
        loadBuffer.height = img.naturalHeight;

        loadBufferCtx.clearRect(0, 0, loadBuffer.width, loadBuffer.height);
        loadBufferCtx.drawImage(img, 0, 0);

        scanner.load_alphabet_from_canvas_id('loadBuffer');

        loadingCompletes();
    };
}


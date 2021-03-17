import { loadingCompletes } from "./app";

export const loadBuffer = document.getElementById('loadBuffer');
const loadBufferCtx = loadBuffer.getContext('2d');
const img = new Image();

export function loadAlphabet() {
    return new Promise((resolve) => {
        const path = "assets/alphabet/alphabet2.png";
        img.src = path;
        img.onload = function () {
            loadBuffer.width = img.naturalWidth;
            loadBuffer.height = img.naturalHeight;

            loadBufferCtx.clearRect(0, 0, loadBuffer.width, loadBuffer.height);
            loadBufferCtx.drawImage(img, 0, 0);

            loadingCompletes();
            resolve();
        };
    });
}


import { RawScanner } from "symcode";

const canvas = document.getElementById('frame');
const ctx = canvas.getContext('2d');
const img = new Image();
document.getElementById('imageInput').addEventListener('change', function (e) {
    const file = this.files[0];
    img.src = URL.createObjectURL(file);
    img.onload = function () {
        canvas.width = img.naturalWidth;
        canvas.height = img.naturalHeight;
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.drawImage(img, 0, 0);

        const frame = ctx.getImageData(0, 0, canvas.width, canvas.height);
        scanForFinders(frame);
    };
});

function scanForFinders(frame) {
    console.log(RawScanner.scan_from_canvas_id('frame'));
}
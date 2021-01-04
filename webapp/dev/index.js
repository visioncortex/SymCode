import { RawScanner } from "symcode";

const canvas = document.getElementById('frame');
const debugCanvas = document.getElementById('debug');
const ctx = canvas.getContext('2d');
const debugCtx = debugCanvas.getContext('2d');
const img = new Image();
document.getElementById('imageInput').addEventListener('change', function (e) { loadImageFromSource(this.files[0]) });

//document.addEventListener('load', loadImageFromSource('./assets/sample_code_with_noise.jpg'));

function loadImageFromSource(source) {
    img.src = source instanceof File ? URL.createObjectURL(source) : source;
    img.onload = function () {
        canvas.width = img.naturalWidth;
        canvas.height = img.naturalHeight;

        debugCanvas.width = canvas.width;
        debugCanvas.height = canvas.height;

        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.drawImage(img, 0, 0);
        scanForFinders();
    };
}

function scanForFinders() {
    console.log(RawScanner.scan_from_canvas_id('frame', 'debug', 15));
}
import { RawScanner } from "symcode";

const scanner = RawScanner.new();
const canvas = document.getElementById('frame');
const loadBuffer = document.getElementById('loadBuffer');
const loadBufferCtx = loadBuffer.getContext('2d');
const debugCanvas = document.getElementById('debug');
const ctx = canvas.getContext('2d');
const debugCtx = debugCanvas.getContext('2d');
const img = new Image();
const numTemplates = 4;
document.getElementById('imageInput').addEventListener('change', function (e) { scanImageFromSource(this.files[0]) });

document.addEventListener('load', loadAlphabet());

function scanImageFromSource(source) {
    img.src = source instanceof File ? URL.createObjectURL(source) : source;
    img.onload = function () {
        canvas.width = img.naturalWidth;
        canvas.height = img.naturalHeight;

        debugCanvas.width = canvas.width;
        debugCanvas.height = canvas.height;

        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.drawImage(img, 0, 0);
        let startTime = new Date();
        scan();
        console.log("Scanning finishes in " + (new Date() - startTime) / 1000 + " seconds.");
    };
}

function scan() {
    console.log(scanner.scan_from_canvas_id('frame', 'debug', 20, 15));
}

function loadAllTemplates() {
    loadTemplateByIndex(1);
}

function loadTemplateByIndex(index) {
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

function loadAlphabet() {
    const path = "assets/alphabet.jpg";
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

function loadingCompletes() {
    console.log("Template loading completes.");
    scanImageFromSource("assets/camera_inputs/test_prototype_3/0.jpg");
}
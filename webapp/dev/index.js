import { SymcodeScanner, SymcodeScannerConfig, AlphabetReaderParams } from "symcode";

const scanner = SymcodeScanner.new();
const canvas = document.getElementById('frame');
const loadBuffer = document.getElementById('loadBuffer');
const loadBufferCtx = loadBuffer.getContext('2d');
const debugCanvas = document.getElementById('debug');
const ctx = canvas.getContext('2d');
const camera = document.getElementById('camera');
const cameraButton = document.getElementById('cameraButton');
const img = new Image();
const numTemplates = 4;

let debugging = true;
let finishScanning = false;

const inputFrameSize = {
    width: 350,
    height: 350,
};
const fps = 60;

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
        scan_from_canvas('frame');
    };
}

// Returns true if a Symcode is recognized and decoded
function scan_from_canvas(canvas_id) {
    return new Promise((resolve) => {
        let startTime = new Date();
        let config = SymcodeScannerConfig.from_canvas_id(canvas_id);
        if (debugging) {
            config = config.debug_canvas('debug');
        }
        const result = scanner.scan_with_config(config);
        console.log(result);
        console.log("Scanning finishes in " + (new Date() - startTime) / 1000 + " seconds.");
        resolve(result.localeCompare("Success") == 0);
    });
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
    const params = AlphabetReaderParams.new()
        .top_left(53, 53)
        .glyph_size(80, 80)
        .offset(111, 112)
        .matrix_size(4, 4)
    ;
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

function loadingCompletes() {
    console.log("Template loading completes.");
    scanImageFromSource("assets/streaming_test/test1.png");
}

const constraints = {
    video: { width: {min: 720}, height: {min: 720} },
};

function stopCamera() {
    const stream = camera.srcObject;
    stream.getTracks().forEach(function(track) {
        track.stop();
    });
    camera.srcObject = null;
}

cameraButton.onclick = function() {
    navigator.mediaDevices
        .getUserMedia(constraints)
        .then(handleSuccess)
        .catch(handleError);
}

function handleSuccess(stream) {
    //camera.style.display = "block";
    camera.srcObject = stream;
    getCameraVideoDimensions()
        .then(({width, height}) => {
            startStreaming(width, height);
        });
}

function handleError(error) {
    console.error("Error: ", error);
}

function getCameraVideoDimensions() {
    return new Promise(function(resolve) {
        camera.addEventListener("loadedmetadata", function () {
            let width = this.videoWidth;
            let height = this.videoHeight;
            resolve({
                width: width,
                height: height,
            });
        }, false);
    });
}

function startStreaming(videoWidth, videoHeight) {
    console.log("Start streaming");
    console.log(videoWidth + " " + videoHeight);
    const sx = (videoWidth - inputFrameSize.width) / 2;
    const sy = (videoHeight - inputFrameSize.height) / 2;

    drawFrame(sx, sy);
}

function drawFrame(sx, sy) {
    canvas.width = inputFrameSize.width;
    canvas.height = inputFrameSize.height;
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.drawImage(camera, sx, sy, inputFrameSize.width, inputFrameSize.height,
                                        0, 0, canvas.width, canvas.height);
    scan_from_canvas('frame')
        .then((successful) => {
            if (!successful) {
                sleep(1/fps)
                    .then(() => drawFrame(sx, sy))
            } else {
                stopCamera();
                return;
            }
        });
}

function sleep(s) {
    const ms = s*1000;
    return new Promise(resolve => setTimeout(resolve, ms));
}
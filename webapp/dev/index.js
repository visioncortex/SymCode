import { RawScanner, RawScannerConfig, AlphabetReaderParams } from "symcode";

const scanner = RawScanner.new();
const canvas = document.getElementById('frame');
const loadBuffer = document.getElementById('loadBuffer');
const loadBufferCtx = loadBuffer.getContext('2d');
const debugCanvas = document.getElementById('debug');
const ctx = canvas.getContext('2d');
const camera = document.getElementById('camera');
const cameraButton = document.getElementById('cameraButton');
const cameraInputBuffer = document.getElementById('cameraInputBuffer');
const cameraInputBufferCtx = cameraInputBuffer.getContext('2d');
const img = new Image();
const numTemplates = 4;

const inputFrameSize = {
    width: 720,
    height: 720,
};
const fps = 30;

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
    const config = RawScannerConfig.from_canvas_id('frame')
        .debug_canvas('debug')
    ;
    console.log(scanner.scan_with_config(config));
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
    scanImageFromSource("assets/camera_inputs/test_prototype_3/0.jpg");
}

const constraints = {
    video: { width: {min: 720}, height: {min: 720} },
};

cameraButton.onclick = function() {
    navigator.mediaDevices
        .getUserMedia(constraints)
        .then(handleSuccess)
        .catch(handleError);
}

function handleSuccess(stream) {
    camera.style.display = "block";
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
    cameraInputBuffer.width = inputFrameSize.width;
    cameraInputBuffer.height = inputFrameSize.height;
    const dx = (videoWidth - inputFrameSize.width) / 2;
    const dy = (videoHeight - inputFrameSize.height) / 2;

    drawFrame(dx, dy);
}

function drawFrame(dx, dy) {
    cameraInputBufferCtx.clearRect(0, 0, cameraInputBuffer.width, cameraInputBuffer.height);
    cameraInputBufferCtx.drawImage(camera, dx, dy, inputFrameSize.width, inputFrameSize.height);

    sleep(1/fps)
        .then(() => drawFrame(dx, dy));
}

function sleep(s) {
    const ms = s*1000;
    return new Promise(resolve => setTimeout(resolve, ms));
}
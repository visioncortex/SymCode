import { SymcodeScanner, SymcodeConfig } from "symcode";
import { SYMCODE_CONFIG } from "./config";
import { loadAlphabet } from "./load";
import { ready_perspective_with_image_src } from "./perspective";

const canvas = document.getElementById('frame');
const debugCanvas = document.getElementById('debug');
const ctx = canvas.getContext('2d');
const camera = document.getElementById('camera');
const cameraButton = document.getElementById('cameraButton');
const img = new Image();

let debugging = true;
let finishScanning = false;

ready_perspective_with_image_src("testCanvas", "assets/prototype_4/4.png");

const scannerConfig = SymcodeConfig.from_json_string(JSON.stringify(SYMCODE_CONFIG));
const scanner = SymcodeScanner.from_config(scannerConfig);

const inputFrameSize = {
    width: 350,
    height: 350,
};
const fps = 60;

export function loadingCompletes() {
    console.log("Template loading completes.");
    scanImageFromSource("assets/prototype_4/4.png");
}

const ERROR_COLOR = "color: #ff5050;";

function handleError(e) {
    console.log("%c" + e, ERROR_COLOR);
}

const SUCCESS_COLOR = "color: #00ff00;";

function handleSuccess(msg) {
    console.log("%c" + msg, SUCCESS_COLOR);
}

document.getElementById('generate').addEventListener('click', function (e) {
    let groundTruthCode = "";
    try {
        groundTruthCode = scanner.generate_symcode_to_canvas();
    } catch (e) {
        handleError(e);
        return;
    }
    console.log("Generated code: " + groundTruthCode);
    scan()
        .then((result) => {
            console.log("Recognition result: " + result);
            if (result.localeCompare(groundTruthCode) == 0) {
                handleSuccess("Generated code is correctly recognized.");
            } else {
                handleError("Generated code is INCORRECTLY recognized.");
            }
        })
        .catch((e) => {
            handleError(e);
        });
});

document.getElementById('imageInput').addEventListener('change', function (e) { scanImageFromSource(this.files[0]) });

document.addEventListener('load', loadAlphabet(scanner));

function scanImageFromSource(source) {
    img.src = source instanceof File ? URL.createObjectURL(source) : source;
    img.onload = function () {
        canvas.width = img.naturalWidth;
        canvas.height = img.naturalHeight;

        debugCanvas.width = canvas.width;
        debugCanvas.height = canvas.height;

        ctx.clearRect(0, 0, canvas.width, canvas.height);
        ctx.drawImage(img, 0, 0);
        scan()
            .then((result) => {
                console.log("Recognition result: " + result);
            })
            .catch((e) => {
                handleError(e);
            });
    };
}

// Returns true if a Symcode is recognized and decoded
function scan() {
    return new Promise((resolve) => {
        try {
            let startTime = new Date();
            const result = scanner.scan_from_canvas_id("frame");
            handleSuccess("Scanning finishes in " + (new Date() - startTime) + " ms.");
            resolve(result);
        } catch (e) {
            throw e;
        }
    });
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
        .then(handleGetCameraSuccess)
        .catch((e) => handleError(e));
}

function handleGetCameraSuccess(stream) {
    //camera.style.display = "block";
    camera.srcObject = stream;
    getCameraVideoDimensions()
        .then(({width, height}) => {
            startStreaming(width, height);
        });
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

    finishScanning = false;
    drawFrame(sx, sy);
}

function drawFrame(sx, sy) {
    canvas.width = inputFrameSize.width;
    canvas.height = inputFrameSize.height;
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.drawImage(camera, sx, sy, inputFrameSize.width, inputFrameSize.height,
                                        0, 0, canvas.width, canvas.height);
    scan()
        .then((result) => {
            console.log("Recognition result: " + result);
            stopCamera();
            return;
        })
        .catch((e) => {
            handleError(e);
            if (!finishScanning) {
                sleep(1/fps)
                    .then(() => drawFrame(sx, sy))
            }
        });
}

function sleep(s) {
    const ms = s*1000;
    return new Promise(resolve => setTimeout(resolve, ms));
}
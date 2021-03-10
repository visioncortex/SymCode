import { Acute32SymcodeMain } from "symcode";
import { loadAlphabet } from "./load";

const wrapper = document.getElementById('coupon_wrapper');

const frameCanvas = document.getElementById('frame');
const frameCtx = frameCanvas.getContext('2d');
const originalFrameSize = [frameCanvas.width, frameCanvas.height];

const scanner = Acute32SymcodeMain.new();

export function loadingCompletes() {
    console.log("Template loading completes.");
}

document.addEventListener('load', loadAlphabet(scanner));

const ERROR_COLOR = "color: #ff5050;";

function handleError(e) {
    console.log("%c" + e, ERROR_COLOR);
}

const SUCCESS_COLOR = "color: #00ff00;";

function handleSuccess(msg) {
    console.log("%c" + msg, SUCCESS_COLOR);
}

// Returns true if a Symcode is recognized and decoded
function scan() {
    return new Promise((resolve, reject) => {
        try {
            let startTime = new Date();
            const code = scanner.scan_from_canvas_id("frame");
            const time = (new Date() - startTime);
            handleSuccess("Scanning finishes in " + time + " ms.");
            resolve({code, time});
        } catch (e) {
            reject(e);
        }
    });
}

function reset() {
    wrapper.classList.add("hidden");
    finishScanning = true;
}

//#region Camera Input
const scanButton = document.getElementById('scan');
const camera = document.getElementById('camera');

// Flag to control termination of scanning
let finishScanning = false;

const inputFrameSize = {
    width: 720,
    height: 720,
};
const fps = 60;

const mediaConstraints = {
    video: { width: {min: 720}, height: {min: 720} },
};

scanButton.onclick = () => {
    wrapper.classList.remove("hidden");
    navigator.mediaDevices
        .getUserMedia(mediaConstraints)
        .then((stream) => {
            camera.srcObject = stream;
            getCameraVideoDimensions()
                .then(({width, height}) => {
                    startStreaming(width, height);
                });
        })
        .catch(handleError);
};

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
    [frameCanvas.width, frameCanvas.height] = originalFrameSize;
    frameCtx.clearRect(0, 0, frameCanvas.width, frameCanvas.height);
    frameCtx.drawImage(camera, sx, sy, inputFrameSize.width, inputFrameSize.height,
                                        0, 0, frameCanvas.width, frameCanvas.height);
    scan()
        .then((result) => {
            console.log("Recognition result: " + result.code);
            stopCamera();
        })
        .catch((e) => {
            handleError(e);
            if (!finishScanning) {
                sleep(1/fps)
                    .then(() => drawFrame(sx, sy))
            } else {
                stopCamera();
            }
        })
}

function stopCamera() {
    const stream = camera.srcObject;
    if (stream) {
        stream.getTracks().forEach(function(track) {
            track.stop();
        });
        camera.srcObject = null;
    }
}

function sleep(s) {
    const ms = s*1000;
    return new Promise(resolve => setTimeout(resolve, ms));
}

//#endregion

//#region Upload

const uploadButton = document.getElementById('upload');
const imageInput = document.getElementById('imageInput');
uploadButton.onclick = () => imageInput.click();
imageInput.onchange = function(e) {
    wrapper.classList.remove("hidden");
    scanImageFromSource(this.files[0]);
};

function scanImageFromSource(source) {
    let img = new Image();
    img.onload = function () {
        [frameCanvas.width, frameCanvas.height] = [img.naturalWidth, img.naturalHeight];
        frameCtx.clearRect(0, 0, frameCanvas.width, frameCanvas.height);
        frameCtx.drawImage(img, 0, 0, frameCanvas.width, frameCanvas.height);
        scan()
            .then((result) => {
                console.log("Recognition result: " + result.code);
            })
            .catch((e) => {
                handleError(e);
            });
    };
    img.src = source instanceof File ? URL.createObjectURL(source) : source;
}

//#endregion
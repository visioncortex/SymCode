import { Acute32SymcodeMain } from "symcode";
import { loadAlphabet } from "./load";

const wrapper = document.getElementById('coupon_wrapper');

const frameCanvas = document.getElementById('frame');
const frameCtx = frameCanvas.getContext('2d');
const originalFrameSize = [frameCanvas.width, frameCanvas.height];

let scanner;
loadAlphabet().then(() => scanner = getNewScanner());

function getNewScanner() {
    let sc = Acute32SymcodeMain.new();
    sc.load_alphabet_from_canvas_id('loadBuffer');
    return sc;
}

export function loadingCompletes() {
    console.log("Template loading completes.");
}

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
    try {
        let startTime = new Date();
        const code = scanner.scan_from_canvas_id("frame");
        const time = (new Date() - startTime);
        handleSuccess("Scanning finishes in " + time + " ms.");
        return {code, time};
    } catch (e) {
        throw e;
    }
}

//#region Camera Input

const scanButton = document.getElementById('scan');
const camera = document.getElementById('camera');
const showFps = document.getElementById('fps');
const reticleCanvas = document.getElementById('reticle');
const reticleCtx = reticleCanvas.getContext('2d');

// Flag to control termination of scanning
let finishScanning = false;
let lastScanTime = new Date();
let scanningCount = 0;

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
                    console.log("About to call startStreaming()");
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
    const sx = (videoWidth - inputFrameSize.width) / 2;
    const sy = (videoHeight - inputFrameSize.height) / 2;

    reticleCtx.clearRect(0, 0, reticleCanvas.width, reticleCanvas.height);
    drawReticle(reticleCanvas, reticleCtx);

    finishScanning = false;
    lastScanTime = new Date();
    scanningCount = 0;
    console.log("Start streaming loop");
    function loop() {
        if ((scanningCount++) % 1000 == 0) {
            console.log("Reallocating scanner...");
            scanner.free();
            scanner = getNewScanner();
        }
        try {
            let result = drawFrame(sx, sy);
            console.log("Recognition result: " + result.code);
            finishScanning = true;
        } catch (e) {
            const currScanTime = new Date();
            const scanDuration = (currScanTime - lastScanTime) / 1000; // scanning duration in seconds
            showFps.innerHTML = Math.round(1/scanDuration);
            lastScanTime = currScanTime;

            handleError(e);
            if (scanningCount >= 2000) {
                finishScanning = true;
            } else if (!finishScanning) {
                sleep(1/fps, loop);
            }
        } finally {
            if (finishScanning) {
                stopCamera();
            }
        }
    }
    sleep(1/fps, loop);
}

function drawReticle(canvas, ctx) {
    const horiQ1 = canvas.width*0.25;
    const vertQ1 = canvas.height*0.25;
    ctx.lineWidth = 3;
    ctx.strokeStyle = "white";
    ctx.strokeRect(horiQ1, vertQ1, canvas.width/2, canvas.height/2);
}

function drawFrame(sx, sy) {
    [frameCanvas.width, frameCanvas.height] = originalFrameSize;
    frameCtx.clearRect(0, 0, frameCanvas.width, frameCanvas.height);
    frameCtx.drawImage(camera, sx, sy, inputFrameSize.width, inputFrameSize.height,
        0, 0, frameCanvas.width, frameCanvas.height);
    
    return scan();
}

function stopCamera() {
    const stream = camera.srcObject;
    if (stream) {
        stream.getTracks().forEach(function(track) {
            track.stop();
        });
        camera.srcObject = null;
        reticleCtx.clearRect(0, 0, reticleCanvas.width, reticleCanvas.height);
    }
}

function sleep(s, callback) {
    const ms = s*1000;
    setTimeout(callback, ms);
}

//#endregion

//#region Upload

const uploadButton = document.getElementById('upload');
const imageInput = document.getElementById('imageInput');
uploadButton.onclick = () => imageInput.click();
imageInput.onchange = function(e) {
    const imgSrc = this.files[0];
    finishScanning = true;
    // Wait for camera to stop
    sleep(
        1/fps,
        () => {
            wrapper.classList.remove("hidden");
            scanImageFromSource(imgSrc);
        }
    );
};

function scanImageFromSource(source) {
    let img = new Image();
    img.onload = function () {
        [frameCanvas.width, frameCanvas.height] = [img.naturalWidth, img.naturalHeight];
        frameCtx.clearRect(0, 0, frameCanvas.width, frameCanvas.height);
        frameCtx.drawImage(img, 0, 0, frameCanvas.width, frameCanvas.height);
        try {
            let result = scan();
            console.log("Recognition result: " + result.code);
        } catch (e) {
            handleError(e);
        }
    };
    img.src = source instanceof File ? URL.createObjectURL(source) : source;
}

//#endregion
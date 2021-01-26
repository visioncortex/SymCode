import { SymcodeScanner, SymcodeConfig } from "symcode";
import { SYMCODE_CONFIG } from "./config";
import { loadAlphabet, loadBuffer } from "./load";
import { generate_perspective_with_image_src } from "./perspective";

const frameCanvas = document.getElementById('frame');
const debugCanvas = document.getElementById('debug');
const ctx = frameCanvas.getContext('2d');
const camera = document.getElementById('camera');
const cameraButton = document.getElementById('cameraButton');
const img = new Image();

let finishScanning = false;

const scannerConfig = SymcodeConfig.from_json_string(JSON.stringify(SYMCODE_CONFIG));
const scanner = SymcodeScanner.from_config(scannerConfig);

const inputFrameSize = {
    width: 350,
    height: 350,
};
const fps = 60;

export function loadingCompletes() {
    console.log("Template loading completes.");
    //scanImageFromSource("assets/prototype_4/4.png");
    runNTestCases(10);
}

const ERROR_COLOR = "color: #ff5050;";

function handleError(e) {
    console.log("%c" + e, ERROR_COLOR);
}

const SUCCESS_COLOR = "color: #00ff00;";

function handleSuccess(msg) {
    console.log("%c" + msg, SUCCESS_COLOR);
}

function runOneTestCase(consoleOutput) {
    return new Promise((resolve, reject) => {
        let groundTruthCode = "";
        try {
            groundTruthCode = scanner.generate_symcode_to_canvas("loadBuffer");
        } catch (e) {
            handleError(e);
            return;
        }
        if (consoleOutput) {
            console.log("Generated code: " + groundTruthCode);
        }
        ctx.clearRect(0, 0, frameCanvas.width, frameCanvas.height);
        generate_perspective_with_image_src("frame", loadBuffer.toDataURL())
            .then(() => {
                scan()
                    .then((result) => {
                        if (consoleOutput) {
                            console.log("Recognition result: " + result);
                        }
                        if (result.localeCompare(groundTruthCode) == 0) {
                            resolve({isCorrect: true, groundTruth: groundTruthCode, recognized: result});
                        } else {
                            resolve({isCorrect: false, groundTruth: groundTruthCode, recognized: result});
                        }
                    })
                    .catch(e => { reject(e);});
            });
    });
}

async function runNTestCases(n) {
    console.log("Running ", n, " test cases...");
    let correctCases = 0;
    let resultsHtml = [
        `<tr>
            <th>Recognition result</th>
            <th>Raw Frame</th>
            <th>Rectified code image</th>
            <th>Recognized code</th>
            <th>Ground-truth code</th>
        </tr>`
    ];
    let errors = {};
    for (let i = 0; i < n; ++i) {
        let result = {};
        let msg = "";
        try {
            result = await runOneTestCase(false);
            if (result.isCorrect) {
                msg = "Correct";
                ++correctCases;
            } else {
                msg = "Recognition is Wrong.";
                errors[msg]? errors[msg]++ : errors[msg] = 1;
            }
        } catch (e) {
            msg = e;
            errors[msg]? errors[msg]++ : errors[msg] = 1;
        }
        if (!result.isCorrect) {
            resultsHtml.push(
                `<tr>
                    <th><h3 style="${ERROR_COLOR}">${msg}</h3></th>
                    <th><img src="${frameCanvas.toDataURL("image/png;base64")}" /></th>
                    <th><img src="${debugCanvas.toDataURL("image/png;base64")}" /></th>
                    <th><h4>${result.recognized}</h4></th>
                    <th><h4>${result.groundTruth}</h4></th>
                </tr>`
            );
        }
        console.log("Running " + n + " test cases: ", correctCases, " out of ", i+1, " are correct. Running Accuracy: ", correctCases / (i+1) * 100 + "%");
    }
    document.getElementById("testResults").innerHTML = resultsHtml.join("");
    console.log("Test result: ", correctCases, " out of ", n, " test cases are correctly recognized.");
    console.log("Overall accuracy: ", correctCases / n * 100 + "%")

    for (let key in errors) {
        errors[key] = {num: errors[key], rate: errors[key]/n*100 + "%"};
    }
    console.log("Errors: ", JSON.stringify(errors, null, 2));
}

document.getElementById('test').addEventListener('click', () => {
    runNTestCases(100);
});

document.getElementById('generate').addEventListener('click', () => {
    runOneTestCase(true)
        .then((isCorrect) => {
            if (isCorrect) {
                handleSuccess("Generated code is correctly recognized.");
            } else {
                handleError("Generated code is INCORRECTLY recognized.");
            }
        })
        .catch(e => {
            handleError(e);
        });
});

document.getElementById('imageInput').addEventListener('change', function (e) { scanImageFromSource(this.files[0]) });

document.addEventListener('load', loadAlphabet(scanner));

function scanImageFromSource(source) {
    img.onload = function () {
        frameCanvas.width = img.naturalWidth;
        frameCanvas.height = img.naturalHeight;

        ctx.clearRect(0, 0, frameCanvas.width, frameCanvas.height);
        ctx.drawImage(img, 0, 0);
        scan()
            .then((result) => {
                console.log("Recognition result: " + result);
            })
            .catch((e) => {
                handleError(e);
            });
    };
    img.src = source instanceof File ? URL.createObjectURL(source) : source;
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

if (document.getElementById('export')) {
    document.getElementById('export').addEventListener('click', function (e) {
        let filename = 'symcode-' + new Date().toISOString().slice(0, 19).replace(/:/g, '').replace('T', ' ') + '.png';

        /// create an "off-screen" anchor tag
        let lnk = document.createElement('a');

        /// the key here is to set the download attribute of the a tag
        lnk.download = filename;

        /// convert canvas content to data-uri for link. When download
        /// attribute is set the content pointed to by link will be
        /// pushed as "download" in HTML5 capable browsers
        lnk.href = frameCanvas.toDataURL("image/png;base64");

        /// create a "fake" click-event to trigger the download
        if (document.createEvent) {
            e = document.createEvent("MouseEvents");
            e.initMouseEvent("click", true, true, window,
                0, 0, 0, 0, 0, false, false, false,
                false, 0, null);

            lnk.dispatchEvent(e);
        } else if (lnk.fireEvent) {
            lnk.fireEvent("onclick");
        }
    }, false);
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
    frameCanvas.width = inputFrameSize.width;
    frameCanvas.height = inputFrameSize.height;
    ctx.clearRect(0, 0, frameCanvas.width, frameCanvas.height);
    ctx.drawImage(camera, sx, sy, inputFrameSize.width, inputFrameSize.height,
                                        0, 0, frameCanvas.width, frameCanvas.height);
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
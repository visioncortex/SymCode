import { Acute32SymcodeMain } from "symcode";
import { loadAlphabet } from "./load";

export function loadingCompletes() {
    console.log("Template loading completes.");
}

export function main() {
    const params = new URLSearchParams(document.location.search);
    if (params.get('debug') === "true") {
        document.getElementById('debug').style.display = 'block';
        document.getElementById('errmsg').style.display = 'block';
    } else {
        document.getElementById('debug').style.display = 'none';
        document.getElementById('errmsg').style.display = 'none';
    }

    const wrapper = document.getElementById('coupon_wrapper');

    const frameCanvas = document.getElementById('frame');
    const frameCtx = frameCanvas.getContext('2d');

    let scanner;
    loadAlphabet('loadBuffer').then(() => scanner = getNewScanner());

    function getNewScanner() {
        let sc = Acute32SymcodeMain.new();
        sc.load_alphabet_from_canvas_id('loadBuffer');
        return sc;
    }

    const ERROR_COLOR = "color: #ff5050;";

    function handleError(e) {
        console.log("%c" + e, ERROR_COLOR);
    }

    const SUCCESS_COLOR = "color: #00ff00;";

    function handleSuccess(msg) {
        console.log("%c" + msg, SUCCESS_COLOR);
    }

    function fetchUrlWithCode(code) {
        console.log(`Trying to fetch URL with code: ${code}`);
        try {
            $.ajax({
                url: 'https://symcode.visioncortex.org/api/symcode?code=' + code,
                type: 'get',
                success: function(result) {
                    if (!result.success) {
                        document.getElementById("datainfo-viewspace").classList.add('hidden');
                        console.log("Fetch URL failed!");
                        return;
                    }
                    document.getElementById("datainfo-viewspace").classList.remove('hidden');
                    console.log("Fetched:", result.symcode);
                    console.log("writing payload to 'data-info'...");
                    let linkUrl = result.symcode.payload;
                    if (linkUrl.startsWith("http")) {
                        document.getElementById('data-info').innerHTML = `<a href="${linkUrl}">${result.symcode.title}</a>`;
                    } else {
                        document.getElementById('data-info').innerHTML = result.symcode.title;
                    }
                }
            });
        } catch (e) {
            console.error(e);
        }
    }

    function scan() {
        try {
            let startTime = new Date();
            const code = scanner.scan_from_canvas_id("frame");
            document.getElementById('errmsg').innerHTML = '';
            const time = (new Date() - startTime);
            handleSuccess("Scanning finishes in " + time + " ms.");
            fetchUrlWithCode(code);
            scanner.generate_symcode_to_canvas('frame', parseInt(code, 2));
            frameCanvas.style.display = 'block';
            frameCtx.filter = 'blur(1px)';
            frameCtx.drawImage(frameCanvas, 0, 0);
            return {code, time};
        } catch (e) {
            document.getElementById('errmsg').innerHTML = e;
            throw e;
        }
    }

    //#region Camera Input

    const scanButton = document.getElementById('scan');
    const camera = document.getElementById('camera');
    const showFps = document.getElementById('fps');

    // Flag to control termination of scanning
    let finishScanning = false;
    let lastScanTime = new Date();
    let scanningCount = 0;

    const inputFrameSize = 160;
    const padding = 0;

    const fps = 60;

    const mediaConstraints = {
        video: { width: {ideal: 720}, height: {ideal: 720}, facingMode: "environment" },
    };

    function adjustCameraPosition(vWidth, vHeight) {
        const containerRect = document.getElementsByClassName('coupon_container')[0].getBoundingClientRect();
        const cWidth = containerRect.width;
        const cHeight = containerRect.height;
        const cLeft = (vWidth - cWidth) / 2;
        const cTop = (vHeight - cHeight) / 2;
        camera.style.left = -cLeft + 'px';
        camera.style.top = -cTop + 'px';
        return {cWidth, cHeight, cLeft, cTop};
    }

    scanButton.onclick = () => {
        frameCanvas.style.display = 'none';
        document.getElementById('errmsg').innerHTML = '';
        wrapper.classList.remove("hidden");
        navigator.mediaDevices
            .getUserMedia(mediaConstraints)
            .then((stream) => {
                camera.srcObject = stream;
                getCameraVideoDimensions()
                    .then(({width, height}) => {
                        const containerDimensions = adjustCameraPosition(width, height);
                        startStreaming(containerDimensions);
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
                    width,
                    height,
                });
            }, false);
        });
    }

    function startStreaming({cWidth, cHeight, cLeft, cTop}) {
        finishScanning = false;
        lastScanTime = new Date();
        scanningCount = 0;
        function loop() {
            if ((scanningCount++) % 1000 == 0) {
                console.log("Reallocating scanner...");
                if (scanner) scanner.free();
                scanner = getNewScanner();
            }
            try {
                let result = drawFrame(cLeft, cTop, cWidth, cHeight, padding);
                console.log("Recognition result: " + result.code);
                finishScanning = true;
            } catch (e) {
                const currScanTime = new Date();
                const scanDuration = (currScanTime - lastScanTime) / 1000; // scanning duration in seconds
                showFps.innerHTML = Math.round(1/scanDuration);
                lastScanTime = currScanTime;

                handleError(e);
                if (scanningCount >= 3000) {
                    console.log("Too many scanning ticks! Terminating...");
                    wrapper.classList.add("hidden");
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

    function drawFrame(sx, sy, cw, ch, padding) {
        [frameCanvas.width, frameCanvas.height] = [inputFrameSize, inputFrameSize];
        frameCtx.fillStyle = "#ffffff";
        frameCtx.fillRect(0, 0, frameCanvas.width, frameCanvas.height);
        frameCtx.drawImage(camera, sx, sy, cw, ch,
            padding, padding, frameCanvas.width - 2*padding, frameCanvas.height - 2*padding);
        return scan();
    }

    function stopCamera() {
        const stream = camera.srcObject;
        if (stream) {
            showFps.innerHTML = '';
            stream.getTracks().forEach(function(track) {
                track.stop();
            });
            camera.srcObject = null;
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
        frameCanvas.style.display = 'none';
        document.getElementById('errmsg').innerHTML = '';
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
}
import { Acute32SymcodeMain } from "symcode";
import { loadAlphabet } from "./load";

export function loadingCompletes() {
    console.log("Template loading completes.");
}

export function main() {
    const wrapper = document.getElementById('coupon_wrapper');

    const frameCanvas = document.getElementById('frame');
    const frameCtx = frameCanvas.getContext('2d');
    const originalFrameSize = [frameCanvas.width, frameCanvas.height];

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
                        console.log("Fetch URL failed!");
                        return;
                    }
                    console.log("Fetched:", result.symcode);
                    console.log("writing payload to 'data-info'...");
                    let linkUrl = result.symcode.payload;
                    if (!linkUrl.startsWith("https://")) {
                        linkUrl = "https://" + linkUrl;
                    }
                    document.getElementById('data-info').innerHTML = `<a href="${linkUrl}">${result.symcode.title}</a>`;
                    document.getElementById("datainfo-viewspace").classList.remove('hidden');
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
            const time = (new Date() - startTime);
            handleSuccess("Scanning finishes in " + time + " ms.");
            fetchUrlWithCode(code);
            scanner.generate_symcode_to_canvas('frame', parseInt(code, 2));
            frameCanvas.style.display = 'block';
            frameCtx.filter = 'blur(1px)';
            frameCtx.drawImage(frameCanvas, 0, 0);
            return {code, time};
        } catch (e) {
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

    const fps = 60;

    const mediaConstraints = {
        video: { width: {ideal: 720}, height: {ideal: 720}, facingMode: "environment" },
    };

    function adjustCameraPosition(vWidth, vHeight) {
        let containerRect = document.getElementsByClassName('coupon_container')[0].getBoundingClientRect();
        console.log(containerRect);
        let cWidth = containerRect.width;
        let cHeight = containerRect.height;
        camera.style.left = (-(vWidth - cWidth) / 2) + 'px';
        camera.style.top = (-(vHeight - cHeight) / 2) + 'px';
    }

    scanButton.onclick = () => {
        wrapper.classList.remove("hidden");
        navigator.mediaDevices
            .getUserMedia(mediaConstraints)
            .then((stream) => {
                camera.srcObject = stream;
                getCameraVideoDimensions()
                    .then(({width, height}) => {
                        adjustCameraPosition(width, height);
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
                let result = drawFrame(sx, sy);
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
                    showFps.innerHTML = '';
                    stopCamera();
                }
            }
        }
        sleep(1/fps, loop);
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
}
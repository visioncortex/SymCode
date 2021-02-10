const SeedableRandom = require("./random");
const noiseRng = new SeedableRandom();

export function addNoiseToCanvas(canvasId, maxOpacity, seed) {
    noiseRng.seed(seed);
    const canvas = document.getElementById(canvasId);
    if (!canvas || canvas.tagName.localeCompare("CANVAS") != 0) {
        console.log("addNoise error: no <canvas> element with id", canvasId);
        return;
    }
    if (maxOpacity == 0) {
        return;
    }
    const ctx = canvas.getContext("2d");
    for (let y = 0; y < canvas.height; ++y) {
        for (let x = 0; x < canvas.clientWidth; ++x) {
            const color = noiseRng.next() * 255;
            const opacity = noiseRng.next() * maxOpacity;
            ctx.fillStyle = "rgba(" + color + ", " + color + ", " + color + ", " + opacity + ")";
            ctx.fillRect(x, y, 1, 1); 
        }
    }
}
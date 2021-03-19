import { Acute32SymcodeMain } from "symcode";
import { loadAlphabet } from "./load";
import { main } from './app';

console.log("running bootstrap.js");
if (document.body.id === "scanning") {
    main();
} else {
    console.log("entering bootstrap.js > generation scripts...");
    const loadBufferId = 'loadBuffer';
    loadAlphabet(loadBufferId)
        .then(() => {
            window.symcodeGenerator = Acute32SymcodeMain.new();
            window.symcodeGenerator.load_alphabet_from_canvas_id(loadBufferId);
        });
    
    window.generateCode = (canvasId, payload) => {
        console.log("window.generateCode() is called.");
        if (window.symcodeGenerator) {
            window.symcodeGenerator.generate_symcode_to_canvas(canvasId, payload);
        } else {
            console.error("window.symcodeGenerator is not assigned.");
        }
    };
    
    window.testing = () => console.log("Testing success");
}
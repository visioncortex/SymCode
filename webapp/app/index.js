import { Acute32SymcodeMain } from "symcode";
import { loadAlphabet } from "./load";
import { main } from './app';

if (document.body.id === "scanning") {
    main();
}

window.readyGenerator = async (loadBufferId) => {
    await loadAlphabet(loadBufferId);
    window.symcodeGenerator = Acute32SymcodeMain.new();
    window.symcodeGenerator.load_alphabet_from_canvas_id(loadBufferId);
};

window.generateCode = (canvasId, payload) => {
    window.symcodeGenerator.generate_symcode_to_canvas(canvasId, payload);
};
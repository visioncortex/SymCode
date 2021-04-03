cd webapp
wasm-pack build
rsync -av --delete "./pkg" "../../symcode-private"
rsync -av --delete "./pkg" "../../acute32"
rm ../../acute32/pkg/.gitignore
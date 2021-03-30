## Install
    in webapp/: wasm-pack build // to generate pkg/
    in webapp/dev/: npm install

## Run
    in webapp/dev/: npm run start // and go to http://localhost:8080/
    if files in pkg are not properly linked,
        in webapp/pkg/: npm link
        in webapp/dev/: npm link symcode

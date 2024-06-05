# use PowerShell instead of sh:
set shell := ["powershell.exe", "-c"]

#upload wasm to DEST (user@domain.name:/path/)
wasmup DEST:
    scp ./target/wasm32-unknown-unknown/release/ldj54.wasm {{DEST}}

#upload wasm + html to pew.dk
allup DEST:
    scp ./target/wasm32-unknown-unknown/release/ldj54.wasm {{DEST}}
    scp ./mq_js_bundle.js {{DEST}}
    scp ./index.html {{DEST}}

#TODO upload all res to site
#resup DEST:

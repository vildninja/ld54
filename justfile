# use PowerShell instead of sh:
set shell := ["powershell.exe", "-c"]

#upload wasm to REMOTE <user@domain.dk> PATH </path/on/server/>
# will also gzip wasm (my nginx has gzip_static on)
wasmup REMOTE PATH:
    scp ./target/wasm32-unknown-unknown/release/ldj54.wasm {{REMOTE}}:{{PATH}}
    ssh {{REMOTE}} "cd {{PATH}};gzip -9kf ldj54.wasm"

#upload wasm + js + html to REMOTE <user@domain.dk> PATH </path/on/server/>
# will also gzip wasm + js (my nginx has gzip_static on)
allup REMOTE PATH:
    scp ./target/wasm32-unknown-unknown/release/ldj54.wasm {{REMOTE}}:{{PATH}}
    scp ./mq_js_bundle.js {{REMOTE}}:{{PATH}}
    scp ./index.html {{REMOTE}}:{{PATH}}
    ssh {{REMOTE}} "cd {{PATH}};gzip -9kf ldj54.wasm;gzip -9kf mq_js_bundle.js"

#TODO upload all res to site
#resup DEST:

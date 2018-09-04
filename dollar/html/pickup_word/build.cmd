cargo +nightly web build --target=wasm32-unknown-unknown --release
copy target\wasm32-unknown-unknown\release\pickup_word.js html\pickup_word.js
copy target\wasm32-unknown-unknown\release\pickup_word.wasm html\pickup_word.wasm
rem "请打开html\index.html查看运行结果"
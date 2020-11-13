# Romf
Yet another file upload site
All html and php files are blocked by default.

## Installation
Edit the API_URL and CDN_URL in frontend/src/main.js\\
API_URL is the url for the rust backend
CDN_URL is the url prefix for all uploaded files (protip serve from nginx)

```
cd frontend
yarn build

cd ../
cargo build --release

mkdir dist
mkdir dist/public
mkdir dist/uploads
mv frontend/dist/* dist/public
mv target/release/romf dist/romf
```

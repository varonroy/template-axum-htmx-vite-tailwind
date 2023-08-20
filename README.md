# Axum + htmx + Vite + Tailwind
This is a template project that shows how to setup a web application with Axum as the backend and htmx in frontend.

## Start
To start the backend:
```
$ cargo-watch --watch src -x run
```
To start the frontend:
```
$ cd web
$ pnpm build:watch
```
Then, the app will be available through [127.0.0.1:3000](http://127.0.0.1:3000).

## Live Reloading
This project uses `cargo-watch` to live reload the backend, and `tower-livereload` alongside `vite` to live-reload the frontend.

Note that the actually reloading mechanism sits on the backend. It watches the `web/dist` directory and sends a refresh signal whenever a change is detected. Despite having a debouncer, sometimes (if the build takes too long) the page reloads too soon and then a message appears. If that happens too often you can increase the debouncer time in `src/main.rs`.

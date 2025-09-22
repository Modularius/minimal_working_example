# Router Minimal Repreducible Example

This example creates an nginx proxy which redirects `http://localhost:8089/subfolder` to `http://localhost:3000`.
Change port `8089` as required, by editing:
- `listen 8089;` in `nginx.conf`
- line 39 `<HydrationScripts options=leptos_options.clone() root = "http://localhost:8089/subfolder" />` in `main.rs` .

## Run the server

`podman-compose compose up -d` (or `docker compose` as required).

## Serve the webpage

`cargo leptos watch`.

## Test

Navigate to `http://localhost:8089/subfolder`.
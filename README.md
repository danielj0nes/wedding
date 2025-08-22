# Wedding Website - Rust + Svelte
Website for our wedding, in Rust and Svelte (learning activity :-))

## Docker usage
The easiest way to run the website is with the Docker container, which builds everything then runs it all.

To do so, start at the repository root then:

```powershell
docker build -t wedding -f Dockerfile .
```

Run the container:

```powershell
docker run --rm -p 8080:8080 wedding
```

## Notes
- The server requires the `WEDDING_FRONTEND_DIR` env var to point to the built frontend files inside the container or on the host.
- The env var should point to the production build, which is written to `frontend/wedding-frontend/build`.
- To build the frontend locally:

	```powershell
	npm install
	npx vite build
	```

- To just develop the frontend, run: `npx vite dev`
- The Rust server can be started with `cargo run`
- Or it can be built for production with `cargo build --release` then starting the exe file
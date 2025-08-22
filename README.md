# Wedding Website - Rust + Svelte
Website for our wedding, in Rust and Svelte (learning activity :-))

## Docker usage

Build the Docker image from the repository root:

```powershell
docker build -t wedding -f Dockerfile .
```

Run the container:

```powershell
docker run --rm -p 8080:8080 wedding
```

Notes:
- The server requires the `WEDDING_FRONTEND_DIR` env var to point to the static frontend files inside the container or on the host.
- This project uses SvelteKit with `adapter-static`. The production frontend build is written to `frontend/wedding-frontend/build`.
- To build the frontend locally:

	```powershell
	npm install
	npm vite build
	```

- To build the Docker image, ensure the frontend `build/` folder is present and used by the Dockerfile during image creation.
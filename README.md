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
- The server requires the `WEDDING_FRONTEND_DIR` env var to point to the static frontend files inside the container.
- Ensure frontend build output is present in `frontend/dist` before building the image.
- To set the env var on Windows: `setx WEDDING_FRONTEND_DIR "C:/Full/Path/Goes/Here"`
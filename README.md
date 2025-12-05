# PETALL

## Run the full stack with Docker Compose

1. Generate the RSA key pair (if you haven't already):
   ```
   cd keys
   ./setup.sh
   ```
   This produces `validation.key` (private, keep it safe) and `validation.key.pub`.
2. (Optional) Create a `.env` file in the repo root to override any of the variables consumed by `docker-compose.yml` (for example `GOOGLE_CLIENT_ID`, `GOOGLE_CLIENT_SECRET`, `GOOGLE_REDIRECT_URL`, `PUBLIC_TRUSTED_ENTITY_APP_URL`, etc.).
3. Build and launch every service:
   ```
   docker compose up --build
   ```

The stack exposes:

- `http://localhost:4173` → Community Frontend
- `http://localhost:4174` → Trusted Entity App

Both the backend and the trusted-entity app mount the `./keys` directory, so make sure the generated files stay there (or update the compose file to point somewhere else). Set real Google OAuth credentials before attempting to log in through oauth.

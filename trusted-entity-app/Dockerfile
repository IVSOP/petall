# syntax=docker/dockerfile:1.7

FROM oven/bun:1.1 AS base
ENV SKIP_NODE_VERSION_CHECK=1
WORKDIR /app

FROM base AS deps
COPY trusted-entity-app/package.json ./package.json
COPY trusted-entity-app/bun.lock ./bun.lock
RUN bun install

FROM deps AS builder
COPY trusted-entity-app/ ./
COPY keys/validation.key.pub /keys/validation.key.pub
RUN bun run build

FROM base AS runtime
WORKDIR /app
ENV NODE_ENV=production
COPY --from=builder /app /app
COPY keys/validation.key.pub /keys/validation.key.pub
EXPOSE 4174
ENV HOST=0.0.0.0
ENV PORT=4174
CMD ["sh","-c","bun run preview -- --host ${HOST:-0.0.0.0} --port ${PORT:-4174}"]


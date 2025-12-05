# syntax=docker/dockerfile:1.7

FROM oven/bun:1.1 AS base
ENV SKIP_NODE_VERSION_CHECK=1
WORKDIR /app

FROM base AS deps
COPY community-frontend/package.json ./package.json
COPY community-frontend/bun.lock ./bun.lock
RUN bun install

FROM deps AS builder
COPY community-frontend/ ./
RUN bun run build

FROM base AS runtime
WORKDIR /app
ENV NODE_ENV=production
COPY --from=builder /app /app
EXPOSE 4173
ENV HOST=0.0.0.0
ENV PORT=4173
CMD ["sh","-c","bun run preview -- --host ${HOST:-0.0.0.0} --port ${PORT:-4173}"]


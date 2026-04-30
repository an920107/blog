FROM oven/bun:1-alpine AS base

FROM base AS deps
RUN apk add --no-cache libc6-compat
RUN mkdir -p /temp/dev
RUN mkdir -p /temp/prod
COPY package.json bun.lock /temp/dev/
COPY package.json bun.lock /temp/prod/
RUN cd /temp/dev && bun install --frozen-lockfile
RUN cd /temp/prod && bun install --frozen-lockfile --production

FROM base AS builder
WORKDIR /app
COPY --from=deps /temp/dev/node_modules ./node_modules
COPY . .
RUN --mount=type=secret,id=SENTRY_AUTH_TOKEN \
    SENTRY_AUTH_TOKEN="$(cat /run/secrets/SENTRY_AUTH_TOKEN 2>/dev/null || true)" \
    bun run build

FROM base AS runner
WORKDIR /app
COPY --from=deps /temp/prod/node_modules ./node_modules
COPY --from=builder /app/build ./build
EXPOSE 3000
ENV NODE_ENV=production
ENV HOSTNAME=0.0.0.0
ENV PORT=3000
CMD ["bun", "run", "build/index.js"]

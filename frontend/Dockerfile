FROM node:22-alpine AS base
ENV COREPACK_ENABLE_DOWNLOAD_PROMPT=0
RUN corepack enable pnpm

FROM base AS deps
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
RUN apk add --no-cache libc6-compat && \
    pnpm install --frozen-lockfile

FROM base AS builder
WORKDIR /app
COPY --from=deps /app/node_modules ./node_modules
COPY . .
RUN pnpm run build

FROM base AS runner
WORKDIR /app
COPY package.json pnpm-lock.yaml ./
COPY --from=builder /app/build ./build
RUN pnpm install --prod --frozen-lockfile
EXPOSE 3000
ENV NODE_ENV=production
ENV HOSTNAME=0.0.0.0
ENV PORT=3000
ENV PUBLIC_API_BASE_URL=http://127.0.0.1:8080/
CMD ["node", "build"]

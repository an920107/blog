FROM python:3.14-slim AS base
RUN apt-get update && apt-get install -y build-essential && \
    apt-get clean && rm -rf /var/lib/apt/lists/*

FROM base AS dependencies
WORKDIR /app
COPY requirements.txt .
RUN python -m venv .venv && \
    . .venv/bin/activate && \
    pip install --no-cache-dir -r requirements.txt

FROM base
WORKDIR /app
COPY --from=dependencies /app/.venv .venv
COPY . .
ENV PATH="/app/.venv/bin:$PATH"

# Set default environment variables
ENV DATABASE_HOST=127.0.0.1
ENV DATABASE_PORT=5432
ENV DATABASE_USER=postgres
ENV DATABASE_PASSWORD=""
ENV DATABASE_NAME=postgres
ENV RSS_LANGUAGE="zh-TW"
ENV RSS_OUTPUT_FILE="feed.xml"
ENV SERVER_BASE_URL="https://example.com/"

CMD ["sleep", "infinity"]

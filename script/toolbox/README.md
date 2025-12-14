# Toolbox Scripts

This directory contains utility scripts for the blog application.

## RSS Generator (`generate_rss.py`)

This script connects to the PostgreSQL database, fetches published posts, and generates an RSS 2.0 feed XML file.

### Usage

The toolbox container is designed to run in the background (sleeping). You can execute the generation script inside the running container.

#### 1. Volume Configuration

To make the generated `feed.xml` accessible to your web server or host machine, you need to mount a volume when starting the container.

For example, if you want the feed to be generated at `/var/www/blog/feed.xml` on the host machine:

1. Ensure the file exists on the host (e.g., `touch /var/www/blog/feed.xml`), otherwise it might be created as a directory.
2. Mount the host file directly to the path inside the container.
3. Set the `RSS_OUTPUT_FILE` environment variable to point to that internal path.

```bash
podman run -d \
  --name blog-toolbox \
  --env-file .env \
  -v /var/www/blog/feed.xml:/app/feed.xml \
  -e RSS_OUTPUT_FILE=/app/feed.xml \
  blog-toolbox-image
```

#### 2. Generate RSS Feed

Run the following command to generate the RSS feed:

```bash
podman exec -it blog-toolbox python generate_rss.py
```

### Environment Variables

The script relies on the following environment variables:

| Variable | Description | Default |
|----------|-------------|---------|
| `DATABASE_HOST` | PostgreSQL database host | `127.0.0.1` |
| `DATABASE_PORT` | PostgreSQL database port | `5432` |
| `DATABASE_NAME` | Database name | `postgres` |
| `DATABASE_USER` | Database user | `postgres` |
| `DATABASE_PASSWORD` | Database password | (empty) |
| `SERVER_BASE_URL` | Base URL of the blog. Used for generating links and fetching site title/description. | `http://localhost/` |
| `RSS_LANGUAGE` | Language code for the RSS feed | `zh-TW` |
| `RSS_OUTPUT_FILE` | Absolute path where the XML file will be written inside the container | `feed.xml` |

### Features

- **Dynamic Metadata**: Fetches the site title and description from the `SERVER_BASE_URL` HTML head.
- **Preview Images**: Includes preview images in the feed as `<enclosure>` elements, automatically detecting the correct MIME type and content length from HTTP response headers.
- **Semantic IDs**: Uses semantic IDs for permanent links and GUIDs.
- **UTF-8 Support**: Properly handles Chinese and other Unicode characters in titles and descriptions.

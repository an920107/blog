import os
from datetime import datetime, timezone
from typing import Any
from xml.dom import minidom
from xml.etree.ElementTree import Element, SubElement, tostring

import psycopg2
import requests
from dotenv import load_dotenv
from psycopg2.extras import RealDictCursor

load_dotenv()

# Database configuration
DB_CONFIG = {
    "host": os.getenv("DATABASE_HOST", "127.0.0.1"),
    "port": os.getenv("DATABASE_PORT", "5432"),
    "database": os.getenv("DATABASE_NAME", "postgres"),
    "user": os.getenv("DATABASE_USER", "postgres"),
    "password": os.getenv("DATABASE_PASSWORD", ""),
}

# RSS feed configuration
RSS_CONFIG = {
    "language": os.getenv("RSS_LANGUAGE", "zh-TW"),
    "output_file": os.getenv("RSS_OUTPUT_FILE", "feed.xml"),
}

SERVER_BASE_URL = os.getenv("SERVER_BASE_URL", "http://localhost/")


def get_site_metadata() -> dict[str, str]:
    """
    Get site title and description from SERVER_BASE_URL HTML head
    """
    try:
        response = requests.get(SERVER_BASE_URL, timeout=10)
        response.raise_for_status()

        # Ensure proper encoding for Chinese characters
        response.encoding = response.apparent_encoding or "utf-8"
        html_content = response.text

        # Extract title
        title = "My Blog"
        title_start = html_content.find("<title>")
        if title_start != -1:
            title_start += len("<title>")
            title_end = html_content.find("</title>", title_start)
            if title_end != -1:
                title = html_content[title_start:title_end].strip()

        # Extract description from meta tag
        description = "Blog RSS Feed"
        desc_start = html_content.find('<meta name="description"')
        if desc_start != -1:
            content_start = html_content.find('content="', desc_start)
            if content_start != -1:
                content_start += len('content="')
                content_end = html_content.find('"', content_start)
                if content_end != -1:
                    description = html_content[content_start:content_end].strip()

        return {"title": title, "description": description}

    except Exception as e:
        print(f"Warning: Failed to fetch site metadata: {e}")
        return {"title": "My Blog", "description": "Blog RSS Feed"}


def get_content_type_and_length(url: str) -> tuple[str, int]:
    """
    Get content type and content length from URL response headers
    Returns: (content_type, content_length)
    """
    try:
        # Try HEAD request first
        response = requests.head(url, timeout=5, allow_redirects=True)
        content_type = response.headers.get("Content-Type", "")
        content_length = int(response.headers.get("Content-Length", 0))

        # If we got a valid image content type, return it
        if content_type and content_type.startswith("image/"):
            return content_type.split(";")[0].strip(), content_length

        # If HEAD request didn't work, try GET with stream
        response = requests.get(url, timeout=5, allow_redirects=True, stream=True)
        content_type = response.headers.get("Content-Type", "")
        content_length = int(response.headers.get("Content-Length", 0))
        response.close()  # Close the connection immediately

        if content_type:
            return content_type.split(";")[0].strip(), content_length

        return "image/jpeg", content_length  # Default fallback

    except Exception:
        # Default to image/jpeg if request fails
        return "image/jpeg", 0


def get_posts_from_db() -> list[dict[str, Any]]:
    conn = None
    try:
        conn = psycopg2.connect(
            dsn=f"host={DB_CONFIG['host']} port={DB_CONFIG['port']} dbname={DB_CONFIG['database']} user={DB_CONFIG['user']} password={DB_CONFIG['password']}"
        )
        cursor = conn.cursor(cursor_factory=RealDictCursor)

        # Fetch published posts with labels
        query = """
            SELECT
                p.id,
                p.semantic_id,
                p.preview_image_url,
                p.title,
                p.description,
                p.published_time,
                p.updated_time,
                ARRAY_AGG(l.name ORDER BY pl."order") FILTER (WHERE l.id IS NOT NULL) AS labels
            FROM
                post p
            LEFT JOIN
                post_label pl ON p.id = pl.post_id
            LEFT JOIN
                label l ON pl.label_id = l.id AND l.deleted_time IS NULL
            WHERE
                p.deleted_time IS NULL
                AND p.published_time IS NOT NULL
            GROUP BY
                p.id, p.semantic_id, p.title, p.description, p.content, p.published_time, p.updated_time
            ORDER BY
                p.published_time DESC
            LIMIT 20
        """

        cursor.execute(query)
        posts = cursor.fetchall()
        cursor.close()

        return [dict(post) for post in posts]

    except Exception as e:
        print(f"Database error: {e}")
        return []

    finally:
        if conn:
            conn.close()


def generate_rss_feed(posts: list[dict[str, Any]]) -> str:
    """
    Generate RSS 2.0 format XML
    """
    # Get site metadata from homepage
    site_metadata = get_site_metadata()

    # Create root element
    rss = Element(
        "rss",
        version="2.0",
        attrib={
            "xmlns:atom": "http://www.w3.org/2005/Atom",
            "xmlns:content": "http://purl.org/rss/1.0/modules/content/",
        },
    )

    channel = SubElement(rss, "channel")

    # Channel basic information
    SubElement(channel, "title").text = site_metadata["title"]
    SubElement(channel, "link").text = SERVER_BASE_URL
    SubElement(channel, "description").text = site_metadata["description"]
    SubElement(channel, "language").text = RSS_CONFIG["language"]
    SubElement(channel, "lastBuildDate").text = datetime.now(tz=timezone.utc).strftime(
        "%a, %d %b %Y %H:%M:%S +0000"
    )

    # Atom self link
    SubElement(
        channel,
        "atom:link",
        attrib={
            "href": f"{SERVER_BASE_URL}feed.xml",
            "rel": "self",
            "type": "application/rss+xml",
        },
    )

    # Create item for each post
    for post in posts:
        item = SubElement(channel, "item")

        # Title
        SubElement(item, "title").text = post.get("title", "Untitled")

        # Post link (using semantic_id)
        post_link = f"{SERVER_BASE_URL}post/{post.get('semantic_id', '')}"
        SubElement(item, "link").text = post_link

        # GUID uses semantic_id as unique identifier
        sementic_id = post.get("semantic_id", "")
        SubElement(item, "guid", attrib={"isPermaLink": "false"}).text = sementic_id

        # Description or content
        description = post.get("description", "")
        SubElement(item, "description").text = description

        # Preview image (enclosure)
        if "preview_image_url" in post and post["preview_image_url"]:
            preview_url = post["preview_image_url"]
            # If it's a relative URL, make it absolute
            if not preview_url.startswith(("http://", "https://")):
                preview_url = f"{SERVER_BASE_URL.rstrip('/')}/{preview_url.lstrip('/')}"

            # Get content type and length from response headers
            content_type, content_length = get_content_type_and_length(preview_url)

            enclosure_attrs = {
                "url": preview_url,
                "type": content_type,
            }

            # Add length attribute if available
            if content_length > 0:
                enclosure_attrs["length"] = str(content_length)

            SubElement(item, "enclosure", attrib=enclosure_attrs)

        # Publish date
        if "published_time" in post and post["published_time"]:
            pub_date = post["published_time"]
            if isinstance(pub_date, datetime):
                SubElement(item, "pubDate").text = pub_date.strftime(
                    "%a, %d %b %Y %H:%M:%S +0000"
                )

        # Labels/categories
        if "labels" in post and post["labels"]:
            for label in post["labels"]:
                if label:  # Filter empty values
                    SubElement(item, "category").text = label

    # Format XML
    xml_string = tostring(rss, encoding="unicode")
    dom = minidom.parseString(xml_string)
    pretty_xml = dom.toprettyxml(indent="  ", encoding="UTF-8").decode("utf-8")

    # Remove extra blank lines
    lines = [line for line in pretty_xml.split("\n") if line.strip()]
    return "\n".join(lines)


def save_rss_feed(xml_content: str, output_file: str):
    """
    Save RSS feed to file
    """
    try:
        with open(output_file, "w", encoding="utf-8") as f:
            f.write(xml_content)
        print(f"RSS feed successfully generated: {output_file}")
    except Exception as e:
        print(f"Error saving RSS feed: {e}")


def main():
    print("Fetching posts from database...")
    posts = get_posts_from_db()

    if not posts:
        print("No posts found or database error occurred.")
        return

    print(f"Found {len(posts)} posts. Generating RSS feed...")
    xml_content = generate_rss_feed(posts)

    output_file = RSS_CONFIG["output_file"]
    save_rss_feed(xml_content, output_file)


if __name__ == "__main__":
    main()

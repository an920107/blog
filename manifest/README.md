# Deployment Manifests

Kubernetes manifests for self-hosting the blog. Everything runs in a single
`Pod` named `blog`, which bundles Postgres, Qdrant, Redis, the backend and the
frontend containers together, backed by persistent volumes.

## Contents

| File | In Git | Description |
| --- | --- | --- |
| `blog.pod.yaml` | Yes | The `Pod`: `postgres`, `qdrant`, `redis`, `backend` and `frontend` containers, their volumes and host ports. |
| `blog.secret.yaml` | No | `Secret` `blog-secret` holding sensitive values. |
| `blog.secret.yaml.example` | Yes | Template for `blog.secret.yaml`. |
| `blog.config.yaml` | No | `Secret` `blog-config` holding non-sensitive settings. |
| `blog.config.yaml.example` | Yes | Template for `blog.config.yaml`. |
| `Makefile` | Yes | Concatenates the sources into `blog.bundle.yaml`. |
| `blog.bundle.yaml` | No | Generated, apply-ready single-file manifest. Do not edit by hand. |

`.gitignore` keeps the real `blog.secret.yaml`, `blog.config.yaml` and the
generated `blog.bundle.yaml` out of Git; only the `.example` templates are
committed.

## Configuration

Create the local files from the templates:

```sh
cp blog.secret.yaml.example blog.secret.yaml
cp blog.config.yaml.example blog.config.yaml
```

`blog-secret` (sensitive) needs:

- `DATABASE_PASSWORD`
- `QDRANT_API_KEY`
- `SESSION_KEY`
- `OIDC_CLIENT_SECRET`

`blog-config` (non-sensitive) needs:

- `OIDC_ISSUER_URL`, `OIDC_REDIRECT_URL`, `OIDC_CLIENT_ID`
- `QDRANT_SEARCH_THRESHOLD`
- `API_BASE_URL`, `APP_BASE_URL`
- `GA_MEASUREMENT_ID`, `ADSENSE_CLIENT_ID`
- `FRONTEND_SENTRY_DSN`, `BACKEND_SENTRY_DSN`

`blog-secret` uses the `data` field, so its values must be base64-encoded:

```sh
printf '%s' 'your-password' | base64
```

## Generating the bundle

The Makefile concatenates `blog.secret.yaml`, `blog.config.yaml` and
`blog.pod.yaml` (in that order) into `blog.bundle.yaml`, separating the
documents with `---`, and marks the result read-only:

```sh
make        # writes blog.bundle.yaml
make clean  # removes it
```

`blog.bundle.yaml` is regenerated from scratch on every `make`, so edit the
sources, not the bundle.

> The bundle embeds whatever is in `blog.secret.yaml`, but it is git-ignored and
> never committed, so it is safe for it to contain real values.

## Deploying

```sh
kubectl apply -f blog.bundle.yaml
```

Prerequisites:

- A Kubernetes cluster (a single node is enough).
- The PersistentVolumeClaims referenced by the pod (`blog-postgres`,
  `blog-qdrant`, `blog-storage`, `blog-embedding-cache`) must already exist;
  they are not defined in these manifests.
- The images `registry.squidspirit.com/squid/blog-backend` and
  `registry.squidspirit.com/squid/blog-frontend` must be reachable.

The backend and frontend publish `hostPort` `10012` and `10011` respectively,
both bound to `127.0.0.1`, so they are expected to sit behind a reverse proxy.

## Release flow

On a published release, the Deployment workflow
(`.gitea/workflows/deployment.yaml`) builds and pushes both images (tagged
`latest` and the release tag) and attaches a zip of this directory to the
release. `blog.bundle.yaml` is git-ignored, so the zip carries the manifest
sources and templates rather than the generated bundle; fill in the
configuration and run `make` to produce it.

## Notes

- `blog-config` is a `Secret` (not a `ConfigMap`) even though it only holds
  non-sensitive values; it is consumed the same way as `blog-secret`.
- The pod mounts the host's `/etc/localtime` so container logging follows the
  host timezone.

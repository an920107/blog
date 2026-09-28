# Deployment Manifests

Kubernetes manifests for self-hosting the blog. Everything runs in a single
`Pod` (named `blog` for the primary instance, `blog-beta` for the beta one),
which bundles Postgres, Qdrant, Redis, the backend and the frontend containers
together, backed by persistent volumes.

The instance name is not hardcoded: the sources use a `__NAME__` placeholder
and the `Makefile` substitutes it, so a second instance can be generated with a
single variable instead of hand-editing every resource.

## Contents

| File | In Git | Description |
| --- | --- | --- |
| `blog.pod.yaml` | Yes | The `Pod`: `postgres`, `qdrant`, `redis`, `backend` and `frontend` containers, their volumes, host ports and images. Instance-scoped names, host ports and image tags use placeholders that the Makefile fills in. |
| `blog.secret.yaml` | No | `Secret` `__NAME__-secret` holding sensitive values. |
| `blog.secret.yaml.example` | Yes | Template for `blog.secret.yaml`. |
| `blog.config.yaml` | No | `Secret` `__NAME__-config` holding non-sensitive settings. |
| `blog.config.yaml.example` | Yes | Template for `blog.config.yaml`. |
| `Makefile` | Yes | Picks the sources for `NAME`, substitutes the name, port, image and tag placeholders and concatenates them into `<NAME>.bundle.yaml`. |
| `<NAME>.bundle.yaml` | No | Generated, apply-ready single-file manifest. Do not edit by hand. |

`.gitignore` keeps the real `blog.secret.yaml`, `blog.config.yaml` and the
generated `<NAME>.bundle.yaml` out of Git; only the `.example` templates are
committed. Extra per-instance files are covered by the same patterns
(`*.secret.yaml`, `*.config.yaml`, `*.bundle.yaml`).

## Configuration

Create the local files from the templates:

```sh
cp blog.secret.yaml.example blog.secret.yaml
cp blog.config.yaml.example blog.config.yaml
```

`__NAME__-secret` (sensitive) needs:

- `DATABASE_PASSWORD`
- `QDRANT_API_KEY`
- `SESSION_KEY`
- `OIDC_CLIENT_SECRET`

`__NAME__-config` (non-sensitive) needs:

- `OIDC_ISSUER_URL`, `OIDC_REDIRECT_URL`, `OIDC_CLIENT_ID`
- `QDRANT_SEARCH_THRESHOLD`
- `API_BASE_URL`, `APP_BASE_URL`
- `GA_MEASUREMENT_ID`, `ADSENSE_CLIENT_ID`
- `FRONTEND_SENTRY_DSN`, `BACKEND_SENTRY_DSN`

`__NAME__-secret` uses the `data` field, so its values must be base64-encoded:

```sh
printf '%s' 'your-password' | base64
```

## Generating the bundle

The Makefile picks the sources for `NAME` (default `blog`), reads
`FRONTEND_PORT` (default `10011`) and `BACKEND_PORT` (default `10012`), replaces
the `__NAME__`, `__FRONTEND_PORT__` and `__BACKEND_PORT__` placeholders, then
concatenates the secret, config and pod documents (in that order) into
`<NAME>.bundle.yaml`, separating them with `---`, and marks the result
read-only:

```sh
make                    # writes blog.bundle.yaml
make NAME=blog-beta     # writes blog-beta.bundle.yaml
make clean              # removes blog.bundle.yaml
make clean NAME=blog-beta
```

The generated bundle is regenerated from scratch on every `make`, so edit the
sources, not the bundle.

> The bundle embeds whatever is in `blog.secret.yaml`, but it is git-ignored and
> never committed, so it is safe for it to contain real values.

### Deploying a second instance

Pass a different `NAME` to generate an independent instance. Substitution
touches the Pod name, the `__NAME__-secret` and `__NAME__-config` Secret names,
all volume names and all `persistentVolumeClaim.claimName` references, so the
beta manifests reference the beta secrets instead of the production ones.
Container image repositories (`.../blog-backend`, `.../blog-frontend`) are
intentionally left untouched.

Sources are resolved per instance: `<NAME>.secret.yaml`, `<NAME>.config.yaml`
and `<NAME>.pod.yaml` are used when they exist, otherwise the shared `blog.*`
file is used (its `__NAME__` placeholder still resolves to `<NAME>`). A new
instance that only differs by name therefore needs no extra files:

```sh
make NAME=blog-staging
```

If the instance shares a node with the primary one, also give it its own
`hostPort`s via `FRONTEND_PORT` and `BACKEND_PORT`, otherwise both Pods would
try to bind the same ports:

```sh
make NAME=blog-beta FRONTEND_PORT=10021 BACKEND_PORT=10022
kubectl apply -f blog-beta.bundle.yaml
```

`BACKEND_TAG` (default `v0.7.0`) and `FRONTEND_TAG` (default `v0.7.3`) pin the
image tags, so an instance can run a different build without editing the pod:

```sh
make NAME=blog-beta BACKEND_TAG=latest FRONTEND_TAG=latest
```

The registry and repository rarely change, but they are configurable too via
`BACKEND_IMAGE` and `FRONTEND_IMAGE` (default
`registry.squidspirit.com/squid/blog-backend` and
`registry.squidspirit.com/squid/blog-frontend`).

To give an instance its own config values (public URLs, OAuth redirect, Sentry
DSNs, `OIDC_CLIENT_ID`, ...), copy the template to `<NAME>.config.yaml` and edit
the values. Keep the `__NAME__` in `metadata.name` so the Secret name stays in
sync with the Pod, or write the literal name yourself:

```sh
cp blog.config.yaml.example blog-beta.config.yaml
# edit values
make NAME=blog-beta
```

## Deploying

```sh
kubectl apply -f blog.bundle.yaml   # or blog-beta.bundle.yaml
```

Prerequisites:

- A Kubernetes cluster (a single node is enough).
- The PersistentVolumeClaims referenced by the pod (`<NAME>-postgres`,
  `<NAME>-qdrant`, `<NAME>-storage`, `<NAME>-embedding-cache`) must already
  exist; they are not defined in these manifests.
- The images `registry.squidspirit.com/squid/blog-backend` and
  `registry.squidspirit.com/squid/blog-frontend` (overridable via
  `BACKEND_IMAGE` and `FRONTEND_IMAGE`) must be reachable.

The backend and frontend publish `hostPort` `10012` and `10011` by default
(`BACKEND_PORT` and `FRONTEND_PORT`), both bound to `127.0.0.1`, so they are
expected to sit behind a reverse proxy.

## Release flow

On a published release, the Deployment workflow
(`.gitea/workflows/deployment.yaml`) builds and pushes both images (tagged
`latest` and the release tag) and attaches a zip of this directory to the
release. `blog.bundle.yaml` is git-ignored, so the zip carries the manifest
sources and templates rather than the generated bundle; fill in the
configuration and run `make` to produce it.

## Notes

- `__NAME__-config` is a `Secret` (not a `ConfigMap`) even though it only
  holds non-sensitive values; it is consumed the same way as `__NAME__-secret`.
- The pod mounts the host's `/etc/localtime` so container logging follows the
  host timezone.

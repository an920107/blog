import { Environment } from '$lib/environment';
import {
	mockLabels,
	type MockPostInfo,
	mockPosts,
	toPostInfo,
	toPostResponse,
} from '$lib/mock/mockPostFixtures';

const jsonResponse = (body: unknown, status = 200): Response =>
	new Response(JSON.stringify(body), {
		status,
		headers: { 'Content-Type': 'application/json' },
	});

/**
 * Wraps a `fetch` implementation so every call that targets the backend API is answered with
 * fixture data instead. The wrapper is only wired up by `Container` when
 * `PUBLIC_USE_MOCK_API=true` and the app runs in dev mode.
 */
export function createMockFetch(realFetch: typeof fetch): typeof fetch {
	const apiBaseUrl = new URL(Environment.API_BASE_URL);
	const apiBasePath = apiBaseUrl.pathname.replace(/\/+$/, '');

	return async (input, init) => {
		const rawUrl = input instanceof Request ? input.url : input.toString();
		const url = new URL(rawUrl, Environment.APP_BASE_URL);

		const isApiRequest =
			url.origin === apiBaseUrl.origin && url.pathname.startsWith(`${apiBasePath}/`);

		if (!isApiRequest) {
			return realFetch(input, init);
		}

		const method = (
			input instanceof Request ? input.method : (init?.method ?? 'GET')
		).toUpperCase();
		const path = url.pathname.slice(apiBasePath.length + 1);

		if (method !== 'GET') {
			return jsonResponse({ message: `The mock API does not handle ${method} /${path}` }, 501);
		}

		if (path === 'post') {
			return jsonResponse(listPosts(url.searchParams));
		}

		if (path.startsWith('post/')) {
			const id = decodeURIComponent(path.slice('post/'.length));
			const post = mockPosts.find(
				(candidate) => candidate.semantic_id === id || String(candidate.id) === id
			);

			return post
				? jsonResponse(toPostResponse(post))
				: jsonResponse({ message: `Post ${id} not found` }, 404);
		}

		if (path === 'label') {
			return jsonResponse(mockLabels);
		}

		if (path === 'me') {
			// Same response a logged-out visitor gets, so the auth store resolves to null.
			return new Response(null, { status: 401 });
		}

		return jsonResponse({ message: `The mock API does not handle /${path}` }, 501);
	};
}

function listPosts(searchParams: URLSearchParams): MockPostInfo[] {
	const publishedOnly = searchParams.get('is_published_only') === 'true';
	const labelIdParam = searchParams.get('label_id');
	const labelId = labelIdParam === null ? null : Number(labelIdParam);
	const keyword = searchParams.get('keyword')?.trim().toLowerCase();

	return mockPosts
		.filter((post) => (publishedOnly ? post.published_time !== null : true))
		.filter((post) => (labelId === null ? true : post.labels.some((label) => label.id === labelId)))
		.filter((post) => {
			if (!keyword) {
				return true;
			}

			return `${post.title} ${post.description}`.toLowerCase().includes(keyword);
		})
		.map((post) => toPostInfo(post));
}

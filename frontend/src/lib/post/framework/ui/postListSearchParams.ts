export interface PostListSearchParams {
	keyword?: string;
	labelId?: number;
}

/**
 * Reads the `/post` listing search params (`keyword` / `label_id`) from a URL.
 *
 * Shared by the route load and the search launcher so both decode the query the
 * same way.
 */
export function parsePostListSearchParams(searchParams: URLSearchParams): PostListSearchParams {
	const keyword = searchParams.get('keyword') ?? undefined;
	const labelIdParam = searchParams.get('label_id');
	return { keyword, labelId: labelIdParam ? Number(labelIdParam) : undefined };
}

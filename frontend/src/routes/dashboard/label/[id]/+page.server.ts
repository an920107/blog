import { error } from '@sveltejs/kit';

import type { Container } from '$lib/container';
import type { LabelViewModel } from '$lib/label/adapter/presenter/viewModel/labelViewModel';
import type { PostInfoViewModel } from '$lib/post/adapter/presenter/viewModel/postInfoViewModel';
import { Strings } from '$lib/strings';

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals, params }) => {
	const { container } = locals;

	const numericId = Number(params.id);
	if (isNaN(numericId)) {
		error(400, { message: Strings.INVALID_LABEL_ID });
	}
	const label = await loadLabel(container, numericId);
	const posts = await loadPosts(container, label.id);

	return {
		id: numericId,
		dehydratedLabel: label.dehydrate(),
		dehydratedPosts: posts.map((post) => post.dehydrate()),
	};
};

async function loadLabel(container: Container, id: number): Promise<LabelViewModel> {
	const store = container.createLabelLoadedStore();
	const { trigger } = store;

	const state = await trigger(id);
	if (!state.data) {
		error(404, { message: Strings.LABEL_NOT_FOUND });
	}

	return state.data;
}

async function loadPosts(
	container: Container,
	labelId: number
): Promise<readonly PostInfoViewModel[]> {
	const store = container.createPostsListedStore();
	const { trigger } = store;

	const state = await trigger({ labelId, showUnpublished: true });

	return state.data ?? [];
}

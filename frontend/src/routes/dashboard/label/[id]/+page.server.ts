import { error } from '@sveltejs/kit';
import type { PageServerLoad } from './$types';
import type { Container } from '$lib/container';
import type { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
import type { PostInfoViewModel } from '$lib/post/adapter/presenter/postInfoViewModel';

export const load: PageServerLoad = async ({ locals, params }) => {
	const { container } = locals;

	const numericId = Number(params.id);
	if (isNaN(numericId)) {
		error(400, { message: 'Invalid label ID' });
	}
	const label = await loadLabel(container, numericId);
	const posts = await loadPosts(container, label.id);

	return {
		dehydratedLabel: label.dehydrate(),
		dehydratedPosts: posts.map((post) => post.dehydrate()),
	};
};

async function loadLabel(container: Container, id: number): Promise<LabelViewModel> {
	const store = container.createLabelLoadedStore();
	const { trigger } = store;

	const state = await trigger(id);
	if (!state.data) {
		error(404, { message: 'Label not found' });
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

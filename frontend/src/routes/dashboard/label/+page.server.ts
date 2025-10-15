import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals }) => {
	const { container } = locals;
	const store = container.createLabelsListedStore();
	const { trigger: loadLabels } = store;

	const state = await loadLabels();

	return {
		dehydratedData: state.data?.map((label) => label.dehydrate()),
	};
};

import type { PageServerLoad } from './$types';

export const load: PageServerLoad = async ({ locals }) => {
	const { container } = locals;
	const store = container.createImagesListedStore();
	const { trigger: loadImages } = store;

	const state = await loadImages();

	return {
		dehydratedData: state.data?.map((image) => image.dehydrate()),
	};
};

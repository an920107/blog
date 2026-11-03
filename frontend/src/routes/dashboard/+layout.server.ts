import { error } from '@sveltejs/kit';

import { Strings } from '$lib/strings';

import type { LayoutServerLoad } from './$types';

export const load: LayoutServerLoad = async ({ locals }) => {
	const store = locals.container.createAuthLoadedStore();
	const state = await store.trigger();

	if (!state.isSuccess()) {
		error(500, { message: Strings.AUTHENTICATION_CHECK_FAILED });
	}

	if (!state.data.isAuthenticated) {
		error(404, { message: Strings.NOT_FOUND });
	}
};

import { writable } from 'svelte/store';

import type { ActionStore } from '$lib/shared/adapter/presenter/actionStore';
import { AsyncState } from '$lib/shared/adapter/presenter/asyncState';
import { DrawerViewModel } from '$lib/shared/adapter/presenter/viewModel/drawerViewModel';

type DrawerState = AsyncState<DrawerViewModel>;

export class DrawerConfiguredStore implements ActionStore<DrawerViewModel, DrawerViewModel> {
	private readonly state = writable<DrawerState>(
		AsyncState.success<DrawerViewModel>(DrawerViewModel.empty())
	);

	constructor() {}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return (params: DrawerViewModel) => this.configureDrawer(params);
	}

	private async configureDrawer(params: DrawerViewModel): Promise<DrawerState> {
		const result = AsyncState.success(params);
		this.state.set(result);
		return result;
	}
}

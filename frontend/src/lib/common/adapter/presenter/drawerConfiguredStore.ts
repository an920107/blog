import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import { writable } from 'svelte/store';
import { DrawerViewModel } from './drawerViewModel';

type DrawerState = AsyncState<DrawerViewModel>;

export class DrawerConfiguredStore implements BaseStore<DrawerState, DrawerViewModel> {
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

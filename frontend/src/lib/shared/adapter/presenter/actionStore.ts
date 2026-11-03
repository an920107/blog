import type { Readable } from 'svelte/store';

import type { AsyncState } from '$lib/shared/adapter/presenter/asyncState';

export interface ActionStore<T, U = void> {
	get subscribe(): Readable<AsyncState<T>>['subscribe'];
	get trigger(): (arg: U) => Promise<AsyncState<T>>;
}

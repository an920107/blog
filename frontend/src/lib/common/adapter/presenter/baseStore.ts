import type { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { Readable } from 'svelte/store';

export interface BaseStore<T extends AsyncState<unknown>, U = void> {
	get subscribe(): Readable<T>['subscribe'];
	get trigger(): (arg: U) => Promise<T>;
}

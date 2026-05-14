import type { Readable } from 'svelte/store';

import type { AsyncState } from '$lib/common/adapter/presenter/asyncState';

export interface BaseStore<T extends AsyncState<unknown>, U = void> {
	get subscribe(): Readable<T>['subscribe'];
	get trigger(): (arg: U) => Promise<T>;
}

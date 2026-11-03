import { captureException } from '@sentry/sveltekit';
import { get, type Readable, type Writable, writable } from 'svelte/store';

import type { ActionStore } from '$lib/shared/adapter/presenter/actionStore';
import { AsyncState } from '$lib/shared/adapter/presenter/asyncState';
import { Strings } from '$lib/strings';

/**
 * Shared implementation for stores that run a use case and expose the result as
 * an `AsyncState`. Returning `null` from `execute` means "not found": the store
 * enters the error state without reporting it to Sentry.
 */
export abstract class BaseActionStore<T, U = void> implements ActionStore<T, U> {
	private readonly state: Writable<AsyncState<T>>;

	protected readonly notFoundMessage: string = Strings.RESOURCE_NOT_FOUND;

	protected constructor(initialData: T | null = null) {
		this.state = writable<AsyncState<T>>(AsyncState.idle<T>(initialData));
	}

	get subscribe(): Readable<AsyncState<T>>['subscribe'] {
		return this.state.subscribe;
	}

	get trigger(): (arg: U) => Promise<AsyncState<T>> {
		return (arg: U) => this.run(arg);
	}

	setData(data: T): void {
		this.state.set(AsyncState.success(data));
	}

	protected abstract execute(arg: U): Promise<T | null>;

	protected async run(arg: U): Promise<AsyncState<T>> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: AsyncState<T>;
		try {
			const data = await this.execute(arg);
			result =
				data === null
					? AsyncState.error(new Error(this.notFoundMessage), get(this.state).data)
					: AsyncState.success(data);
		} catch (error) {
			result = AsyncState.error(error, get(this.state).data);
			captureException(error);
		}

		this.state.set(result);
		return result;
	}
}

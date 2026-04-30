import { get, writable } from 'svelte/store';

import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import type { DeleteImageUseCase } from '$lib/image/application/useCase/deleteImageUseCase';

type VoidState = AsyncState<void>;

export class ImageDeletedStore implements BaseStore<VoidState, number> {
	private readonly state = writable<VoidState>(AsyncState.idle<void>(null));

	constructor(private readonly deleteImageUseCase: DeleteImageUseCase) {}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return (id: number) => this.deleteImage(id);
	}

	private async deleteImage(id: number): Promise<VoidState> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: VoidState;
		try {
			await this.deleteImageUseCase.execute(id);
			result = AsyncState.success(undefined);
		} catch (e) {
			result = AsyncState.error(e, get(this.state).data);
		}

		this.state.set(result);
		return result;
	}
}

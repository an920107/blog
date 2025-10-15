import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
import type { GetAllLabelsUseCase } from '$lib/label/application/useCase/getAllLabelsUseCase';
import { captureException } from '@sentry/sveltekit';
import { get, writable } from 'svelte/store';

type LabelListState = AsyncState<readonly LabelViewModel[]>;

export class LabelsListedStore implements BaseStore<LabelListState, void> {
	private readonly state = writable<LabelListState>(AsyncState.idle([]));

	constructor(
		private readonly getAllLabelsUseCase: GetAllLabelsUseCase,
		initialData?: readonly LabelViewModel[]
	) {
		if (initialData) {
			this.state.set(AsyncState.idle(initialData));
		}
	}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return () => this.loadLabels();
	}

	private async loadLabels(): Promise<LabelListState> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: LabelListState;
		try {
			const labels = await this.getAllLabelsUseCase.execute();
			const labelViewModels = labels.map((label) => LabelViewModel.fromEntity(label));
			result = AsyncState.success(labelViewModels);
		} catch (e) {
			result = AsyncState.error(e, get(this.state).data);
			captureException(e);
		}

		this.state.set(result);
		return result;
	}
}

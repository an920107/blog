import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
import type { GetLabelUseCase } from '$lib/label/application/useCase/getLabelUseCase';
import { captureException } from '@sentry/sveltekit';
import { get, writable } from 'svelte/store';

type LabelState = AsyncState<LabelViewModel>;

export class LabelLoadedStore implements BaseStore<LabelState, number> {
	private state = writable<LabelState>(AsyncState.idle<LabelViewModel>(null));

	constructor(
		private readonly getLabelUseCase: GetLabelUseCase,
		initialData?: LabelViewModel
	) {
		if (initialData) {
			this.state.set(AsyncState.idle(initialData));
		}
	}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return (id: number) => this.loadLabel(id);
	}

	async loadLabel(id: number): Promise<LabelState> {
		this.state.set(AsyncState.loading<LabelViewModel>(get(this.state).data));

		let result: LabelState;
		try {
			const label = await this.getLabelUseCase.execute(id);
			if (!label) {
				result = AsyncState.error(new Error('Label not found'), get(this.state).data);
				this.state.set(result);
				return result;
			}
			const labelViewModel = LabelViewModel.fromEntity(label);
			result = AsyncState.success(labelViewModel);
		} catch (e) {
			result = AsyncState.error(e, get(this.state).data);
			captureException(e);
		}

		this.state.set(result);
		return result;
	}
}

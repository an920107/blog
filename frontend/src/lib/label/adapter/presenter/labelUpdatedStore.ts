import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
import type { UpdateLabelParams } from '$lib/label/application/gateway/labelRepository';
import type { UpdateLabelUseCase } from '$lib/label/application/useCase/updateLabelUseCase';
import { captureException } from '@sentry/sveltekit';
import { get, writable } from 'svelte/store';

type LabelState = AsyncState<LabelViewModel>;

interface UpdateLabelTriggerParams {
	id: number;
	params: UpdateLabelParams;
}

export class LabelUpdatedStore implements BaseStore<LabelState, UpdateLabelTriggerParams> {
	static readonly name = 'LabelUpdatedStore';

	private readonly state = writable<LabelState>(AsyncState.idle<LabelViewModel>(null));

	constructor(private readonly updateLabelUseCase: UpdateLabelUseCase) {}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return (params: UpdateLabelTriggerParams) => this.updateLabel(params);
	}

	private async updateLabel(params: UpdateLabelTriggerParams): Promise<LabelState> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: LabelState;
		try {
			const label = await this.updateLabelUseCase.execute(params.id, params.params);
			const labelViewModel = LabelViewModel.fromEntity(label);
			result = AsyncState.success(labelViewModel);
		} catch (error) {
			captureException(error);
			result = AsyncState.error(error, get(this.state).data);
		}

		this.state.set(result);
		return result;
	}
}

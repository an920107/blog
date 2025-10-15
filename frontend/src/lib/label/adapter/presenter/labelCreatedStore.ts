import { AsyncState } from '$lib/common/adapter/presenter/asyncState';
import type { BaseStore } from '$lib/common/adapter/presenter/baseStore';
import { LabelViewModel } from '$lib/label/adapter/presenter/labelViewModel';
import type { CreateLabelParams } from '$lib/label/application/gateway/labelRepository';
import type { CreateLabelUseCase } from '$lib/label/application/useCase/createLabelUseCase';
import { captureException } from '@sentry/sveltekit';
import { get, writable } from 'svelte/store';

type LabelState = AsyncState<LabelViewModel>;

export class LabelCreatedStore implements BaseStore<LabelState, CreateLabelParams> {
	private readonly state = writable<LabelState>(AsyncState.idle<LabelViewModel>(null));

	constructor(private readonly createLabelUseCase: CreateLabelUseCase) {}

	get subscribe() {
		return this.state.subscribe;
	}

	get trigger() {
		return (params: CreateLabelParams) => this.createLabel(params);
	}

	private async createLabel(params: CreateLabelParams): Promise<LabelState> {
		this.state.set(AsyncState.loading(get(this.state).data));

		let result: LabelState;
		try {
			const label = await this.createLabelUseCase.execute(params);
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

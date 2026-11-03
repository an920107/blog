import { LabelViewModel } from '$lib/label/adapter/presenter/viewModel/labelViewModel';
import type { UpdateLabelParams } from '$lib/label/application/port/labelRepository';
import type { UpdateLabelUseCase } from '$lib/label/application/useCase/updateLabelUseCase';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';

interface LabelUpdatedTriggerParams {
	id: number;
	params: UpdateLabelParams;
}

export class LabelUpdatedStore extends BaseActionStore<LabelViewModel, LabelUpdatedTriggerParams> {
	static readonly name = 'LabelUpdatedStore';

	constructor(private readonly updateLabelUseCase: UpdateLabelUseCase) {
		super();
	}

	protected async execute(params: LabelUpdatedTriggerParams): Promise<LabelViewModel> {
		const label = await this.updateLabelUseCase.execute(params.id, params.params);

		return LabelViewModel.fromEntity(label);
	}
}

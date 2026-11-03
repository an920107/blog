import { LabelViewModel } from '$lib/label/adapter/presenter/viewModel/labelViewModel';
import type { CreateLabelParams } from '$lib/label/application/port/labelRepository';
import type { CreateLabelUseCase } from '$lib/label/application/useCase/createLabelUseCase';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';

export class LabelCreatedStore extends BaseActionStore<LabelViewModel, CreateLabelParams> {
	constructor(private readonly createLabelUseCase: CreateLabelUseCase) {
		super();
	}

	protected async execute(params: CreateLabelParams): Promise<LabelViewModel> {
		const label = await this.createLabelUseCase.execute(params);

		return LabelViewModel.fromEntity(label);
	}
}

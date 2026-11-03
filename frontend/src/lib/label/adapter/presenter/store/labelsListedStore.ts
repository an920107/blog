import { LabelViewModel } from '$lib/label/adapter/presenter/viewModel/labelViewModel';
import type { GetAllLabelsUseCase } from '$lib/label/application/useCase/getAllLabelsUseCase';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';

export class LabelsListedStore extends BaseActionStore<readonly LabelViewModel[]> {
	constructor(
		private readonly getAllLabelsUseCase: GetAllLabelsUseCase,
		initialData?: readonly LabelViewModel[]
	) {
		super(initialData ?? []);
	}

	protected async execute(): Promise<readonly LabelViewModel[]> {
		const labels = await this.getAllLabelsUseCase.execute();

		return labels.map((label) => LabelViewModel.fromEntity(label));
	}
}

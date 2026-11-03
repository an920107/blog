import { LabelViewModel } from '$lib/label/adapter/presenter/viewModel/labelViewModel';
import type { GetLabelUseCase } from '$lib/label/application/useCase/getLabelUseCase';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';
import { Strings } from '$lib/strings';

export class LabelLoadedStore extends BaseActionStore<LabelViewModel, number> {
	protected readonly notFoundMessage = Strings.LABEL_NOT_FOUND;

	constructor(
		private readonly getLabelUseCase: GetLabelUseCase,
		initialData?: LabelViewModel
	) {
		super(initialData ?? null);
	}

	protected async execute(id: number): Promise<LabelViewModel | null> {
		const label = await this.getLabelUseCase.execute(id);

		return label ? LabelViewModel.fromEntity(label) : null;
	}
}

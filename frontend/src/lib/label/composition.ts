import { LabelRepositoryImpl } from '$lib/label/adapter/gateway/labelRepositoryImpl';
import { LabelCreatedStore } from '$lib/label/adapter/presenter/store/labelCreatedStore';
import { LabelLoadedStore } from '$lib/label/adapter/presenter/store/labelLoadedStore';
import { LabelsListedStore } from '$lib/label/adapter/presenter/store/labelsListedStore';
import { LabelUpdatedStore } from '$lib/label/adapter/presenter/store/labelUpdatedStore';
import type { LabelViewModel } from '$lib/label/adapter/presenter/viewModel/labelViewModel';
import type { LabelRepository } from '$lib/label/application/port/labelRepository';
import { CreateLabelUseCase } from '$lib/label/application/useCase/createLabelUseCase';
import { GetAllLabelsUseCase } from '$lib/label/application/useCase/getAllLabelsUseCase';
import { GetLabelUseCase } from '$lib/label/application/useCase/getLabelUseCase';
import { UpdateLabelUseCase } from '$lib/label/application/useCase/updateLabelUseCase';
import { LabelApiServiceImpl } from '$lib/label/framework/api/labelApiServiceImpl';
import type { ApiClient } from '$lib/shared/framework/http/apiClient';

export function createLabelModule(apiClient: ApiClient) {
	const apiService = new LabelApiServiceImpl(apiClient);
	const repository: LabelRepository = new LabelRepositoryImpl(apiService);
	const getAllLabelsUseCase = new GetAllLabelsUseCase(repository);
	const getLabelUseCase = new GetLabelUseCase(repository);
	const createLabelUseCase = new CreateLabelUseCase(repository);
	const updateLabelUseCase = new UpdateLabelUseCase(repository);

	return {
		createLabelsListedStore: (initialData?: readonly LabelViewModel[]) =>
			new LabelsListedStore(getAllLabelsUseCase, initialData),
		createLabelLoadedStore: (initialData?: LabelViewModel) =>
			new LabelLoadedStore(getLabelUseCase, initialData),
		createLabelCreatedStore: () => new LabelCreatedStore(createLabelUseCase),
		createLabelUpdatedStore: () => new LabelUpdatedStore(updateLabelUseCase),
	};
}

export type LabelModule = ReturnType<typeof createLabelModule>;

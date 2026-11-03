import { ImageInfoViewModel } from '$lib/image/adapter/presenter/viewModel/imageInfoViewModel';
import type { ListImagesUseCase } from '$lib/image/application/useCase/listImagesUseCase';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';

export class ImagesListedStore extends BaseActionStore<readonly ImageInfoViewModel[]> {
	constructor(
		private readonly listImagesUseCase: ListImagesUseCase,
		initialData?: readonly ImageInfoViewModel[]
	) {
		super(initialData ?? []);
	}

	protected async execute(): Promise<readonly ImageInfoViewModel[]> {
		const images = await this.listImagesUseCase.execute();

		return images.map((image) => ImageInfoViewModel.fromEntity(image));
	}
}

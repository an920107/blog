import { ImageInfoViewModel } from '$lib/image/adapter/presenter/viewModel/imageInfoViewModel';
import type { GetImageInfoUseCase } from '$lib/image/application/useCase/getImageInfoUseCase';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';

export class ImageLoadedStore extends BaseActionStore<ImageInfoViewModel, number> {
	constructor(
		private readonly getImageInfoUseCase: GetImageInfoUseCase,
		initialData?: ImageInfoViewModel
	) {
		super(initialData ?? null);
	}

	protected async execute(id: number): Promise<ImageInfoViewModel> {
		const image = await this.getImageInfoUseCase.execute(id);

		return ImageInfoViewModel.fromEntity(image);
	}
}

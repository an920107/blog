import { ImageInfoViewModel } from '$lib/image/adapter/presenter/viewModel/imageInfoViewModel';
import type { UploadImageUseCase } from '$lib/image/application/useCase/uploadImageUseCase';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';

export class ImageUploadedStore extends BaseActionStore<ImageInfoViewModel, File> {
	constructor(private readonly uploadImageUseCase: UploadImageUseCase) {
		super();
	}

	protected async execute(file: File): Promise<ImageInfoViewModel> {
		const imageInfo = await this.uploadImageUseCase.execute(file);

		return ImageInfoViewModel.fromEntity(imageInfo);
	}
}

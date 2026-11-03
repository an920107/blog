import { ImageRepositoryImpl } from '$lib/image/adapter/gateway/imageRepositoryImpl';
import { ImageDeletedStore } from '$lib/image/adapter/presenter/store/imageDeletedStore';
import { ImageLoadedStore } from '$lib/image/adapter/presenter/store/imageLoadedStore';
import { ImagesListedStore } from '$lib/image/adapter/presenter/store/imagesListedStore';
import { ImageUploadedStore } from '$lib/image/adapter/presenter/store/imageUploadedStore';
import type { ImageInfoViewModel } from '$lib/image/adapter/presenter/viewModel/imageInfoViewModel';
import type { ImageRepository } from '$lib/image/application/port/imageRepository';
import { DeleteImageUseCase } from '$lib/image/application/useCase/deleteImageUseCase';
import { GetImageInfoUseCase } from '$lib/image/application/useCase/getImageInfoUseCase';
import { ListImagesUseCase } from '$lib/image/application/useCase/listImagesUseCase';
import { UploadImageUseCase } from '$lib/image/application/useCase/uploadImageUseCase';
import { ImageApiServiceImpl } from '$lib/image/framework/api/imageApiServiceImpl';
import type { ApiClient } from '$lib/shared/framework/http/apiClient';

export function createImageModule(apiClient: ApiClient) {
	const apiService = new ImageApiServiceImpl(apiClient);
	const repository: ImageRepository = new ImageRepositoryImpl(apiService);
	const uploadImageUseCase = new UploadImageUseCase(repository);
	const listImagesUseCase = new ListImagesUseCase(repository);
	const getImageInfoUseCase = new GetImageInfoUseCase(repository);
	const deleteImageUseCase = new DeleteImageUseCase(repository);

	return {
		createImageUploadedStore: () => new ImageUploadedStore(uploadImageUseCase),
		createImagesListedStore: (initialData?: readonly ImageInfoViewModel[]) =>
			new ImagesListedStore(listImagesUseCase, initialData),
		createImageLoadedStore: (initialData?: ImageInfoViewModel) =>
			new ImageLoadedStore(getImageInfoUseCase, initialData),
		createImageDeletedStore: () => new ImageDeletedStore(deleteImageUseCase),
	};
}

export type ImageModule = ReturnType<typeof createImageModule>;

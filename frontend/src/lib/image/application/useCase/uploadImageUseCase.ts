import type { ImageRepository } from '$lib/image/application/gateway/imageRepository';
import type { ImageInfo } from '$lib/image/domain/entity/imageInfo';

export class UploadImageUseCase {
	constructor(private readonly imageRepository: ImageRepository) {}

	execute(file: File): Promise<ImageInfo> {
		return this.imageRepository.uploadImage(file);
	}
}

import type { ImageRepository } from '$lib/image/application/gateway/imageRepository';
import type { ImageInfo } from '$lib/image/domain/entity/imageInfo';

export class ListImagesUseCase {
	constructor(private readonly imageRepository: ImageRepository) {}

	async execute(): Promise<ImageInfo[]> {
		return await this.imageRepository.listImages();
	}
}

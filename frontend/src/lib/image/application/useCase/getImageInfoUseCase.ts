import type { ImageRepository } from '$lib/image/application/gateway/imageRepository';
import type { ImageInfo } from '$lib/image/domain/entity/imageInfo';

export class GetImageInfoUseCase {
	constructor(private readonly imageRepository: ImageRepository) {}

	async execute(id: number): Promise<ImageInfo> {
		return this.imageRepository.getImageInfo(id);
	}
}

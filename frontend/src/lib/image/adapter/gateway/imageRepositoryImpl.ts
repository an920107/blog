import type { ImageApiService } from '$lib/image/adapter/gateway/imageApiService';
import type { ImageRepository } from '$lib/image/application/gateway/imageRepository';
import { ImageInfo } from '$lib/image/domain/entity/imageInfo';

export class ImageRepositoryImpl implements ImageRepository {
	constructor(private readonly imageApiService: ImageApiService) {}

	async uploadImage(file: File): Promise<ImageInfo> {
		const dto = await this.imageApiService.uploadImage(file);
		return new ImageInfo({
			id: dto.id,
			mimeType: dto.mimeType,
			url: this.imageApiService.getUrlFromId(dto.id),
		});
	}
}

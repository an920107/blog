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

	async listImages(): Promise<ImageInfo[]> {
		const responseDtos = await this.imageApiService.listImages();
		return responseDtos.map((dto) => dto.toEntity(this.imageApiService.getUrlFromId(dto.id)));
	}

	async getImageInfo(id: number): Promise<ImageInfo> {
		const responseDto = await this.imageApiService.getImageInfo(id);
		return responseDto.toEntity(this.imageApiService.getUrlFromId(id));
	}
}

import { ImageInfoResponseDto } from '$lib/image/adapter/gateway/dto/imageInfoResponseDto';
import type { ImageApiService } from '$lib/image/adapter/port/imageApiService';
import { ApiClient } from '$lib/shared/framework/http/apiClient';

export class ImageApiServiceImpl implements ImageApiService {
	constructor(private readonly apiClient: ApiClient) {}

	uploadImage(file: File): Promise<ImageInfoResponseDto> {
		const formData = new FormData();
		formData.append('file', file);

		return this.apiClient.postForm('image/upload', formData, ImageInfoResponseDto.fromJson);
	}

	listImages(): Promise<ImageInfoResponseDto[]> {
		return this.apiClient.getJsonArray('image', ImageInfoResponseDto.fromJson);
	}

	getImageInfo(id: number): Promise<ImageInfoResponseDto> {
		return this.apiClient.getJson(`image/${id}/info`, ImageInfoResponseDto.fromJson);
	}

	deleteImage(id: number): Promise<void> {
		return this.apiClient.delete(`image/${id}`);
	}

	getUrlFromId(id: number): URL {
		return this.apiClient.url(`image/${id}`);
	}
}

import type { ImageInfoResponseDto } from '$lib/image/adapter/gateway/imageInfoResponseDto';

export interface ImageApiService {
	uploadImage(file: File): Promise<ImageInfoResponseDto>;
	listImages(): Promise<ImageInfoResponseDto[]>;
	getImageInfo(id: number): Promise<ImageInfoResponseDto>;
	deleteImage(id: number): Promise<void>;
	getUrlFromId(id: number): URL;
}

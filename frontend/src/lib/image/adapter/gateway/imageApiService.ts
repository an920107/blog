import type { ImageInfoResponseDto } from '$lib/image/adapter/gateway/imageInfoResponseDto';

export interface ImageApiService {
	uploadImage(file: File): Promise<ImageInfoResponseDto>;
	listImages(): Promise<ImageInfoResponseDto[]>;
	getImageInfo(id: number): Promise<ImageInfoResponseDto>;
	getUrlFromId(id: number): URL;
}

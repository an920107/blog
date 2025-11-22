import type { ImageInfoResponseDto } from '$lib/image/adapter/gateway/imageInfoResponseDto';

export interface ImageApiService {
	uploadImage(file: File): Promise<ImageInfoResponseDto>;
	listImages(): Promise<ImageInfoResponseDto[]>;
	getUrlFromId(id: number): URL;
}

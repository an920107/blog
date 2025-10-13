import type { ImageInfo } from '$lib/image/domain/entity/imageInfo';

export interface ImageRepository {
	uploadImage(file: File): Promise<ImageInfo>;
}

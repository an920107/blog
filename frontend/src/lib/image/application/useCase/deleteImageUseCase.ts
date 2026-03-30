import type { ImageRepository } from '$lib/image/application/gateway/imageRepository';

export class DeleteImageUseCase {
	constructor(private readonly imageRepository: ImageRepository) {}

	async execute(id: number): Promise<void> {
		return this.imageRepository.deleteImage(id);
	}
}

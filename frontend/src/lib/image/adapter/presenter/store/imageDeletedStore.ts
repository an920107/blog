import type { DeleteImageUseCase } from '$lib/image/application/useCase/deleteImageUseCase';
import { BaseActionStore } from '$lib/shared/adapter/presenter/baseActionStore';

export class ImageDeletedStore extends BaseActionStore<void, number> {
	constructor(private readonly deleteImageUseCase: DeleteImageUseCase) {
		super();
	}

	protected async execute(id: number): Promise<void> {
		await this.deleteImageUseCase.execute(id);
	}
}

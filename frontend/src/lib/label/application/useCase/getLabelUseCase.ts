import type { LabelRepository } from '$lib/label/application/gateway/labelRepository';
import type { Label } from '$lib/label/domain/entity/label';

export class GetLabelUseCase {
	constructor(private readonly labelRepository: LabelRepository) {}

	async execute(id: number): Promise<Label | null> {
		return this.labelRepository.getLabelById(id);
	}
}

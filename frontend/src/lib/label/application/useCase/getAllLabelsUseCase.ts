import type { LabelRepository } from '$lib/label/application/port/labelRepository';
import type { Label } from '$lib/label/domain/entity/label';

export class GetAllLabelsUseCase {
	constructor(private readonly labelRepository: LabelRepository) {}

	async execute(): Promise<Label[]> {
		return this.labelRepository.getAllLabels();
	}
}

import type { LabelRepository } from '$lib/label/application/gateway/labelRepository';
import type { Label } from '$lib/label/domain/entity/label';

export class GetAllLabelsUseCase {
	constructor(private readonly labelRepository: LabelRepository) {}

	execute(): Promise<Label[]> {
		return this.labelRepository.getAllLabels();
	}
}

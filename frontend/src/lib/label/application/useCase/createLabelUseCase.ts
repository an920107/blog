import type {
	CreateLabelParams,
	LabelRepository,
} from '$lib/label/application/gateway/labelRepository';
import type { Label } from '$lib/label/domain/entity/label';

export class CreateLabelUseCase {
	constructor(private readonly labelRepository: LabelRepository) {}

	async execute(params: CreateLabelParams): Promise<Label> {
		return this.labelRepository.createLabel(params);
	}
}

import type { Label } from '$lib/label/domain/entity/label';
import type {
	LabelRepository,
	UpdateLabelParams,
} from '$lib/label/application/gateway/labelRepository';

export class UpdateLabelUseCase {
	constructor(private readonly labelRepository: LabelRepository) {}

	async execute(id: number, params: UpdateLabelParams): Promise<Label> {
		return this.labelRepository.updateLabel(id, params);
	}
}

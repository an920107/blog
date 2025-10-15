import { CreateLabelRequestDto } from '$lib/label/adapter/gateway/createLabelRequestDto';
import type { LabelApiService } from '$lib/label/adapter/gateway/labelApiService';
import type {
	CreateLabelParams,
	LabelRepository,
} from '$lib/label/application/gateway/labelRepository';
import type { Label } from '$lib/label/domain/entity/label';

export class LabelRepositoryImpl implements LabelRepository {
	constructor(private readonly labelApiService: LabelApiService) {}

	async getAllLabels(): Promise<Label[]> {
		const dtos = await this.labelApiService.getAllLabels();
		return dtos.map((dto) => dto.toEntity());
	}

	async createLabel(params: CreateLabelParams): Promise<Label> {
		const requestDto = CreateLabelRequestDto.fromParams(params);
		const responseDto = await this.labelApiService.createLabel(requestDto);
		return responseDto.toEntity();
	}
}

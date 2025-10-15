import { CreateLabelRequestDto } from '$lib/label/adapter/gateway/createLabelRequestDto';
import type { LabelApiService } from '$lib/label/adapter/gateway/labelApiService';
import { UpdateLabelRequestDto } from '$lib/label/adapter/gateway/updateLabelRequestDto';
import type {
	CreateLabelParams,
	LabelRepository,
	UpdateLabelParams,
} from '$lib/label/application/gateway/labelRepository';
import type { Label } from '$lib/label/domain/entity/label';

export class LabelRepositoryImpl implements LabelRepository {
	constructor(private readonly labelApiService: LabelApiService) {}

	async getAllLabels(): Promise<Label[]> {
		const dtos = await this.labelApiService.getAllLabels();
		return dtos.map((dto) => dto.toEntity());
	}

	async getLabelById(id: number): Promise<Label | null> {
		const dto = await this.labelApiService.getLabelById(id);
		return dto?.toEntity() ?? null;
	}

	async createLabel(params: CreateLabelParams): Promise<Label> {
		const requestDto = CreateLabelRequestDto.fromParams(params);
		const responseDto = await this.labelApiService.createLabel(requestDto);
		return responseDto.toEntity();
	}

	async updateLabel(id: number, params: UpdateLabelParams): Promise<Label> {
		const requestDto = UpdateLabelRequestDto.fromParams(params);
		const responseDto = await this.labelApiService.updateLabel(id, requestDto);
		return responseDto.toEntity();
	}
}

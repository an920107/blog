import type { CreateLabelRequestDto } from '$lib/label/adapter/gateway/dto/createLabelRequestDto';
import type { LabelResponseDto } from '$lib/label/adapter/gateway/dto/labelResponseDto';
import type { UpdateLabelRequestDto } from '$lib/label/adapter/gateway/dto/updateLabelRequestDto';

export interface LabelApiService {
	getAllLabels(): Promise<LabelResponseDto[]>;
	getLabelById(id: number): Promise<LabelResponseDto | null>;
	createLabel(payload: CreateLabelRequestDto): Promise<LabelResponseDto>;
	updateLabel(id: number, payload: UpdateLabelRequestDto): Promise<LabelResponseDto>;
}

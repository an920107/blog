import type { CreateLabelRequestDto } from '$lib/label/adapter/gateway/createLabelRequestDto';
import type { LabelResponseDto } from '$lib/label/adapter/gateway/labelResponseDto';
import type { UpdateLabelRequestDto } from '$lib/label/adapter/gateway/updateLabelRequestDto';

export interface LabelApiService {
	getAllLabels(): Promise<LabelResponseDto[]>;
	getLabelById(id: number): Promise<LabelResponseDto | null>;
	createLabel(payload: CreateLabelRequestDto): Promise<LabelResponseDto>;
	updateLabel(id: number, payload: UpdateLabelRequestDto): Promise<LabelResponseDto>;
}

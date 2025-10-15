import type { CreateLabelRequestDto } from '$lib/label/adapter/gateway/createLabelRequestDto';
import type { LabelResponseDto } from '$lib/label/adapter/gateway/labelResponseDto';

export interface LabelApiService {
	getAllLabels(): Promise<LabelResponseDto[]>;
	createLabel(payload: CreateLabelRequestDto): Promise<LabelResponseDto>;
}

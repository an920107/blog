import type { CreateLabelRequestDto } from '$lib/label/adapter/gateway/dto/createLabelRequestDto';
import { LabelResponseDto } from '$lib/label/adapter/gateway/dto/labelResponseDto';
import type { UpdateLabelRequestDto } from '$lib/label/adapter/gateway/dto/updateLabelRequestDto';
import type { LabelApiService } from '$lib/label/adapter/port/labelApiService';
import { ApiClient } from '$lib/shared/framework/http/apiClient';
import { HttpStatusCode } from '$lib/shared/framework/http/httpStatusCode';

export class LabelApiServiceImpl implements LabelApiService {
	constructor(private readonly apiClient: ApiClient) {}

	getAllLabels(): Promise<LabelResponseDto[]> {
		return this.apiClient.getJsonArray('label', LabelResponseDto.fromJson);
	}

	getLabelById(id: number): Promise<LabelResponseDto | null> {
		return this.apiClient.getJsonOrNull(`label/${id}`, LabelResponseDto.fromJson, [
			HttpStatusCode.NOT_FOUND,
		]);
	}

	createLabel(payload: CreateLabelRequestDto): Promise<LabelResponseDto> {
		return this.apiClient.postJson('label', payload.toJson(), LabelResponseDto.fromJson);
	}

	updateLabel(id: number, payload: UpdateLabelRequestDto): Promise<LabelResponseDto> {
		return this.apiClient.putJson(`label/${id}`, payload.toJson(), LabelResponseDto.fromJson);
	}
}

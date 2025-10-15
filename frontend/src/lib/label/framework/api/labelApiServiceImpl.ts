import { HttpError } from '$lib/common/framework/web/httpError';
import { Environment } from '$lib/environment';
import type { CreateLabelRequestDto } from '$lib/label/adapter/gateway/createLabelRequestDto';
import type { LabelApiService } from '$lib/label/adapter/gateway/labelApiService';
import { LabelResponseDto } from '$lib/label/adapter/gateway/labelResponseDto';
import type { UpdateLabelRequestDto } from '$lib/label/adapter/gateway/updateLabelRequestDto';

export class LabelApiServiceImpl implements LabelApiService {
	constructor(private readonly fetchFn: typeof fetch) {}

	async getAllLabels(): Promise<LabelResponseDto[]> {
		const url = new URL('label', Environment.API_BASE_URL);

		const response = await this.fetchFn(url);

		if (!response.ok) {
			throw new HttpError(response.status, url);
		}

		const data = await response.json();
		return data.map(LabelResponseDto.fromJson);
	}

	async getLabelById(id: number): Promise<LabelResponseDto | null> {
		const url = new URL(`label/${id}`, Environment.API_BASE_URL);

		const response = await this.fetchFn(url);

		if (response.status === 404) {
			return null;
		}
		if (!response.ok) {
			throw new HttpError(response.status, url);
		}

		const data = await response.json();
		return LabelResponseDto.fromJson(data);
	}

	async createLabel(payload: CreateLabelRequestDto): Promise<LabelResponseDto> {
		const url = new URL('label', Environment.API_BASE_URL);

		const response = await this.fetchFn(url, {
			method: 'POST',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(payload.toJson()),
		});

		if (!response.ok) {
			throw new HttpError(response.status, url);
		}

		const data = await response.json();
		return LabelResponseDto.fromJson(data);
	}

	async updateLabel(id: number, payload: UpdateLabelRequestDto): Promise<LabelResponseDto> {
		const url = new URL(`label/${id}`, Environment.API_BASE_URL);

		const response = await this.fetchFn(url, {
			method: 'PUT',
			headers: { 'Content-Type': 'application/json' },
			body: JSON.stringify(payload.toJson()),
		});

		if (!response.ok) {
			throw new HttpError(response.status, url);
		}

		const data = await response.json();
		return LabelResponseDto.fromJson(data);
	}
}

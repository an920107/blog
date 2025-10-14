import { HttpError } from '$lib/common/framework/web/httpError';
import { Environment } from '$lib/environment';
import type { ImageApiService } from '$lib/image/adapter/gateway/imageApiService';
import { ImageInfoResponseDto } from '$lib/image/adapter/gateway/imageInfoResponseDto';

export class ImageApiServiceImpl implements ImageApiService {
	constructor(private readonly fetchFn: typeof fetch) {}

	async uploadImage(file: File): Promise<ImageInfoResponseDto> {
		const url = new URL('image/upload', Environment.API_BASE_URL);

		const formData = new FormData();
		formData.append('file', file);

		const response = await this.fetchFn(url, {
			method: 'POST',
			body: formData,
		});

		if (!response.ok) {
			throw new HttpError(response.status, url);
		}

		const data = await response.json();
		return ImageInfoResponseDto.fromJson(data);
	}

	getUrlFromId(id: number): URL {
		return new URL(`image/${id}`, Environment.API_BASE_URL);
	}
}

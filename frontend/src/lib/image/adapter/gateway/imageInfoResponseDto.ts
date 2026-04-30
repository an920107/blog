import z from 'zod';

import { ImageInfo } from '$lib/image/domain/entity/imageInfo';

export const imageInfoResponseSchema = z.object({
	id: z.int32(),
	mime_type: z.string(),
	is_referred: z.boolean(),
});

export class ImageInfoResponseDto {
	readonly id: number;
	readonly mimeType: string;
	readonly isReferred: boolean;

	private constructor(props: { id: number; mimeType: string; isReferred: boolean }) {
		this.id = props.id;
		this.mimeType = props.mimeType;
		this.isReferred = props.isReferred;
	}

	static fromJson(json: unknown): ImageInfoResponseDto {
		const parsedJson = imageInfoResponseSchema.parse(json);
		return new ImageInfoResponseDto({
			id: parsedJson.id,
			mimeType: parsedJson.mime_type,
			isReferred: parsedJson.is_referred,
		});
	}

	toEntity(url: URL): ImageInfo {
		return new ImageInfo({
			id: this.id,
			mimeType: this.mimeType,
			url: url,
			isReferred: this.isReferred,
		});
	}
}

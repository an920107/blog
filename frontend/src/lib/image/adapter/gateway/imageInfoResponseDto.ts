import z from 'zod';

export const imageInfoResponseSchema = z.object({
	id: z.int32(),
	mime_type: z.string(),
});

export class ImageInfoResponseDto {
	readonly id: number;
	readonly mimeType: string;

	private constructor(props: { id: number; mimeType: string }) {
		this.id = props.id;
		this.mimeType = props.mimeType;
	}

	static fromJson(json: unknown): ImageInfoResponseDto {
		const parsedJson = imageInfoResponseSchema.parse(json);
		return new ImageInfoResponseDto({
			id: parsedJson.id,
			mimeType: parsedJson.mime_type,
		});
	}
}

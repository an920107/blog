import z from 'zod';

import { LabelResponseDto, labelResponseSchema } from '$lib/label/adapter/gateway/labelResponseDto';
import { PostInfo } from '$lib/post/domain/entity/postInfo';

export const postInfoResponseSchema = z.object({
	id: z.int32(),
	semantic_id: z.string(),
	title: z.string(),
	description: z.string(),
	preview_image_url: z.string().nullable(),
	preview_image_mime_type: z.string().nullable(),
	preview_image_size: z.int().nullable(),
	labels: z.array(labelResponseSchema),
	published_time: z.iso.datetime({ offset: true }).nullable(),
	updated_time: z.iso.datetime({ offset: true }).nullable(),
});

export class PostInfoResponseDto {
	readonly id: number;
	readonly semanticId: string;
	readonly title: string;
	readonly description: string;
	readonly previewImageUrl: string | null;
	readonly previewImageMimeType: string | null;
	readonly previewImageSize: number | null;
	readonly labels: readonly LabelResponseDto[];
	readonly publishedTime: Date | null;
	readonly updatedTime: Date | null;

	private constructor(props: {
		id: number;
		semanticId: string;
		title: string;
		description: string;
		previewImageUrl: string | null;
		previewImageMimeType: string | null;
		previewImageSize: number | null;
		labels: LabelResponseDto[];
		publishedTime: Date | null;
		updatedTime: Date | null;
	}) {
		this.id = props.id;
		this.semanticId = props.semanticId;
		this.title = props.title;
		this.description = props.description;
		this.previewImageUrl = props.previewImageUrl;
		this.previewImageMimeType = props.previewImageMimeType;
		this.previewImageSize = props.previewImageSize;
		this.labels = props.labels;
		this.publishedTime = props.publishedTime;
		this.updatedTime = props.updatedTime;
	}

	static fromJson(json: unknown): PostInfoResponseDto {
		const parsedJson = postInfoResponseSchema.parse(json);

		let published_time: Date | null = null;
		if (parsedJson.published_time !== null) {
			published_time = new Date(parsedJson.published_time);
		}

		let updated_time: Date | null = null;
		if (parsedJson.updated_time !== null) {
			updated_time = new Date(parsedJson.updated_time);
		}

		return new PostInfoResponseDto({
			id: parsedJson.id,
			semanticId: parsedJson.semantic_id,
			title: parsedJson.title,
			description: parsedJson.description,
			previewImageUrl: parsedJson.preview_image_url,
			previewImageMimeType: parsedJson.preview_image_mime_type,
			previewImageSize: parsedJson.preview_image_size,
			labels: parsedJson.labels.map((label) => LabelResponseDto.fromJson(label)),
			publishedTime: published_time,
			updatedTime: updated_time,
		});
	}

	toEntity(): PostInfo {
		return new PostInfo({
			id: this.id,
			semanticId: this.semanticId,
			title: this.title,
			description: this.description,
			previewImageUrl: this.previewImageUrl,
			previewImageMimeType: this.previewImageMimeType,
			previewImageSize: this.previewImageSize,
			labels: this.labels.map((label) => label.toEntity()),
			publishedTime: this.publishedTime,
			updatedTime: this.updatedTime,
		});
	}
}

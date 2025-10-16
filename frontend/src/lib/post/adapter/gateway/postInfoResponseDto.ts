import { LabelResponseDto, labelResponseSchema } from '$lib/label/adapter/gateway/labelResponseDto';
import { PostInfo } from '$lib/post/domain/entity/postInfo';
import z from 'zod';

export const postInfoResponseSchema = z.object({
	id: z.int32(),
	semantic_id: z.string(),
	title: z.string(),
	description: z.string(),
	preview_image_url: z.httpUrl().nullable(),
	labels: z.array(labelResponseSchema),
	published_time: z.iso.datetime({ offset: true }).nullable(),
});

export class PostInfoResponseDto {
	readonly id: number;
	readonly semanticId: string;
	readonly title: string;
	readonly description: string;
	readonly previewImageUrl: URL | null;
	readonly labels: readonly LabelResponseDto[];
	readonly publishedTime: Date | null;

	private constructor(props: {
		id: number;
		semanticId: string;
		title: string;
		description: string;
		previewImageUrl: URL | null;
		labels: LabelResponseDto[];
		publishedTime: Date | null;
	}) {
		this.id = props.id;
		this.semanticId = props.semanticId;
		this.title = props.title;
		this.description = props.description;
		this.previewImageUrl = props.previewImageUrl;
		this.labels = props.labels;
		this.publishedTime = props.publishedTime;
	}

	static fromJson(json: unknown): PostInfoResponseDto {
		const parsedJson = postInfoResponseSchema.parse(json);

		let published_time: Date | null = null;
		if (parsedJson.published_time !== null) {
			published_time = new Date(parsedJson.published_time);
		}

		let preview_image_url: URL | null = null;
		if (parsedJson.preview_image_url !== null) {
			preview_image_url = new URL(parsedJson.preview_image_url);
		}

		return new PostInfoResponseDto({
			id: parsedJson.id,
			semanticId: parsedJson.semantic_id,
			title: parsedJson.title,
			description: parsedJson.description,
			previewImageUrl: preview_image_url,
			labels: parsedJson.labels.map((label) => LabelResponseDto.fromJson(label)),
			publishedTime: published_time,
		});
	}

	toEntity(): PostInfo {
		return new PostInfo({
			id: this.id,
			semanticId: this.semanticId,
			title: this.title,
			description: this.description,
			previewImageUrl: this.previewImageUrl,
			labels: this.labels.map((label) => label.toEntity()),
			publishedTime: this.publishedTime,
		});
	}
}

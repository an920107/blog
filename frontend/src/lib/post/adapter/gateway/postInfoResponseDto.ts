import { LabelResponseDto, LabelResponseSchema } from '$lib/post/adapter/gateway/labelResponseDto';
import { PostInfo } from '$lib/post/domain/entity/postInfo';
import z from 'zod';

export const PostInfoResponseSchema = z.object({
	id: z.int32(),
	title: z.string(),
	description: z.string(),
	preview_image_url: z.url(),
	labels: z.array(LabelResponseSchema),
	published_time: z.iso.datetime({ offset: true })
});

export class PostInfoResponseDto {
	readonly id: number;
	readonly title: string;
	readonly description: string;
	readonly previewImageUrl: URL;
	readonly labels: readonly LabelResponseDto[];
	readonly publishedTime: Date;

	private constructor(props: {
		id: number;
		title: string;
		description: string;
		previewImageUrl: URL;
		labels: LabelResponseDto[];
		publishedTime: Date;
	}) {
		this.id = props.id;
		this.title = props.title;
		this.description = props.description;
		this.previewImageUrl = props.previewImageUrl;
		this.labels = props.labels;
		this.publishedTime = props.publishedTime;
	}

	static fromJson(json: unknown): PostInfoResponseDto {
		const parsedJson = PostInfoResponseSchema.parse(json);
		return new PostInfoResponseDto({
			id: parsedJson.id,
			title: parsedJson.title,
			description: parsedJson.description,
			previewImageUrl: new URL(parsedJson.preview_image_url),
			labels: parsedJson.labels.map((label) => LabelResponseDto.fromJson(label)),
			publishedTime: new Date(parsedJson.published_time)
		});
	}

	toEntity(): PostInfo {
		return new PostInfo({
			id: this.id,
			title: this.title,
			description: this.description,
			previewImageUrl: this.previewImageUrl,
			labels: this.labels.map((label) => label.toEntity()),
			publishedTime: this.publishedTime
		});
	}
}

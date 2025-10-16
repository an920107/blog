import type { UpdatePostParams } from '$lib/post/application/gateway/postRepository';

export class UpdatePostRequestDto {
	semanticId: string;
	title: string;
	description: string;
	content: string;
	labelIds: number[];
	previewImageUrl: string | null;
	publishedTime: string | null;

	private constructor(props: {
		semanticId: string;
		title: string;
		description: string;
		content: string;
		labelIds: number[];
		previewImageUrl: string | null;
		publishedTime: string | null;
	}) {
		this.semanticId = props.semanticId;
		this.title = props.title;
		this.description = props.description;
		this.content = props.content;
		this.labelIds = props.labelIds;
		this.previewImageUrl = props.previewImageUrl;
		this.publishedTime = props.publishedTime;
	}

	static fromParams(params: UpdatePostParams): UpdatePostRequestDto {
		return new UpdatePostRequestDto({
			...params,
			previewImageUrl: params.previewImageUrl?.href ?? null,
			publishedTime: params.publishedTime?.toISOString() ?? null,
		});
	}

	toJson() {
		return {
			semantic_id: this.semanticId,
			title: this.title,
			description: this.description,
			content: this.content,
			label_ids: this.labelIds,
			preview_image_url: this.previewImageUrl,
			published_time: this.publishedTime,
		};
	}
}

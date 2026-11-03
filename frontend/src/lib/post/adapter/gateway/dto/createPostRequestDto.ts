import type { CreatePostDraftParams } from '$lib/post/application/port/postRepository';

export class CreatePostRequestDto {
	readonly semanticId: string;
	readonly title: string;
	readonly description: string;
	readonly content: string;
	readonly labelIds: number[];
	readonly previewImageUrl: string | null;
	readonly publishedTime: Date | null;

	private constructor(props: {
		semanticId: string;
		title: string;
		description: string;
		content: string;
		labelIds: number[];
		previewImageUrl: string | null;
		publishedTime: Date | null;
	}) {
		this.semanticId = props.semanticId;
		this.title = props.title;
		this.description = props.description;
		this.content = props.content;
		this.labelIds = props.labelIds;
		this.previewImageUrl = props.previewImageUrl;
		this.publishedTime = props.publishedTime;
	}

	/**
	 * A draft only carries its identity fields. The backend create payload expects
	 * the full post shape, so content-related fields start empty.
	 */
	static fromParams(params: CreatePostDraftParams): CreatePostRequestDto {
		return new CreatePostRequestDto({
			...params,
			description: '',
			content: '',
			labelIds: [],
			previewImageUrl: null,
			publishedTime: null,
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

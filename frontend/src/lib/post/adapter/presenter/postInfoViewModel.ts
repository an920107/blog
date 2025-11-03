import { EnhancedDate } from '$lib/common/adapter/presenter/enhancedDate';
import {
	LabelViewModel,
	type DehydratedLabelProps,
} from '$lib/label/adapter/presenter/labelViewModel';
import type { PostInfo } from '$lib/post/domain/entity/postInfo';

export class PostInfoViewModel {
	readonly id: number;
	readonly semanticId: string;
	readonly title: string;
	readonly description: string;
	readonly previewImageUrl: URL | null;
	readonly labels: readonly LabelViewModel[];
	readonly publishedTime: EnhancedDate | null;

	private constructor(props: {
		id: number;
		semanticId: string;
		title: string;
		description: string;
		previewImageUrl: URL | null;
		labels: readonly LabelViewModel[];
		publishedTime: EnhancedDate | null;
	}) {
		this.id = props.id;
		this.semanticId = props.semanticId;
		this.title = props.title;
		this.description = props.description;
		this.previewImageUrl = props.previewImageUrl;
		this.labels = props.labels;
		this.publishedTime = props.publishedTime;
	}

	static fromEntity(postInfo: PostInfo): PostInfoViewModel {
		return new PostInfoViewModel({
			id: postInfo.id,
			semanticId: postInfo.semanticId,
			title: postInfo.title,
			description: postInfo.description,
			previewImageUrl: postInfo.previewImageUrl,
			labels: postInfo.labels.map((label) => LabelViewModel.fromEntity(label)),
			publishedTime: postInfo.publishedTime ? new EnhancedDate(postInfo.publishedTime) : null,
		});
	}

	static rehydrate(props: DehydratedPostInfoProps): PostInfoViewModel {
		let publishedTime: Date | null = null;
		if (props.publishedTime !== null) {
			publishedTime = new Date(props.publishedTime);
		}

		let previewImageUrl: URL | null = null;
		if (props.previewImageUrl) {
			previewImageUrl = new URL(props.previewImageUrl);
		}

		return new PostInfoViewModel({
			id: props.id,
			semanticId: props.semanticId,
			title: props.title,
			description: props.description,
			previewImageUrl: previewImageUrl,
			labels: props.labels.map((label) => LabelViewModel.rehydrate(label)),
			publishedTime: publishedTime ? new EnhancedDate(publishedTime) : null,
		});
	}

	get isPublished(): boolean {
		this.publishedTime?.toLocaleString();
		return this.publishedTime !== null;
	}

	dehydrate(): DehydratedPostInfoProps {
		return {
			id: this.id,
			semanticId: this.semanticId,
			title: this.title,
			description: this.description,
			previewImageUrl: this.previewImageUrl?.href ?? null,
			labels: this.labels.map((label) => label.dehydrate()),
			publishedTime: this.publishedTime?.getTime() ?? null,
		};
	}
}

export interface DehydratedPostInfoProps {
	id: number;
	semanticId: string;
	title: string;
	description: string;
	previewImageUrl: string | null;
	labels: DehydratedLabelProps[];
	publishedTime: number | null;
}

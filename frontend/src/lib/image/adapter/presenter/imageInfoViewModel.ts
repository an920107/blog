import type { ImageInfo } from '$lib/image/domain/entity/imageInfo';

export class ImageInfoViewModel {
	readonly id: number;
	readonly mimeType: string;
	readonly url: URL;
	readonly isReferred: boolean;

	private constructor(props: { id: number; mimeType: string; url: URL; isReferred: boolean }) {
		this.id = props.id;
		this.mimeType = props.mimeType;
		this.url = props.url;
		this.isReferred = props.isReferred;
	}

	static fromEntity(imageInfo: ImageInfo): ImageInfoViewModel {
		return new ImageInfoViewModel(imageInfo);
	}

	static rehydrate(props: DehydratedImageInfoProps): ImageInfoViewModel {
		return new ImageInfoViewModel({
			id: props.id,
			mimeType: props.mimeType,
			url: new URL(props.url),
			isReferred: props.isReferred,
		});
	}

	dehydrate(): DehydratedImageInfoProps {
		return {
			id: this.id,
			mimeType: this.mimeType,
			url: this.url.href,
			isReferred: this.isReferred,
		};
	}
}

export interface DehydratedImageInfoProps {
	id: number;
	mimeType: string;
	url: string;
	isReferred: boolean;
}

import type { ImageInfo } from '$lib/image/domain/entity/imageInfo';

export class ImageInfoViewModel {
	readonly id: number;
	readonly mimeType: string;
	readonly url: URL;

	private constructor(props: { id: number; mimeType: string; url: URL }) {
		this.id = props.id;
		this.mimeType = props.mimeType;
		this.url = props.url;
	}

	static fromEntity(imageInfo: ImageInfo): ImageInfoViewModel {
		return new ImageInfoViewModel(imageInfo);
	}

	static rehydrate(props: DehydratedImageInfoProps): ImageInfoViewModel {
		return new ImageInfoViewModel({
			id: props.id,
			mimeType: props.mimeType,
			url: new URL(props.url),
		});
	}

	dehydrate(): DehydratedImageInfoProps {
		return {
			id: this.id,
			mimeType: this.mimeType,
			url: this.url.href,
		};
	}
}

export interface DehydratedImageInfoProps {
	id: number;
	mimeType: string;
	url: string;
}

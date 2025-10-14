export class PostListQueryDto {
	readonly showUnpublished: boolean;

	constructor(props: { showUnpublished: boolean }) {
		this.showUnpublished = props.showUnpublished;
	}

	toSearchParams(): URLSearchParams {
		const params = new URLSearchParams();
		params.append('is_published_only', (!this.showUnpublished).toString());
		return params;
	}
}

/**
 * A published post projected as a public page. It is the shared unit consumed by both
 * sitemap.xml and feed.xml. Non-post pages (home page, post list, terms, ...) are only
 * relevant to the sitemap and are handled where the sitemap is built.
 */
export class PostPage {
	readonly url: URL;
	/**
	 * The post's stable semantic id. It identifies the post in feed.xml, so it must not change
	 * when the public URL scheme changes, or subscribers would re-surface every post as new.
	 */
	readonly semanticId: string;
	readonly title: string;
	readonly description: string;
	readonly publishedTime: Date;
	readonly lastModified: Date;
	readonly previewImageUrl: URL | null;
	readonly previewImageMimeType: string | null;
	readonly previewImageSize: number | null;
	readonly categories: readonly string[];

	constructor(props: {
		url: URL;
		semanticId: string;
		title: string;
		description: string;
		publishedTime: Date;
		lastModified: Date;
		previewImageUrl: URL | null;
		previewImageMimeType: string | null;
		previewImageSize: number | null;
		categories: readonly string[];
	}) {
		this.url = props.url;
		this.semanticId = props.semanticId;
		this.title = props.title;
		this.description = props.description;
		this.publishedTime = props.publishedTime;
		this.lastModified = props.lastModified;
		this.previewImageUrl = props.previewImageUrl;
		this.previewImageMimeType = props.previewImageMimeType;
		this.previewImageSize = props.previewImageSize;
		this.categories = props.categories;
	}
}

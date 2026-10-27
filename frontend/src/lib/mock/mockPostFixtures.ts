export interface MockColor {
	readonly red: number;
	readonly green: number;
	readonly blue: number;
	readonly alpha: number;
}

export interface MockLabel {
	readonly id: number;
	readonly name: string;
	readonly color: MockColor;
}

export interface MockPostInfo {
	readonly id: number;
	readonly semantic_id: string;
	readonly title: string;
	readonly description: string;
	readonly preview_image_url: string | null;
	readonly preview_image_mime_type: string | null;
	readonly preview_image_size: number | null;
	readonly labels: readonly MockLabel[];
	readonly published_time: string | null;
	readonly updated_time: string | null;
}

export interface MockPost extends MockPostInfo {
	readonly content: string;
}

const label = (id: number, name: string, red: number, green: number, blue: number): MockLabel => ({
	id,
	name,
	color: { red, green, blue, alpha: 255 },
});

const svelte = label(1, 'Svelte', 255, 62, 0);
const rust = label(2, 'Rust', 222, 165, 132);
const frontend = label(3, '前端', 59, 130, 246);
const backend = label(4, '後端', 16, 185, 129);
const architecture = label(5, '架構', 139, 92, 246);
const life = label(6, '生活', 245, 158, 11);
const database = label(7, '資料庫', 14, 165, 233);
const testing = label(8, '測試', 239, 68, 68);
const tooling = label(9, '工具鏈', 107, 114, 128);

export const mockLabels: readonly MockLabel[] = [
	svelte,
	rust,
	frontend,
	backend,
	architecture,
	life,
	database,
	testing,
	tooling,
];

const previewImage = (
	id: number
): Pick<MockPostInfo, 'preview_image_url' | 'preview_image_mime_type' | 'preview_image_size'> => ({
	preview_image_url: `https://picsum.photos/seed/blog-mock-${id}/960/540`,
	preview_image_mime_type: 'image/jpeg',
	preview_image_size: 180_000 + id * 1_000,
});

const buildContent = (lead: string): string => `## 前言

${lead}

## 準備工作

這段是 mock 內容，用來確認文章頁的排版：標題層級、段落、清單、引言與程式碼區塊。

### 檢查清單

- 標題與內文的間距
- 清單項目的縮排
- 程式碼區塊的水平滾動

\`\`\`ts
const posts = await fetchPosts();

console.log(posts.length);
\`\`\`

### 引言

> 這是一段用來確認引言樣式的假文字。

## 結語

以上皆為 mock 資料，不會影響正式環境。
`;

export const mockPosts: readonly MockPost[] = [
	{
		id: 1,
		semantic_id: 'svelte-5-runes-notes',
		title: 'Svelte 5 Runes 筆記：從 store 到 rune 的思維轉換',
		description:
			'把既有專案從 Svelte 4 的 store 逐步搬到 runes 的過程，包含 $state、$derived 與 $effect 的使用時機，以及幾個實際踩過的坑。',
		...previewImage(1),
		labels: [svelte, frontend, architecture, tooling],
		published_time: '2026-10-12T21:30:00+08:00',
		updated_time: '2026-10-14T10:00:00+08:00',
		content: buildContent(
			'這篇筆記整理我將部落格前端從 store 遷移到 runes 的步驟，以及遷移後程式的可讀性變化。'
		),
	},
	{
		id: 2,
		semantic_id: 'rust-blog-backend',
		title: '用 Rust 重寫部落格後端：我學到的五件事',
		description:
			'從 Go 換到 Rust 之後，編譯器幫我擋下了哪些錯？這篇記錄重寫過程中的取捨，以及非同步、錯誤處理與測試策略的調整。',
		...previewImage(2),
		labels: [rust, backend],
		published_time: '2026-09-28T09:15:00+08:00',
		updated_time: '2026-09-28T09:15:00+08:00',
		content: buildContent('重寫後端最花時間的不是語法，而是重新思考資料流與錯誤處理的邊界。'),
	},
	{
		id: 3,
		semantic_id: 'sveltekit-bun-migration',
		title: '從 Node 到 Bun：SvelteKit 部署遷移實戰',
		description:
			'把 SvelteKit 的部署目標從 Node adapter 換成 Bun adapter，並整理容器映像、啟動腳本與 CI 流程需要一起調整的地方。',
		...previewImage(3),
		labels: [tooling, frontend, svelte],
		published_time: '2026-09-05T14:00:00+08:00',
		updated_time: '2026-09-06T08:30:00+08:00',
		content: buildContent('遷移本身不難，難的是讓 CI、映像與本機開發環境維持一致。'),
	},
	{
		id: 4,
		semantic_id: 'tailwind-4-upgrade',
		title: 'Tailwind CSS 4 升級筆記：那些文件沒說的事',
		description:
			'升級到 Tailwind 4 之後遇到的主題變數、自訂工具類與建置速度變化，還有幾個需要回頭修改既有樣式的地方。',
		...previewImage(4),
		labels: [frontend, svelte],
		published_time: '2026-08-19T20:05:00+08:00',
		updated_time: '2026-08-20T12:10:00+08:00',
		content: buildContent('升級過程比預期順利，但主題變數的命名規則需要重新整理。'),
	},
	{
		id: 5,
		semantic_id: 'image-pipeline-module',
		title: '把圖片處理拆成獨立模組的理由',
		description:
			'圖片上傳、縮放與格式轉換原本散落在各處，拆成獨立模組後，測試與快取策略都變得清楚許多。',
		...previewImage(5),
		labels: [rust, architecture, backend],
		published_time: '2026-08-01T11:45:00+08:00',
		updated_time: '2026-08-01T11:45:00+08:00',
		content: buildContent('模組邊界清楚之後，圖片相關的錯誤也更容易定位。'),
	},
	{
		id: 6,
		semantic_id: 'remote-work-japan',
		title: '在日本遠端工作的三個月',
		description: '時區、溝通節奏與生活安排的小觀察，以及我怎麼調整工作與休息的界線。',
		...previewImage(6),
		labels: [],
		published_time: '2026-07-14T18:20:00+08:00',
		updated_time: '2026-07-15T09:00:00+08:00',
		content: buildContent('遠端工作的關鍵不是工具，而是把界線說清楚。'),
	},
	{
		id: 7,
		semantic_id: 'opentelemetry-personal-project',
		title: '用 OpenTelemetry 觀察個人專案：從零到第一條 trace',
		description:
			'在小型專案裡導入 OpenTelemetry 的成本比想像中低，這篇記錄 collector 設定、trace 命名與實際排查問題的經驗。',
		...previewImage(7),
		labels: [backend, tooling],
		published_time: '2026-06-30T08:40:00+08:00',
		updated_time: '2026-07-01T10:25:00+08:00',
		content: buildContent('第一條 trace 出現之後，才真正感受到可觀測性的價值。'),
	},
	{
		id: 8,
		semantic_id: 'label-system-design',
		title: '從零設計標籤系統：資料模型與查詢效能',
		description:
			'標籤看起來只是多對多關聯，但查詢模式、排序與快取策略會決定資料表要怎麼設計。這篇整理我最後採用的方案。',
		preview_image_url: null,
		preview_image_mime_type: null,
		preview_image_size: null,
		labels: [architecture, database, backend],
		published_time: '2026-06-12T22:10:00+08:00',
		updated_time: '2026-06-13T09:45:00+08:00',
		content: buildContent('先確定查詢模式，再決定正規化程度，比反過來容易得多。'),
	},
	{
		id: 9,
		semantic_id: 'frontend-mock-strategy',
		title: '沒有後端也能開發：我的前端 Mock 策略',
		description:
			'用一層可切換的 fetch 包裝，讓前端在沒有後端的情況下也能載入假資料，同時不影響正式環境的程式路徑。',
		...previewImage(9),
		labels: [frontend, testing],
		published_time: '2026-05-24T16:30:00+08:00',
		updated_time: '2026-05-25T11:00:00+08:00',
		content: buildContent('Mock 的目的是確認版面與互動，不是取代整合測試。'),
	},
	{
		id: 10,
		semantic_id: 'draft-post',
		title: '草稿：還沒想好標題的文章',
		description: '這篇沒有發布時間，只會在開啟 showUnpublished 時出現。',
		...previewImage(10),
		labels: [life],
		published_time: null,
		updated_time: '2026-10-20T12:00:00+08:00',
		content: buildContent('草稿內容。'),
	},
];

export function toPostInfo(post: MockPost): MockPostInfo {
	return {
		id: post.id,
		semantic_id: post.semantic_id,
		title: post.title,
		description: post.description,
		preview_image_url: post.preview_image_url,
		preview_image_mime_type: post.preview_image_mime_type,
		preview_image_size: post.preview_image_size,
		labels: post.labels,
		published_time: post.published_time,
		updated_time: post.updated_time,
	};
}

export function toPostResponse(post: MockPost): {
	id: number;
	info: MockPostInfo;
	content: string;
} {
	return {
		id: post.id,
		info: toPostInfo(post),
		content: post.content,
	};
}

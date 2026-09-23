export abstract class Strings {
	static readonly APP_NAME: string = '魚之魷魂 SquidSpirit';

	static readonly PRIVACY_POLICY: string = '隱私權政策';
	static readonly AI_USAGE_POLICY: string = 'AI 輔助創作使用原則聲明';

	static readonly CANCEL: string = '取消';
	static readonly CLEAR: string = '清除';
	static readonly CLOSE_MENU: string = '關閉選單';
	static readonly CONFIRM: string = '確認';
	static readonly EMAIL: string = 'Email';
	static readonly EMPTY_POSTS: string = '查無文章';
	static readonly GIT_REPOSITORY: string = 'Git Repository';
	static readonly KEYWORD: string = '關鍵字';
	static readonly LABEL: string = '標籤';
	static readonly LABEL_SELECT_PLACEHOLDER: string = '選擇一個標籤來篩選文章';
	static readonly LOOK_FOR_POSTS_WITH_SAME_LABEL: string = '查看相同標籤的文章';
	static readonly NOT_FOUND_CODE: string = '404';
	static readonly OPEN_MENU: string = '開啟選單';
	static readonly POST: string = '文章';
	static readonly RSS_FEED: string = 'RSS Feed';
	static readonly SEARCH: string = '搜尋';
	static readonly SEARCH_AND_FILTER_TITLE: string = '搜尋與篩選';
	static readonly SEARCH_LABEL_PLACEHOLDER: string = '搜尋標籤';
	static readonly SEARCH_PLACEHOLDER: string = '搜尋文章';
	static readonly SEARCH_POST_PLACEHOLDER: string = '搜尋文章內容或直接提問';
	static readonly SEARCH_POST_HINT: string =
		'輸入文章內容片段，或直接提出具體問題，系統將會以 AI（向量比對）搜尋相關的文章內容，搜尋結果根據相關性排序。';
	static readonly SEARCH_UNAVAILABLE: string = '搜尋暫時無法使用，請稍後再試';
	static readonly SELF_INTRODUCTION_LINES: readonly string[] = [
		'大家好，我是 Squid 魷魚',
		'身為一位軟體工程師',
		'平常最喜歡埋首於程式碼的世界',
		'鑽研各種新奇有趣的技術',
		'在這裡',
		'我會分享我的技術筆記、開發心得',
		'還有各式各樣實用工具的評測與介紹',
		'一起探索數位世界的無限可能吧！',
	];
	static readonly SHARE: string = 'Share';
	static readonly SHARE_TO_EMAIL: string = 'Email';
	static readonly SHARE_TO_FACEBOOK: string = 'Facebook';
	static readonly SHARE_TO_LINKEDIN: string = 'LinkedIn';
	static readonly SHARE_TO_X: string = 'X';
	static readonly SITE_DESCRIPTION: string = Strings.SELF_INTRODUCTION_LINES.join('，');
	static readonly TOC: string = '章節目錄';
	static readonly YOUTUBE_CHANNEL: string = 'YouTube Channel';
}

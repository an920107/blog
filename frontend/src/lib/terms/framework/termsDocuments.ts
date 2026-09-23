import { Strings } from '$lib/strings';

export interface TermsDocument {
	readonly pathname: string;
	readonly title: string;
}

export const TERMS_DOCUMENTS: readonly TermsDocument[] = [
	{ pathname: 'privacy-policy', title: Strings.PRIVACY_POLICY },
	{ pathname: 'ai-usage-policy', title: Strings.AI_USAGE_POLICY },
];

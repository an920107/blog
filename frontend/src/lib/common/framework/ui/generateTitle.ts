import { Strings } from '$lib/strings';

export default function generateTitle(pageTitle?: string): string {
	if (!pageTitle) return Strings.APP_NAME;
	return `${Strings.APP_NAME} - ${pageTitle}`;
}

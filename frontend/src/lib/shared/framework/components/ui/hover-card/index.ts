import { LinkPreview as HoverCardPrimitive } from 'bits-ui';

import Content from '$lib/shared/framework/components/ui/hover-card/hover-card-content.svelte';
import Trigger from '$lib/shared/framework/components/ui/hover-card/hover-card-trigger.svelte';

const Root = HoverCardPrimitive.Root;

export {
	Content,
	Root as HoverCard,
	Content as HoverCardContent,
	Trigger as HoverCardTrigger,
	Root,
	Trigger,
};

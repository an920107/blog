import { Popover as PopoverPrimitive } from 'bits-ui';

import Content from '$lib/common/framework/components/ui/popover/popover-content.svelte';
import Trigger from '$lib/common/framework/components/ui/popover/popover-trigger.svelte';

const Root = PopoverPrimitive.Root;
const Close = PopoverPrimitive.Close;

export {
	Close,
	Content,
	Root as Popover,
	Close as PopoverClose,
	Content as PopoverContent,
	Trigger as PopoverTrigger,
	Root,
	Trigger,
};

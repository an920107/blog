import { Dialog as DialogPrimitive } from 'bits-ui';

import Close from '$lib/common/framework/components/ui/dialog/dialog-close.svelte';
import Content from '$lib/common/framework/components/ui/dialog/dialog-content.svelte';
import Description from '$lib/common/framework/components/ui/dialog/dialog-description.svelte';
import Footer from '$lib/common/framework/components/ui/dialog/dialog-footer.svelte';
import Header from '$lib/common/framework/components/ui/dialog/dialog-header.svelte';
import Overlay from '$lib/common/framework/components/ui/dialog/dialog-overlay.svelte';
import Title from '$lib/common/framework/components/ui/dialog/dialog-title.svelte';
import Trigger from '$lib/common/framework/components/ui/dialog/dialog-trigger.svelte';

const Root = DialogPrimitive.Root;
const Portal = DialogPrimitive.Portal;

export {
	Close,
	Content,
	Description,
	Root as Dialog,
	Close as DialogClose,
	Content as DialogContent,
	Description as DialogDescription,
	Footer as DialogFooter,
	Header as DialogHeader,
	Overlay as DialogOverlay,
	Portal as DialogPortal,
	Title as DialogTitle,
	Trigger as DialogTrigger,
	Footer,
	Header,
	Overlay,
	Portal,
	Root,
	Title,
	Trigger,
};

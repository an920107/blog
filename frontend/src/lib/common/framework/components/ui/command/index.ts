import { Command as CommandPrimitive } from 'bits-ui';

import Root from '$lib/common/framework/components/ui/command/command.svelte';
import Dialog from '$lib/common/framework/components/ui/command/command-dialog.svelte';
import Empty from '$lib/common/framework/components/ui/command/command-empty.svelte';
import Group from '$lib/common/framework/components/ui/command/command-group.svelte';
import Input from '$lib/common/framework/components/ui/command/command-input.svelte';
import Item from '$lib/common/framework/components/ui/command/command-item.svelte';
import LinkItem from '$lib/common/framework/components/ui/command/command-link-item.svelte';
import List from '$lib/common/framework/components/ui/command/command-list.svelte';
import Separator from '$lib/common/framework/components/ui/command/command-separator.svelte';
import Shortcut from '$lib/common/framework/components/ui/command/command-shortcut.svelte';

const Loading = CommandPrimitive.Loading;

export {
	Root as Command,
	Dialog as CommandDialog,
	Empty as CommandEmpty,
	Group as CommandGroup,
	Input as CommandInput,
	Item as CommandItem,
	LinkItem as CommandLinkItem,
	List as CommandList,
	Loading as CommandLoading,
	Separator as CommandSeparator,
	Shortcut as CommandShortcut,
	Dialog,
	Empty,
	Group,
	Input,
	Item,
	LinkItem,
	List,
	Loading,
	Root,
	Separator,
	Shortcut,
};

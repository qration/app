<script>
	import wordmark from '$lib/assets/qration-wordmark.svg';

  import IconButton from "../ui/IconButton.svelte";
	import SidebarButton from "../ui/SidebarButton.svelte";
</script>

<div class="border-r-2 font-medium border-r-gray-300 p-4 flex-1 flex flex-col justify-between">
  <div class="flex flex-col gap-4">
    <div class="flex flex-col">
      <div class="flex flex-row justify-between items-center">
        <img src={wordmark} alt="Qration logo" class="h-10 cursor-pointer" />
        <IconButton iconClass="ti ti-layout-sidebar-left-collapse" label="Collapse" />
      </div>
    </div>
    <SidebarButton text="Add New" iconClass="ti ti-plus " />
    <div class="flex flex-col">
      <SidebarButton text="All Feeds" iconClass="ti ti-news " />
      <SidebarButton text="Unread" iconClass="ti ti-notification " />
      <SidebarButton text="Favourites" iconClass="ti ti-star " />
      <SidebarButton text="Saved" iconClass="ti ti-bookmark " />
    </div>
    Subscriptions
  </div>

  <div class="flex flex-col">
    <SidebarButton text="Settings" iconClass="ti ti-settings " />
  </div>
</div>
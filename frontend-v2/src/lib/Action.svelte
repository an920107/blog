<script lang="ts">
  let { label, link, isSelected } = $props();
</script>

<div class:rounded={isSelected} class:px-1.5={isSelected} class:bg-blue-600={isSelected} class:bg-transparent={!isSelected}>
  <a class:font-extrabold={true} class:text-white={isSelected} class:text-gray-800={!isSelected} href={link}>
    {label}
  </a>
</div>

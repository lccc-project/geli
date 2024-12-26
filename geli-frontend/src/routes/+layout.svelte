<script lang="ts">
  import "../app.css";
  let { children } = $props();
</script>

<div class="h-screen bg-amber-400 dark:bg-amber-800">
  {@render children()}
</div>

<script lang="ts">
  import logo from "$assets/images/icon.webp";
  import { Spinner } from "flowbite-svelte";
</script>

<div class="h-screen flex flex-col items-center justify-center bg-black">
  <img data-tauri-drag-region src={logo} alt="" srcset="" class="w-48" />
  <Spinner color="yellow" size={"12"} class="mt-5" />
</div>

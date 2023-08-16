<script lang="ts">
  import {onMount} from "svelte";
  import { appWindow } from '@tauri-apps/api/window'
  import {invoke} from "@tauri-apps/api";

  interface TauriEvent {
    payload: {
      message: string;
      done: boolean;
    }
  }

  let promptMessage = ''
  let msg = ''
  let loading = false
  let error: unknown;

  async function onGreetHandler() {
    msg = '';

    loading = true;

    await invoke('greet',  {promptMessage})
  }
  
  function onIncomingMessageHandler(event: TauriEvent) {
    loading = false

    msg += event.payload.message
  }

  onMount(() => {
    document.addEventListener('keypress', ({key}) => {
      if (key === 'Enter' && promptMessage) {
        onGreetHandler()
      }
    })

    appWindow.listen('generate-answer-listener', onIncomingMessageHandler)
  })
</script>

<div class="mx-3">
  <div class="mt-1 text-2xl text-center w-screen">Interactive layer for Ollama</div>
  <label for="request" class="text-md/7 block my-2 font-medium text-gray-900 dark:text-white">Your Request</label>
  <input id="request"
         class="mb-6 block p-2.5 w-full text-sm text-gray-900 bg-gray-50 rounded-md border border-gray-300 focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600 dark:placeholder-gray-400 dark:text-white dark:focus:ring-blue-500 dark:focus:border-blue-500"
         bind:value={promptMessage}/>

  <button disabled={!promptMessage} on:click="{onGreetHandler}" type="button"
          class="text-white bg-gray-800 hover:bg-gray-900 focus:outline-none focus:ring-4 focus:ring-gray-300 font-medium rounded-md text-sm px-5 py-2.5 mr-2 mb-2 dark:bg-gray-800 dark:hover:bg-gray-700 dark:focus:ring-gray-700 dark:border-gray-700">
    Get answer
  </button>
  {#if loading}
    <p>Loading...</p>
  {/if}
  {#if !!error}
    <p>{error}</p>
  {/if}
  <p class="text-gray-600 text-md/loose">{msg}</p>
</div>
<script>
  import { onMount } from 'svelte';
  import { isAuthenticated, checkAuth } from '../lib/stores/auth.js';
  import Login from '../lib/components/Login.svelte';
  import MainApp from '../lib/components/MainApp.svelte';
  
  // When the page loads, check if someone was already logged in
  onMount(() => {
      checkAuth();
  });
</script>

<svelte:head>
  <title>WhereIsIt App</title>
  <!-- Add Font Awesome for icons -->
  <link rel="stylesheet" href="https://cdnjs.cloudflare.com/ajax/libs/font-awesome/6.5.1/css/all.min.css" />
</svelte:head>

<!-- The magic happens here: show different components based on login status -->
{#if $isAuthenticated}
  <MainApp />
{:else}
  <Login />
{/if}
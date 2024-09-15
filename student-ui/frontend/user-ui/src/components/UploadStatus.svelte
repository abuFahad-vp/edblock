<script>
  export let showModal = false;
  export let isPromo = false;
  export let message = '';
</script>

{#if showModal}
  <div class="backdrop" class:promo={isPromo} on:click|self on:keydown>
    <div class="modal">
      <p>{message}</p>
    </div>
  </div>
{/if}

<style>
  .backdrop{
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: rgba(0,0,0,0.8);
  }
  .modal{
    padding: 10px;
    border-radius: 10px;
    max-width: 400px;
    margin: 20% auto;
    text-align: center;
    background: white;
  }
  .promo .modal{
    background: crimson;
    color: white;
  }
</style>

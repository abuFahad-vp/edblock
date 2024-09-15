<script>
    let address = '';
    const invoke = window.__TAURI__.invoke
    invoke('get_address')
        .then((value) => address = value);
    let isCopied = false;

    function copyAddress() {
        navigator.clipboard.writeText(address)
            .then(() => {
                isCopied = true;
                setTimeout(() => {
                    isCopied = false;
                }, 2000); // Reset after 2 seconds
            })
            .catch(err => {
                console.error('Failed to copy: ', err);
            });
    }
</script>

<main>
    <h4 class="address" on:click={copyAddress} on:keypress>
        Your Address: {address}
        {#if isCopied}
            <span class="copied-msg"> (Copied!)</span>
        {/if}
    </h4>
</main>

<style>
    .address {
        cursor: pointer;
        color: #007bff;
        transition: color 0.3s ease;
        font-size: 1.2rem;
        display: inline-block;
        position: relative;
        margin-left: 25%;
    }

    .address:hover {
        color: #0056b3;
    }

    .copied-msg {
        color: green;
        font-size: 1rem;
        margin-left: 10px;
        animation: fade 2s ease-in-out;
    }

    @keyframes fade {
        0% { opacity: 1; }
        100% { opacity: 0; }
    }
</style>

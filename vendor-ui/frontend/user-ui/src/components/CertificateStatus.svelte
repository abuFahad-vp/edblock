<script>
    let certificate_id = '';
    let student_address = '';
    let university_address = '';
    let isAuthentic = null;
    let isLoading = false;

    export let url = "";
    const invoke = window.__TAURI__.invoke

    function verifyCertificate() {
        isLoading = true;

        invoke('get_status', {url, certificate_id: certificate_id.trim(), student_address: student_address.trim(), university_address: university_address.trim()})
            .then((value) => {
                isAuthentic = true
                isLoading = false;
            })
            .catch((e) => {
                isAuthentic = false
                isLoading = false
            }
        )
    }
</script>

<main>
    <form on:submit|preventDefault={verifyCertificate} class="form">
        <div class="form__group">
            <label for="certificate_id">Certificate ID:</label>
            <input id="certificate_id" type="text" bind:value={certificate_id} placeholder="Enter Certificate ID" required>
        </div>
        <div class="form__group">
            <label for="student_wallet_address">Student Wallet Address:</label>
            <input id="student_wallet_address" type="text" bind:value={student_address} placeholder="Enter Student Wallet Address" required>
        </div>
        <div class="form__group">
            <label for="university_address">University Address:</label>
            <input id="university_address" type="text" bind:value={university_address} placeholder="Enter University Address" required>
        </div>
        <button type="submit" class="verify-button">Verify</button>
    </form>

    {#if isLoading}
        <div class="loading-screen">
            <p>Verifying certificate... Please wait.</p>
        </div>
    {/if}

    {#if isAuthentic !== null && !isLoading}
        <div class="result">
            {#if isAuthentic}
                <p class="success">This certificate is authentic!</p>
            {:else}
                <p class="error">Failed to Verify.</p>
            {/if}
        </div>
    {/if}
</main>

<style>
    .form {
        max-width: 400px;
        margin: 0 auto;
        padding: 20px;
        background-color: #f4f4f4;
        border-radius: 8px;
        box-shadow: 0 2px 5px rgba(0, 0, 0, 0.1);
    }

    .form__group {
        margin-bottom: 15px;
    }

    .form__group label {
        display: block;
        margin-bottom: 5px;
        font-weight: bold;
    }

    .form__group input {
        width: 100%;
        padding: 10px;
        border: 1px solid #ccc;
        border-radius: 4px;
    }

    .verify-button {
        width: 100%;
        padding: 10px;
        background-color: gray;
        color: white;
        border: none;
        border-radius: 4px;
        cursor: pointer;
        font-size: 16px;
        font-weight: bold;
    }

    .verify-button:hover {
        background-color: #11998e;
    }

    .loading-screen {
        text-align: center;
        margin-top: 20px;
    }

    .result {
        text-align: center;
        margin-top: 20px;
        font-size: 18px;
    }

    .success {
        color: green;
    }

    .error {
        color: red;
    }
</style>
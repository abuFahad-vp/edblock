<script>
    import CertDetails from "./CertDetails.svelte";

    export let certificates = [];
</script>

<div class="cert-list">
    {#each certificates as certificate (certificate.id)}
        <div>
            <CertDetails {certificate}/>
        </div>
    {/each}
</div>

<style>
    .cert-list{
        display: grid;
        grid-template-columns: 1fr 1fr;
        gap: 20px;
    }
</style>
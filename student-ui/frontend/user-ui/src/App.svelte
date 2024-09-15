<script>
	import Header from './components/Header.svelte';
	import Footer from './components/Footer.svelte';
  	import Tabs from './shared/Tabs.svelte';
    import CreateUploadForm from './components/CreateUploadForm.svelte';
	import CertificateStatus from './components/CertificateStatus.svelte';
    import UrlBox from './components/UrlBox.svelte';
	import ShowAddress from './components/ShowAddress.svelte';
    import { onMount } from 'svelte';
    const invoke = window.__TAURI__.invoke
  
 	// tabs
	let items = ["Status", 'Upload Certificate'];
	let activeItem = 'Status';
	const tabChange = (e) => activeItem = e.detail;

	let certificates = [];
	let url = "http://127.0.0.1:12345";
	let trans_url = "http://127.0.0.1:3000";

    function get_certificates() {
		// console.log(url)
        invoke('get_certificates', {url})
        .then((value) =>  {
				let new_certificates = value.sort((a, b) => a - b);
				new_certificates.map((elem, i) => elem.id = i + 1);
				// console.log(new_certificates)
				certificates = new_certificates
            }
        )
        .catch((e) => console.log(e))
    }
	onMount(() => {
		get_certificates();
    	const intervalId = setInterval(get_certificates, 2000);
		return () => clearInterval(intervalId);
	});

</script>

<Header />
<main>
	<ShowAddress/>
	<Tabs {activeItem} {items} on:tabChange={tabChange} />
	{#if activeItem === 'Status'}
	<UrlBox label="Explorer Url: " {url} on:input={(e) => url = e.detail}/>
	<CertificateStatus {certificates}/>
	{:else if activeItem === 'Upload Certificate'}
	<UrlBox label="Node url: " url={trans_url} on:input={(e) => trans_url = e.detail}/>
	<CreateUploadForm {trans_url} />
	{/if}
</main>
<Footer />

<style>
	main{
    width: 100%;
    max-width: 960px;
    margin: 40px auto;
  }
</style>
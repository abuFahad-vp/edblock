<script>
	import Header from './components/Header.svelte';
	import Footer from './components/Footer.svelte';
	import CertificateStatus from './components/CertificateStatus.svelte';
    import UrlBox from './components/UrlBox.svelte';
    import { onMount } from 'svelte';
	import ShowAddress from './components/ShowAddress.svelte';
    const invoke = window.__TAURI__.invoke
  
	let certificates = [];
	let trans_url = "http://127.0.0.1:3000";
	let expl_url = "http://127.0.0.1:12345";

    function get_not_completed_certificates() {
		// console.log(url)
        invoke('get_not_completed_certificates', {url: expl_url})
        .then((value) =>  {
				let new_certificates = value.sort((a, b) => a - b);
				new_certificates.map((elem, i) => elem.id = i + 1);
				console.log(new_certificates)
				certificates = new_certificates
            }
        )
        .catch((e) => console.log(e))
    }
	onMount(() => {
		get_not_completed_certificates();
    	const intervalId = setInterval(get_not_completed_certificates, 2000);
		return () => clearInterval(intervalId);
	});

</script>

<Header />
<main>
	<UrlBox label="Node Url: " url={trans_url} on:input={(e) => trans_url = e.detail}/>
	<UrlBox label="Explorer Url: " url={expl_url} on:input={(e) => expl_url = e.detail}/>
  	<ShowAddress/>
	<CertificateStatus {trans_url} {certificates}/>
</main>
<Footer />

<style>
	main{
    width: 100%;
    max-width: 960px;
    margin: 40px auto;
  }
</style>
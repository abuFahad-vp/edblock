<script>
	import Header from './components/Header.svelte';
	import Footer from './components/Footer.svelte';
  import UrlBox from './components/UrlBox.svelte';
  import CertificateStatus from './components/CertificateStatus.svelte';

	let url = "http://127.0.0.1:12345";

</script>

<Header />
<main>
  <UrlBox label="Explorer Url: " {url} on:input={(e) => url = e.detail}/>
  <CertificateStatus {url}/>
</main>
<Footer />

<style>
	main{
    width: 100%;
    max-width: 960px;
    margin: 40px auto;
  }
</style>
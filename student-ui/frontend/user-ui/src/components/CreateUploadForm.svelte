<script>
  import Button from '../shared/Button.svelte';
  import UploadStatus from './UploadStatus.svelte';
  const invoke = window.__TAURI__.invoke

  let fields = {course_id: '', course_name: '', uni_addr: ''};
  let errors = {course_id: '', course_name: '', uni_addr: ''};
  let valid = false;
  let uploadStatus = "Successfull";
  let showModal = false;
  let showUploadButton = true;
  export let trans_url;

  const toggleStatus = () => {
    showModal = !showModal
  }

  const submitHandler = () => {
    valid = true;

    //course id 
    if (fields.course_id.trim().length < 1) {
        valid = false
        errors.course_id = 'Course Id cannot be empty'
    }else {
        errors.course_id = ''
    }

    //course name
    if (fields.course_name.trim().length < 1) {
        valid = false
        errors.course_name = 'Course name cannot be empty'
    }else {
        errors.course_name = ''
    }

    //uni address
    if (fields.uni_addr.trim().length < 1) {
        valid = false
        errors.uni_addr = 'University Address cannot be empty'
    } else {
        errors.uni_addr = ''
    }

    if (valid) {
      showUploadButton = false;
      console.log("trans url: ", trans_url)
      invoke('upload_certificate', {trans_url, course_id: fields.course_id, course_name: fields.course_name, uni_addr: fields.uni_addr})
        .then((_) => {
          uploadStatus = "Submission Successfull"
          toggleStatus();
          showUploadButton = true
        })
        .catch((_) => {
          uploadStatus = "Submission Failed"
          toggleStatus();
          showUploadButton = true
        })
      fields.course_id = '';
      fields.course_name = '';
      fields.uni_addr = '';
    }
  }
</script>

<UploadStatus message={uploadStatus} {showModal} on:click={toggleStatus}/>
<form on:submit|preventDefault={submitHandler}>
    <div class="form-field">
      <label for="course_id">Course Id: </label>
      <input type="text" id="course_id" bind:value={fields.course_id}>
      <div class="error">{ errors.course_id }</div>
    </div>
    <div class="form-field">
      <label for="course_name">Course Name: </label>
      <input type="text" id="course_name" bind:value={fields.course_name}>
      <div class="error">{ errors.course_name }</div>
    </div>
    <div class="form-field">
      <label for="uni_addr">University Address: </label>
      <input type="text" id="uni_addr" bind:value={fields.uni_addr}>
      <div class="error">{ errors.uni_addr }</div>
    </div>
    {#if showUploadButton}
      <Button>Upload</Button>
    {:else}
      <p>Uploading...</p>
    {/if}
</form>

<style>
    form{
      width: 400px;
      margin: 0 auto;
      text-align: center;
    }
    .form-field{
      margin: 18px auto;
    }
    input{
      width: 100%;
      border-radius: 6px;
    }
    label{
      margin: 10px auto;
      text-align: left;
    }
    .error{
      font-weight: bold;
      font-size: 12px;
      color: #d91b42;
    }
</style>
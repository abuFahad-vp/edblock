<script>
    export let url = ''; // URL input value
    import {createEventDispatcher} from 'svelte';

    let dispatch = createEventDispatcher();
    export let label = "Url: "

    const handleInput = (e) => {
        url = e.target.value
        dispatch('input', url)
    }

</script>

<div class="input-container">
    <label for="url" class="form__label"> {label} </label>
    <input id="url" type="text" on:input={handleInput} bind:value={url} placeholder="Enter explorer URL here">
</div>

<style>
.input-container {
    display: flex;
    justify-content: center;
    align-items: center;
    position: relative; /* For positioning the hint */
    margin: 30px;
}

input {
    padding: 10px;
    border: 1px solid #ccc;
    border-radius: 50px;
    width: 200px; /* Adjust the width as needed */
    height: 30px;
    font-size: 16px; /* Larger font size for better readability */
}

.form__label {
    margin-bottom: 10px;
    padding: 10px;
    font-size: 16px;
}
</style>
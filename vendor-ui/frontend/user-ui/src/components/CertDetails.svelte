<script>
    export let certificate;
</script>

<div class="certificate">
    <h4><span class="label">TimeStamp:</span> <span class="value">{certificate.timestamp}</span></h4>
    <h4><span class="label">Certificate ID:</span> <span class="value">{certificate.certificate_id}</span></h4>
    <h4><span class="label">Course ID:</span> <span class="value">{certificate.course_id}</span></h4>
    <h4><span class="label">Course Name:</span> <span class="value">{certificate.course_name}</span></h4>
    <h4><span class="label">Institution Address:</span> <span class="value">{certificate.uni_wallet_addr}</span></h4>
    <h4>
        <span class="label">Status:</span> 
        <span class="status-dot {certificate.status === 'completed' ? 'green' : (certificate.status ? '' : 'red')}"></span> 
        <span class="value">{certificate.status || 'Pending'}</span>
    </h4>
</div>

<style>
    .certificate {
        border: 1px solid #ccc;
        padding: 20px;
        border-radius: 10px;
        width: 500px;
        background-color: #f9f9f9;
        font-family: Arial, sans-serif;
        box-shadow: 0 4px 8px rgba(0, 0, 0, 0.1);
        margin-left: 21%;
    }
    
    .certificate h4 {
        margin: 10px 0;
        font-weight: normal;
        color: #333;
    }

    .label {
        font-weight: bold;
        color: #555;
    }

    .value {
        color: #000;
        font-style: italic;
    }

    .status-dot {
        height: 10px;
        width: 10px;
        border-radius: 50%;
        display: inline-block;
        margin-right: 10px;
    }

    .status-dot.green {
        background-color: green;
    }

    .status-dot.red {
        background-color: red;
    }
</style>

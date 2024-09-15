const STYLE: &str = r#"
        body {
            font-family: Arial, sans-serif;
            margin: 20px;
        }
        label {
            display: block;
            margin: 10px 0 5px;
        }
        input, textarea {
            width: 100%;
            padding: 10px;
            margin: 5px 0;
            border-radius: 5px;
            border: 1px solid #ccc;
        }
        button {
            margin-top: 20px;
            padding: 10px 20px;
            background-color: #4CAF50;
            color: white;
            border: none;
            border-radius: 5px;
            cursor: pointer;
        }
        button:hover {
            background-color: #45a049;
        }
"#;

pub fn certificate_template() -> String {
    format!(r#"
<!DOCTYPE html>
<html lang="en">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>Certificate Submission</title>
    <style>
    {}
    </style>
</head>
<body>

    <h1>Submit Certificate Information</h1>

    <form action="/submit_certificate" method="post">

        <!-- Course Details -->
        <h3>Course Information</h3>
        
        <label for="course_id">Course ID:</label>
        <input type="text" id="course_id" name="course_id" required>
        
        <label for="course_name">Course Name:</label>
        <input type="text" id="course_name" name="course_name" required>

        <!-- University Details -->
        <h3>University Information</h3>

        <label for="wallet_address_university">University Wallet Address:</label>
        <input type="text" id="wallet_address_university" name="wallet_address_university" required>
        
        <!-- Submit Button -->
        <button type="submit">Submit Certificate</button>
    </form>
</body>
</html>
"#, STYLE)
}
use crate::blockchain::wallet::KeyPairAddress;
use urlencoding;

pub fn key_pair_template(keys: KeyPairAddress) -> String {
  format!(
    r#"
<!DOCTYPE html>
<html>
<head>
    <style>
        .key-box {{
            width: 100%;
            height: 100px;
            padding: 10px;
            border: 1px solid #ccc;
            border-radius: 5px;
            font-family: monospace;
            white-space: pre-wrap;
            word-wrap: break-word;
        }}
        .button {{
            margin-top: 10px;
            padding: 5px 15px;
            background-color: #4CAF50;
            color: white;
            border: none;
            cursor: pointer;
            border-radius: 5px;
        }}
        .button:active {{
            background-color: #45a049;
        }}
    </style>
</head>
<body>
    <h3>Your Keys:</h3>

    <h4>Private Key</h4>
    <textarea id="private_key" class="key-box" readonly>{priv_key_pem}</textarea>
    <br>
    <button class="button" onclick="copyToClipboard('private_key')">Copy Private Key</button>
    <a class="button" href="data:text/plain;charset=utf-8,{encoded_priv_key}" download="private_key.pem">Download Private Key</a>

    <h4>Public Key</h4>
    <textarea id="public_key" class="key-box" readonly>{pub_key_pem}</textarea>
    <br>
    <button class="button" onclick="copyToClipboard('public_key')">Copy Public Key</button>
    <a class="button" href="data:text/plain;charset=utf-8,{encoded_pub_key}" download="public_key.pem">Download Public Key</a>

    <h4>Wallet Address</h4>
    <textarea id="wallet_address" class="key-box" readonly>{wallet_address}</textarea>
    <br>
    <button class="button" onclick="copyToClipboard('wallet_address')">Copy Wallet Address</button>

    <script>
        function copyToClipboard(elementId) {{
            var copyText = document.getElementById(elementId);
            copyText.select();
            document.execCommand("copy");
            alert(elementId.replace('_', ' ') + " copied to clipboard!");
        }}
    </script>
</body>
</html>
"#,
    priv_key_pem = keys.priv_key_pem,
    pub_key_pem = keys.pub_key_pem,
    wallet_address = keys.wallet_address,
    encoded_priv_key = urlencoding::encode(&keys.priv_key_pem).into_owned(),
    encoded_pub_key = urlencoding::encode(&keys.pub_key_pem).into_owned()
)
}
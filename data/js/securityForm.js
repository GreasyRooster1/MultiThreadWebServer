let crypt = new Crypt();
let rsa = new RSA();
let publicKey;
let privateKey;
let serverPublicKey;
let doTelemetry = false;

function testHttpGet(){
    fetch("/index.html").then(function(response) {
        return response.text();
    }).then(function(data) {
        console.log(data);
    }).catch(function(err) {
        console.log('Fetch Error :-S', err);
    });
}
function testGetServerPublicKey(){
    fetch("/public_key").then(function(response) {
        return response.text();
    }).then(function(data) {
        serverPublicKey = data.replace("-----END PUBLIC KEY-----","");
    }).catch(function(err) {
        console.log('Fetch Error :-S', err);
    });
}
function generateKeys() {
    var rsa = new RSA();
    rsa.generateKeyPair(function(keyPair) {
        publicKey = keyPair.publicKey;
        privateKey = keyPair.privateKey;
    });
    console.log('publicKey', publicKey);
    console.log('privateKey', privateKey);
}

function encrypt(data) {
    var entropy = Math.random()*19283749128;
    var crypt = new Crypt({
        rsaStandard: 'RSA-OAEP',
        entropy: entropy
    });
    var rsa = new RSA({
        entropy: entropy
    });
    return crypt.encrypt(serverPublicKey, data);
}

var requestSecureData = function(){
    let filename = $("#filename-input").val();
    let transferKey = $("#transfer-key-input").val();
    let decodeKey = $("#decode-key-input").val();//not important
    let passwordHash = $("#password-input").val();

    window.location.replace("/raw_data_request/"+passwordHash+"/"+filename);
}

generateKeys();
$("#submit-button").click(requestSecureData);
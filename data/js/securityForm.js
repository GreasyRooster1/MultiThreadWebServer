let crypt = new Crypt();
let rsa = new RSA();
let publicKey;
let privateKey;
let serverPublicKey;
let doTelemetry = false;

function logTelemetry(str){
    $("#telemetry-console").append("<p>"+new Date().toString()+" - "+str+"</p>");
}
function getServerPublicKey(){
    fetch("/public_key").then(function(response) {
        return response.text();
    }).then(function(data) {
        serverPublicKey = data.replace("-----END PUBLIC KEY-----","");
    }).catch(function(err) {
        console.log('Fetch Error :-S', err);
    });
}
function preformHttpRequest(url){
    let data = null;
    fetch(url).then(function(response) {
        return response.text();
    }).then(function(d) {
        data=d;
    }).catch(function(err) {
        console.log('Fetch Error :-S', err);
    });
    return data;
}
function generateKeys() {
    var rsa = new RSA();
    rsa.generateKeyPair(function(keyPair) {
        publicKey = keyPair.publicKey;
        privateKey = keyPair.privateKey;
    });
    logTelemetry("PubKey: "+publicKey);
    logTelemetry("PrivKey: "+privateKey);
    logTelemetry("Generated session encryption keys!");
}
function encrypt(data) {
    let entropy ="wow its encryption";
    let crypt = new Crypt({
        rsaStandard: 'RSA-OAEP',
        entropy: entropy
    });
    let rsa = new RSA({
        entropy: entropy
    });
    return crypt.encrypt({
        name: "RSA-OAEP",
    },serverPublicKey, data);
}
function decrypt(data) {
    var entropy = "wow its encryption";
    var crypt = new Crypt({
        rsaStandard: 'RSA-OAEP',
        entropy: entropy
    });
    var rsa = new RSA({
        entropy: entropy
    });
    return crypt.decrypt({
        name: "RSA-OAEP",
    },privateKey, data);
}

var requestSecureData = function(){
    let filename = $("#filename-input").val();
    let decodeKey = $("#decode-key-input").val();//not important
    let password = $("#password-input").val();

    let encryptedRequest = encrypt(password+"."+filename+"."+publicKey);

    //let response = preformHttpRequest("/raw_data_request/"+encryptedRequest);
    //logTelemetry("raw_data_request response: "+response);
}


$("#submit-button").click(requestSecureData);
$("#telemetry-console").hide();
$("#telemetry-checkbox").click(function() {
    $("#telemetry-console").toggle(this.checked);
});
getServerPublicKey()
generateKeys();
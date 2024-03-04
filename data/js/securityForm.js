$("#submit-button").click(function(){
    let filename = $("#filename-input").val();
    let transferKey = $("#transfer-key-input").val();
    let decodeKey = $("#decode-key-input").val();
    let passwordHash = $("#password-input").val();

    window.location.replace("/raw_data_request/"+passwordHash+"/"+filename);
});
function load_data() {
    fetch("./data.json")
    .then(response => response.json())
    .then(json => console.log(json));

    };
    // document.getElementById("put_json").innerHTML = ;

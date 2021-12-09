const town_data_url = "https://byui-cit230.github.io/weather/data/towndata.json";



function get_town_data() {
    fetch(town_data_url)
    .then(function (response) {
    return response.json();
    })
    .then(function (jsonObject) {
    //temporary checking for valid response and data parsing
    // console.table(jsonObject);
    if (document.querySelector(".page_title").textContent == "Preston") {
        populate_preston(jsonObject);
    }
    else if (document.querySelector(".page_title").textContent == "Fish Haven") {
        populate_fish_haven(jsonObject);
    }
    else if (document.querySelector(".page_title").textContent == "Soda Springs") {
    populate_soda_springs(jsonObject);
    }})
}

function populate_preston(town_data) {
    let events_span = document.getElementById("events_span");
    let events = town_data.towns[6].events;
    // console.log(events)
    for (let i = 0; i < events.length; i++) {
        // console.log(events[i]);
        events_span.innerHTML += events[i] + "<br>";
    }
}

function populate_fish_haven(town_data) {
    let events_span = document.getElementById("events_span");
    let events = town_data.towns[2].events;
    // console.log(events)
    for (let i = 0; i < events.length; i++) {
        // console.log(events[i]);
        events_span.innerHTML += events[i] + "<br>";
    }
}

function populate_soda_springs(town_data) {
    let events_span = document.getElementById("events_span");
    let events = town_data.towns[0].events;
    // console.log(events)
    for (let i = 0; i < events.length; i++) {
        // console.log(events[i]);
        events_span.innerHTML += events[i] + "<br>";
    }
}

get_town_data();
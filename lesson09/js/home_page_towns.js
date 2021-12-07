const town_data_url = "https://byui-cit230.github.io/weather/data/towndata.json";



function get_town_data() {
    fetch(town_data_url)
    .then(function (response) {
    return response.json();
    })
    .then(function (jsonObject) {
    //temporary checking for valid response and data parsing
    console.table(jsonObject);
    populate_preston(jsonObject);
    populate_fish_haven(jsonObject);
    populate_soda_springs(jsonObject);
    })
}

function populate_preston(town_data) {
    let subheading = document.getElementById('preston_subheading')
    let output_span =  document.getElementById('preston_span')
    subheading.textContent = town_data["towns"][6]["motto"]
    output_span.innerHTML = "Year Founded: " + town_data["towns"][6]["yearFounded"] + "<br>"
    + "Population: " + town_data["towns"][6]["currentPopulation"] + "<br>"
    + "Annual Rain Fall: " + town_data["towns"][6]["averageRainfall"]
}

function populate_fish_haven(town_data) {
    let subheading = document.getElementById('fish_haven_subheading')
    let output_span =  document.getElementById('fish_haven_span')
    subheading.textContent = town_data["towns"][2]["motto"]
    output_span.innerHTML = "Year Founded: " + town_data["towns"][2]["yearFounded"] + "<br>"
    + "Population: " + town_data["towns"][2]["currentPopulation"] + "<br>"
    + "Annual Rain Fall: " + town_data["towns"][2]["averageRainfall"]
}

function populate_soda_springs(town_data) {
    let subheading = document.getElementById('soda_springs_subheading')
    let output_span =  document.getElementById('soda_springs_span')
    subheading.textContent = town_data["towns"][0]["motto"]
    output_span.innerHTML = "Year Founded: " + town_data["towns"][0]["yearFounded"] + "<br>"
    + "Population: " + town_data["towns"][0]["currentPopulation"] + "<br>"
    + "Annual Rain Fall: " + town_data["towns"][0]["averageRainfall"]
}
get_town_data();
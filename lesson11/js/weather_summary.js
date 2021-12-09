// temp is measured in farenheit and wind_speed is measured in mph
function calc_wind_chill(temp, wind_speed) {
    let wind_chill = 0;
    if (temp <= 50 && wind_speed >= 3) {
        wind_chill = 35.74+(0.6215*temp)-(35.75*(Math.pow(wind_speed, 0.16)))+(0.4275*temp*(Math.pow(wind_speed, 0.16)));
        wind_chill = (Math.round(wind_chill*10))/10
        // console.log(wind_chill);
        wind_chill = wind_chill + "°F"
    }
    else {
        wind_chill = "NA";
        // console.log(wind_chill);
    }
    // console.log(wind_chill);
    return wind_chill
}


// api call
// api.openweathermap.org/data/2.5/weather?id={city id}&appid={API key}

//     "id": 5607916,
//     "name": "Soda Springs",
//     "state": "ID",
//     "country": "US",

//     "id": 5604473,
//     "name": "Preston",
//     "state": "ID",
//     "country": "US",

//         "id": 5777544,
//         "name": "Logan",
//         "state": "UT",
//         "country": "US",

async function get_preston_weather() {
        let response = await fetch("https://api.openweathermap.org/data/2.5/forecast?id=5604473&appid=e3f28000665e53058b6110dbbe2f5365&units=imperial")
        let data = await response.json() 
        // console.log(data)   
        let five_days = process_weather_data(data.list);
        // console.log(five_days)
        display_weather(five_days);
    }
async function get_fish_haven_weather() {
    let response = await fetch("https://api.openweathermap.org/data/2.5/forecast?id=5777544&appid=e3f28000665e53058b6110dbbe2f5365&units=imperial")
    let data = await response.json() 
    // console.log(data)   
    let five_days = process_weather_data(data.list);
    // console.log(five_days)
    display_weather(five_days);
    }
async function get_soda_springs_weather() {
    let response = await fetch("https://api.openweathermap.org/data/2.5/forecast?id=5607916&appid=e3f28000665e53058b6110dbbe2f5365&units=imperial")
        let data = await response.json() 
        // console.log(data)   
        let five_days = process_weather_data(data.list);
        // console.log(five_days)
        display_weather(five_days);
    }
    
function process_weather_data(weather) {
        // each day object contains a high_temp and icon_id
        // five_days contains a list of the five current day objects
        let five_days = []
        let weekday_options = {
            weekday: "short",
          }; 
        const day_template = {
        name: "Sun",
        high_temp: "69",
        icon_id: "01d",
        wind_speed: "5",
        condition: "sunny",
        humidity: "50"
        };
        for (let i = 0; i < weather.length; i++) {
            if(weather[i].dt_txt.includes("18:00:00")) {
                let weekday = Object.create(day_template);
                weekday["name"] = new Date(weather[i].dt_txt).toLocaleDateString("en-US", weekday_options);
                weekday["high_temp"] = weather[i].main.temp_max;
                weekday["icon_id"] = weather[i].weather[0].icon;
                weekday["wind_speed"] = weather[i].wind.speed;
                weekday["condition"] = weather[i].weather[0].main;
                weekday["humidity"] = weather[i].main.humidity;
                
                five_days.push(weekday)
            }
        }
        return five_days;
    }

function display_weather(five_days) {
    let current_weather = five_days[0];
    let current_wind_chill = calc_wind_chill(current_weather.high_temp, current_weather.wind_speed); 
    const condition = document.querySelector(".weather_summary_td1");
    const high_temp = document.querySelector(".weather_summary_td2");
    const wind_chill = document.querySelector(".weather_summary_td3");
    const humidity = document.querySelector(".weather_summary_td4");
    const wind_speed = document.querySelector(".weather_summary_td5");
    condition.innerHTML = current_weather.condition;
    high_temp.innerHTML = current_weather.high_temp + "°F";
    wind_chill.innerHTML = current_wind_chill;
    humidity.innerHTML = current_weather.humidity + "%";
    wind_speed.innerHTML = current_weather.wind_speed + "mph";

    
    for (let i=0; i < 5; i++) {
        let icon_src = get_icon_src(five_days[i].icon_id);
        let n = i + 1;
        let tag_class = ".day" + n;
        // console.log(tag_class)
        let day_elements = document.querySelector(tag_class).children;
        day_elements[0].innerHTML = five_days[i].name;
        day_elements[1].innerHTML = five_days[i].high_temp + "°F";
        day_elements[2].setAttribute("src", icon_src);   
    }
}

function get_icon_src(icon_id) {
    let icon_src = ""
    if (icon_id.includes("01")) {
        icon_src = "assets/weather_icons/sunny.png";
    }
    else if (icon_id.includes("02")) {
        icon_src = "assets/weather_icons/partly_cloudy.png";
    }
    else if (icon_id.includes("03") || icon_id.includes("04")) {
        icon_src = "assets/weather_icons/cloudy.png";
    }
    else if (icon_id.includes("09") || icon_id.includes("10") || icon_id.includes("11")) {
        icon_src = "assets/weather_icons/rainy.png";
    }
    else if (icon_id.includes("13")) {
        icon_src = "assets/weather_icons/snowy.png";
    }
    else {
        icon_src = "assets/weather_icons/sunny.png";
    }
    // console.log(icon_src)
    return icon_src;
}


if (document.querySelector(".page_title").textContent == "Preston") {
    get_preston_weather();
}
else if (document.querySelector(".page_title").textContent == "Fish Haven") {
    get_fish_haven_weather();
}
else if (document.querySelector(".page_title").textContent == "Soda Springs") {
    get_soda_springs_weather();
}
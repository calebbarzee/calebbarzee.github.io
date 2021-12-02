const wind_chill_output = document.querySelector('.weather_summary_td3');

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
    wind_chill_output.innerHTML = wind_chill
}

calc_wind_chill(30, 5);

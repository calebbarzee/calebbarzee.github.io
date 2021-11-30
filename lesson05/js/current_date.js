function current_date() {
    const options = {
      weekday: "long",
      day: "numeric",
      month: "long",
      year: "numeric",
    };
    let current_date = new Date();
    current_date = current_date.toLocaleDateString('en-US', options);
    document.getElementById("current_date").innerHTML = current_date;
    // console.log(current_date);
  }
  // JavaScript Document

function copyrightDate() {
  let options1 = {
    year: "numeric",
  };
  let copyrightYear = new Date().toLocaleDateString(undefined, options1);
  document.getElementById("copyrightYear").textContent = copyrightYear;
}

function lastUpdatedDate() {
  let options2 = {
    month: "short",
    day: "numeric",
    year: "numeric",
  };
  let lastUpdated = new Date(document.lastModified);
  lastUpdated = lastUpdated.toLocaleDateString(undefined, options2);
  document.getElementById("updateDate").textContent = lastUpdated;
}


function banner_display() {
  const options = {
    weekday: "long",
  };
  let current_date = new Date();
  current_date = current_date.toLocaleDateString('en-US', options);
  // console.log(current_date);
  if (current_date == "Friday") {
    document.getElementById("pancake_banner").style.display = "block";
  }
  else {
    document.getElementById("pancake_banner").style.display = "none";
  }
  
}

current_date();
banner_display();
copyrightDate();
lastUpdatedDate();
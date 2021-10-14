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
    console.log(current_date);
  }
current_date();

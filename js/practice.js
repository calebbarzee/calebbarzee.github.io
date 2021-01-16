let options = {
  weekday: "long",
  year: "numeric",
  month: "long",
  day: "numeric",
};
date = new Date().toLocaleDateString("en-US", options);
document.getElementById("dateSpan").textContent = date;

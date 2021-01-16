// JavaScript Document

//function copyrightDate() {
let options = {
  weekday: "long",
  year: "numeric",
  month: "long",
  day: "numeric",
};
let copyrightYear = new Date.toLocaleDateString("en-US", options);
print(copyrightYear);
document.getElementById("copyrightYear").textContent = copyrightYear;
//}

//function lastUpdatedDate() {
let lastUpdated = Date(document.lastModified);
print(lastUpdated);
document.getElementById("updateDate").textContent = lastUpdated;
//}

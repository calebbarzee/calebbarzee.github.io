// JavaScript Document

//function copyrightDate() {
let options = {
  year: "numeric",
};
let copyrightYear = new Date().toLocaleDateString("en-US", options);
document.getElementById("copyrightYear").textContent = copyrightYear;
//}

//function lastUpdatedDate() {
let lastUpdated = Date(document.lastModified);
document.getElementById("updateDate").textContent = lastUpdated;
//}

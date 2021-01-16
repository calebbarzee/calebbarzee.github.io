// JavaScript Document

function copyrightDate() {
let options1 = {
  year: "numeric",
};
let copyrightYear = new Date().toLocaleDateString(undefined, options1);
document.getElementById("copyrightYear").textContent = copyrightYear;
}

function lastUpdatedDate() {
let options2 ={
    month: "short",
    day: "numeric",
    year: "numeric"
};
let lastUpdated = new Date(document.lastModified);
lastUpdated = lastUpdated.toLocaleDateString(undefined, options2);
document.getElementById("updateDate").textContent = lastUpdated;
}

$(document).ready(function () {

    ////////////////////////////////////
    // Constants & Shared
    ////////////////////////////////////
    const url = window.location.origin;
    currentApplications = [];

    ////////////////////////////////////
    // Utils
    ////////////////////////////////////

    // Set status in footer
    function setStatus(content) {
        $("#status").html(content);
    }

    // Check health/connection to backend, and set status in footer
    function checkServerStatus() {
        const xhr = new XMLHttpRequest();
        xhr.open("GET", url + "/health");
        xhr.send();
        xhr.onreadystatechange = function () {
            if (this.status !== 204) {
                setStatus("Failed to connect to backend!");
                return false;
            }
            else {
                setStatus("Successfully connected to backend!");
                return true;
            }
        }
    }

    // Get all applications, updates the contents of `currentApplications`
    function getAllApplications() {
        const xhr = new XMLHttpRequest();
        xhr.open("GET", url + "/applications");
        xhr.send();
        xhr.onreadystatechange = function () {
            if (String(this.status)[0] == "2" & this.readyState == XMLHttpRequest.DONE) {
                currentApplications = JSON.parse(this.response);
                updateList();
            }
        }
    }

    // Reads the contents of `currentApplications` and spits it out to the page.
    // This has a potential Injection security risk as we are setting html content in JS from backend values.
    function updateList() {
        html = "<ul>";
        html_end = "</ul>"
        for (const [i, element] of currentApplications.entries()) {
            html += `<li>${i + 1}. ${element["fullname"]}| <a href="${element["cv_link"]}">CV</a> <br>Submission Datetime: ${formatDatetime(element["creation_time"])}`
        };
        $("#app-data").html(html + html_end);
    }

    // Call a number of functions on startup
    function startUp() {
        console.log('JS Started!');
        checkServerStatus();
        getAllApplications();
    }

    startUp();

});

// Submit form button
function submitForm(oFormElement) {
    var xhr = new XMLHttpRequest();
    xhr.open(oFormElement.method, oFormElement.getAttribute("action"));
    xhr.send(new FormData(oFormElement));
    xhr.onreadystatechange = function () {
            if (String(this.status)[0] == "2" & this.readyState == XMLHttpRequest.DONE) {
                location.reload()
            }
        }
    return false;
}

// turn Epoch into formatted date
function formatDatetime(value) {
    
    let date = new Date(Math.round(Number(value*1000)));
    let min = ("0" + date.getMinutes()).slice(-2);
    let hour = ("0" + date.getHours()).slice(-2);
    let day = ("0" + date.getDate()).slice(-2);
    let month = ("0" + (date.getMonth() + 1)).slice(-2);
    let formatDate = hour + ":" + min + " " +(day)+"-"+(month)+"-"+ date.getFullYear();

    return formatDate;
}
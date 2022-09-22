$(document).ready(function(){

    ////////////////////////////////////
    // Constants
    ////////////////////////////////////
    const url = window.location.origin
    const searched = false;



    ////////////////////////////////////
    // Utils
    ////////////////////////////////////
    function setStatus(content) {
        $("#status").html(content);
    }
    
    function checkServerStatus() {
        const xhr = new XMLHttpRequest();
        xhr.open("GET", url+"/health");
        xhr.send();
        xhr.onreadystatechange = function(){
            if (this.status !== 204){
                setStatus("Failed to connect to backend!")
                return false;
            }
            else{
                setStatus("Successfully connected to backend!")
                return true;
            }
        }
    }

    function startUp() {
        checkServerStatus();
    }


    startUp();
});
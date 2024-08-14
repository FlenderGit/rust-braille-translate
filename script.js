import __wbg_init, { initSync, string_to_braille } from "./braille_translate.js";

await __wbg_init();

(function(){
    "use strict";

    /*
    document.getElementById("translate").onclick = function() {
        console.log("translate button clicked");
        const input = document.getElementById("input").value;
        const output = string_to_braille(input);
        document.getElementById("output").innerHTML = convert(output);
    }
    */

    document.getElementById("input").oninput = function() {
        const input = document.getElementById("input").value;
        const output = string_to_braille(input);
        document.getElementById("output").innerHTML = convert(output);
    }

    const convert = function(list) {
        let output = "";
        list.forEach(element => {
            output += String.fromCharCode(element + 0x2800);
        });
        return output;
    }
})();
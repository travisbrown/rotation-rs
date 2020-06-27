import * as wasm from "rotation";

document.getElementById("url_form").onsubmit = function() {
  let url = document.getElementById("image").value;
  document.getElementById("original").src = url;
  
  let compute = wasm.estimate_image_rotation(url);
  compute.then(
    function(result) {
      document.getElementById("original_label").innerHTML = "Original image";
      document.getElementById("rotated_label").innerHTML = "Rotated by " + -result.toFixed(4) + " degrees";
      let rotated = document.getElementById("rotated");
      rotated.src = url;
      rotated.style = 'transform: rotate(' + -result + 'deg)';
    },    
    function(error) { alert(error); }
  );
  return false;
  
}

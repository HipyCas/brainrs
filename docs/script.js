var bfSymbols = ["+", "-", ">", "<"];

var separators = document.querySelectorAll(".separator");
for (var i = 0; i < separators.length; i++) {
  var string = "";
  for (
    var j = 0;
    j < Math.floor(document.documentElement.clientWidth / 8.5);
    j++
  ) {
    string += bfSymbols[Math.floor(Math.random() * bfSymbols.length)];
  }
  separators[i].innerHTML = string;
}

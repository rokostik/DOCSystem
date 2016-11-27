function onFileSelect(event) {
    var fullPath = document.getElementById('upload').value;
  if (fullPath) {
      var startIndex = (fullPath.indexOf('\\') >= 0 ? fullPath.lastIndexOf('\\') : fullPath.lastIndexOf('/'));
      var filename = fullPath.substring(startIndex);
      if (filename.indexOf('\\') === 0 || filename.indexOf('/') === 0) {
          filename = filename.substring(1);
      }
      document.getElementById('fileName').value = filename;
  }
}

function addTag(event) {
    var tag = prompt("Please enter a new tag", "");
    if (tag != null && tag != "") {
        var tagLink = document.getElementById('addTag');

        var span = document.createElement('span');
            span.innerHTML = tag;
        tagLink.parentNode.insertBefore(span, tagLink);

    }
}

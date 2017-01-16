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

function newFile(event) {
    var main = document.getElementById('main');
    main.innerHTML =
    '<div class="main-top new-file-heading">'+
        '<h1><a href="dashboard_document">Vacation</a> / New file</h1>'+
    '</div>'+
    '<div class="new-file">'+
        '<form action="dashboard.html" enctype="multipart/form-data">'+
            '<label>Select file:<input type="file" name="file" accept="image/*" id="upload" onchange="onFileSelect(event);" required></label>'+
            '<label>Name:<input name="file_name" id="fileName" required/></label>'+
            '<label class="tags main-tags">Tags:<a href="#" onclick="addTag(event)" id="addTag"> +Add tag</a></label>'+
            '<label>I am not a robot <input type="checkbox" required/></label>'+
            '<label><input type="submit"/ value="Upload"></label>'+
        '</form>'+
    '</div >';
}

function addFolder(event) {
    var name = prompt("Please enter the folder name", "");
    if (name != null && name != "") {
        var folderLink = document.getElementById('addFolder');

        var listItem = document.createElement('li');
            listItem.innerHTML = '<a href="#">' + name + '</a>';
        folderLink.parentNode.parentNode.insertBefore(listItem, folderLink.parentNode);

    }
}

function shareFile(event) {
    var name = prompt("Please the person you want to share with", "");
    if (name != null && name != "") {
        var list = document.getElementById('sharedWith');

        var span = document.createElement('span');
            span.innerHTML = '<span>, ' + name + '</span>';
        list.lastChild.appendChild(span);

    }
}

function showNavigation() {
    var nav = document.getElementsByTagName("aside")[0];
    if(nav.style.display == "block"){
        nav.style.display = "none";
    }
    else{
        nav.style.display = "block";
        nav.style.minWidth = "100%";
        nav.style.minHeight = "100%";
    }
}

function logout() {
    window.location.pathname = "";
}
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

    var file = document.querySelector('input[type=file]').files[0];
    var reader = new FileReader();

    reader.addEventListener("load", function () {
      document.querySelector('input[type=hidden]').value = reader.result.split(",")[1];
    }, false);

    if (file) {
      reader.readAsDataURL(file);
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

function newFile(folder_name) {
    var main = document.getElementById('main');
    main.innerHTML =
    '<div class="main-top new-file-heading">'+
        '<h1><a href="/home/'+ folder_name +'">'+ folder_name +'</a> / New document</h1>'+
    '</div>'+
    '<div class="new-file">'+
        '<form method="post" action="/home/'+ folder_name +'">'+
            '<label>Select file:<input type="file" name="file" accept="image/*" id="upload" onchange="onFileSelect(event);" required></label>'+
            '<label>Name:<input name="file_name" id="fileName" required/></label>'+
            '<label class="tags main-tags">Tags:<a href="#" onclick="addTag(event)" id="addTag"> +Add tag</a></label>'+
            '<label>I am not a robot <input type="checkbox" required/></label>'+
            '<label><input type="submit"/ value="Upload"></label>'+
            '<input type="hidden" name="file_b64" id=base64file</input>'
        '</form>'+
    '</div >';
}

function addFolder(event) {
    var name = prompt("Please enter the folder name", "");
    if (name != null && name != "") {
        var form = document.createElement("form");
        form.setAttribute("method", "post");
        form.setAttribute("action", "/new");

        var hiddenField = document.createElement("input");
        hiddenField.setAttribute("type", "hidden");
        hiddenField.setAttribute("name", "folder_name");
        hiddenField.setAttribute("value", name);
        form.appendChild(hiddenField);

        document.body.appendChild(form);
        form.submit();
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
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

function loadJSON(filePath, callback) {
    var xhr = new XMLHttpRequest();
    xhr.open("GET", filePath, true);
    xhr.onreadystatechange = function () {
          if (xhr.readyState == 4 && xhr.status == "200") {
            callback(xhr.responseText);
          }
    };
    xhr.send(null);
}

function showFile(event) {
    var filePath = "json/file.json";
    loadJSON(filePath, function(response) {
        var json = JSON.parse(response);
        var main = document.getElementById('main');

        main.innerHTML = '<div class="main-top">' +
            '<h2><a onclick="showFolder()">'+ json.folder +'</a> / '+ json.name +'</h2>'+
            '<a href="images/'+ json.image +'" alt="download" download><img src="images/download.png" title="Download file" style="padding-top: 3px"/></a>' +
        '</div>';
        var tagData =
        '<div class="main-tags">';
        for(var i in json.tags){
            tagData += '<span>' + json.tags[i].tag + '</span>';
        }
        main.innerHTML += tagData +
        '</div>'+
        '<div class="main-image-container">' +
            '<img src="images/'+ json.image +'"alt="image"/>'+
        '</div>';
    });
}

function showFolder(event) {
    var mq = window.matchMedia("(max-width: 550px)");
    if(mq.matches){
        showNavigation();
    }

    var filePath = "json/file_list.json";
    loadJSON(filePath, function(response) {
        var json = JSON.parse(response);
        main = document.getElementById('main');

        main.innerHTML =
        '<div class="main-top">'+
            '<h1>Vacation</h1>'+
            '<a href="#"> <img src="images/share.png" title="Share file" onclick="shareFile()" /> </a>'+
        '</div>'+

        '<div class="main-shared-with">'+
            '<span>Shared with: </span>';

            for(var i in json.sharedWith){
                main.innerHTML += '<span>';
                if(i != 0){
                    main.innerHTML += ', '
                }
                main.innerHTML += json.sharedWith[i].name + '</span>';
            }

        var tData =
        '</div>'+

        '<div class="files">'+
            '<table>'+
                '<thead>'+
                    '<tr>'+
                        '<th>Name</td>'+
                        '<th width="1%">Changed</td>'+
                    '</tr>'+
                '</thead>'+
                '<tbody>';

                    for(var i in json.files){
                        tData += '<tr><td><a href="#" onclick="showFile()">'+ json.files[i].fileName +'</td>'
                        tData += '<td>'+ json.files[i].changed +'</td></tr>'
                    }
                tData +=
                    '<tr>'+
                        '<td><a onclick="newFile()"><img src="images/plus_file.png">Add a file</a></td>'+
                    '</tr>'+
                '</tbody>'+
            '</table>'+
        '</div>';

        main.innerHTML += tData;
    });
}

function newFile(event) {
    var main = document.getElementById('main');
    main.innerHTML =
    '<div class="main-top new-file-heading">'+
        '<h1><a href="dashboard_document">Vacation</a> / New file</h1>'+
    '</div>'+
    '<div class="new-file">'+
        '<form action="dashboard_files.html">'+
            '<label>Select file:<input type="file" name="file" id="upload" onchange="onFileSelect(event);" required></label>'+
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
        var folderLink = document.getElementById('addFolder');

        var listItem = document.createElement('li');
            listItem.innerHTML = '<a href="#">' + name + '</a>';
        folderLink.parentNode.parentNode.insertBefore(listItem, folderLink.parentNode);

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

document.addEventListener("DOMContentLoaded", function() {
    showFolder();
});

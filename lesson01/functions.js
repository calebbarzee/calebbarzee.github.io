/* This script edits a bulleted list */

function modify_list() {
    let new_li = document.querySelector('#new_li').value
    // .value grabs the actual value of an input box
    //.querySelector can be used for both #id and .class
    let create_li = document.createElement('li');
    //.createElement creates a new tag/element in DOM
    create_li.innerHTML = new_li;
    document.querySelector('#bulleted_list').appendChild(create_li);
}
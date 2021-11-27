const menu_button = document.querySelector('.menu_button');
const nav = document.querySelector('.navigation')


menu_button.addEventListener('click', add_show_list, false);

window.onresize = function() {
	if(window.innerWidth > 760) 
		nav.classList.remove('responsive')};
		
function add_show_list() {
	// console.log("add_show_list success")
	nav.classList.toggle('show_list')
}



menu_button.addEventListener('mouseover' ,change_to_black, false);

menu_button.addEventListener('mouseout' ,change_to_white, false);

function change_to_black() {
	menu_button.removeAttribute("src", "assets/hamburger_menu.png");
	menu_button.setAttribute("src", "assets/hamburger_menu_black-01.png");
}

function change_to_white() {
	menu_button.removeAttribute("src", "assets/hamburger_menu_black-01.png");
	menu_button.setAttribute("src", "assets/hamburger_menu.png");
}
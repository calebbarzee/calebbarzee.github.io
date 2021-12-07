const menu_button = document.querySelector('.menu_button');
const nav = document.querySelector('.navigation');
const header= document.querySelector('header');

menu_button.addEventListener('click', add_responsive, false);

function add_responsive() {
	// console.log("add_responsive success")
	nav.classList.toggle('responsive');
	header.classList.toggle('extend_menu');
}


menu_button.addEventListener('mouseover' ,change_to_black, false);

menu_button.addEventListener('mouseout' ,change_to_white, false);

function change_to_black() {
	menu_button.removeAttribute("src", "assets/menu_icon/hamburger_menu.png");
	menu_button.setAttribute("src", "assets/menu_icon/hamburger_menu_black-01.png");
}

function change_to_white() {
	menu_button.removeAttribute("src", "assets/menu_icon/hamburger_menu_black-01.png");
	menu_button.setAttribute("src", "assets/menu_icon/hamburger_menu.png");
}
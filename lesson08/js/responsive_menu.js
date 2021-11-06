const menu_button = document.querySelector('.menu_button');
const mainnav = document.querySelector('.navigation')


menu_button.addEventListener('click',function() {
		mainnav.classList.toggle('responsive')
		},false);

window.onresize = function() {
	if(window.innerWidth > 760) 
		mainnav.classList.remove('responsive')};
		
menu_button.addEventListener('mouseover', change_to_black ,false);

menu_button.addEventListener('mouseout', change_to_white ,false);

function change_to_black() {
	menu_button.removeAttribute("src", "assets/hamburger_menu.png");
	menu_button.setAttribute("src", "assets/hamburger_menu_black-01.png");
}

function change_to_white() {
	menu_button.removeAttribute("src", "assets/hamburger_menu_black-01.png");
	menu_button.setAttribute("src", "assets/hamburger_menu.png");
}
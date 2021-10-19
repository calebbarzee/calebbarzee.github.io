const menu_button = document.querySelector('.menu_button');
const mainnav = document.querySelector('.navigation')

menu_button.addEventListener('click',function() {
		mainnav.classList.toggle('responsive')
		},false);

menu_button.addEventListener('mouseover', function( event ) {
	event.target.src = "assests/hamburger_menu_black-01.png";
setTimeout(function() {
		event.target.src = "assests/hamburger_menu.png";
	  }, 500);
	}, false);

window.onresize = function() {
	if(window.innerWidth > 760) 
		mainnav.classList.remove('responsive')};


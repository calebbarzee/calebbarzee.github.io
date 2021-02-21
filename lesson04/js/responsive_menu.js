const menu_button = document.querySelector('.menu_button');
const mainnav = document.querySelector('.navigation')

menu_button.addEventListener('click',function() {
		mainnav.classList.toggle('responsive')
		},false);

window.onresize = function() {
	if(window.innerWidth > 760) 		mainnav.classList.remove('responsive')};

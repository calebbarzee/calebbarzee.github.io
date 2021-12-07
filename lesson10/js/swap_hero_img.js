const hero_img = document.getElementById('hero_img');

// window.innerHeight - the inner height of the browser window (in pixels)
// window.innerWidth - the inner width of the browser window (in pixels)

window.addEventListener("resize", set_src_hero_img);
function set_src_hero_img() {

    if (window.innerWidth > 1200) {
        hero_img.setAttribute('src', 'assets/hero_img/preston_hero_img_lrg.png')
    }
    else if (window.innerWidth > 800) {
        hero_img.setAttribute('src', 'assets/hero_img/preston_hero_img_med.png')
    }
    else {
        hero_img.setAttribute('src', 'assets/hero_img/preston_hero_img_sm.png')
    }
}
set_src_hero_img()
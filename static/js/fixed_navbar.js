document.onscroll = _ => {
  let banner_height = parseInt(getComputedStyle(document.getElementById("banner")).height);
  let navbar = document.querySelector("nav");
  let navbar_height = getComputedStyle(navbar).height;
  let scroll_top = (window.pageYOffset !== undefined) ? window.pageYOffset : (document.documentElement || document.body.parentNode || document.body).scrollTop;
  let main = document.querySelector("main");

  if (scroll_top > banner_height) {
    navbar.classList.add("navbar-fixed");
    main.style.marginTop = navbar_height;
  } else {
    navbar.classList.remove("navbar-fixed");
    main.style.marginTop = `0px`;
  }
};

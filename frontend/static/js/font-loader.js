// Defer loading of fonts to improve performance
window.addEventListener('load', function() {
  const fontLink = document.getElementById('google-fonts');
  if (fontLink) {
    fontLink.media = 'all';
  }
});

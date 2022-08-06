const set_html = (html) => {
  document.querySelector('#content').innerHTML = html
}


const set_css = (css) => {
  const style_element = document.createElement('style')
  document.head.appendChild(style_element)
  for (let x of css.split('}')) {
      try {
        style_element.sheet.insertRule(x+'}');
      } catch(e) {

      }
  }
}


export {
  set_html,
  set_css,
}

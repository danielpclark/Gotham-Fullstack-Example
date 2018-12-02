import Vue from 'vue/dist/vue.esm'
import App from '../app.vue'

document.addEventListener('DOMContentLoaded', ->
  element = document.getElementById 'vue-app'

  if element?
    app = new Vue(
      el: element
      render: (h) -> h App
    )

  # Vue.config.devtools = true
)

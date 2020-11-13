class Toaster {
  constructor() {
    this.callbacks = [];
  }

  on(fn) {
    this.callbacks.push(fn);
  }

  emit(title, data) {
    for (let fn of this.callbacks) {
      fn(title, data);
    }
  }
}

export default {
  install: (Vue, options) => {
    Vue.prototype.$toaster = new Toaster();
    Vue.prototype.$toast = (...args) => Vue.prototype.$toaster.emit(...args);
  },
};
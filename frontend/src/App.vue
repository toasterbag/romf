<template>
  <main class="app">
    <router-view :key="$route.fullPath" />
    <img class="grill" :src="`${publicPath}${grill}`" />
  </main>
</template>

<script>
export default {
  name: "App",
  data: () => ({
    grill: "",
    publicPath: process.env.BASE_URL
  }),
  created() {
    this.grill = this.random_grill().replace("/", "");
  }
};
</script>

<style lang="scss">
//@import url("https://fonts.googleapis.com/css2?family=Raleway:wght@300&display=swap");

.grill {
  width: 20%;
  position: absolute;
  right: 10%;
  bottom: 0;
}

html {
  background: url("assets/memphis-colorful.png");
}

body {
  font-family: "Lato", sans-serif;
  font-weight: 300;
  color: #4a4a4a;

  margin: 10% 30%;
}

h1 {
  font-size: 2rem;
}

.caption {
  margin-top: -1rem;
  font-size: 1.4rem;
}

a {
  text-decoration: none;
  font-weight: 700;
  font-size: 16px;
}

.text-center {
  text-align: center;
}

.text-right {
  text-align: right;
}

.text-left {
  text-align: left;
}

.fill {
  width: 100%;
  height: 100%;
}

.clickable {
  cursor: pointer;
}

*::selection {
  background: var(--primary-color);
  color: #fff;
}

.fixed {
  position: fixed;
}

.center {
  position: absolute;
  top: 50%;
  left: 50%;
  transform: translate(-50%, -50%);
}

.left {
  float: left;
}
.right {
  float: right;
}

.relative {
  position: relative;
}

.hidden {
  transition: opacity 0.3s ease;
  opacity: 0;
}

.visible-on-hover:hover {
  opacity: 1;
}

.inline-block {
  display: inline-block;
}
</style>

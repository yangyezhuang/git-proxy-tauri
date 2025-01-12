<template>
  <img id="logo" alt="Wails logo" src="./assets/images/logo-universal.png" />
  <Home />
</template>

<script setup>
import Home from "./components/Home.vue";
</script>

<style>
#app {
    /* height: 100vh; */
    text-align: center;
}

#logo {
  display: block;
  width: 10%;
  height: 10%;
  margin: auto;
  padding: 10% 0 0;
  background-position: center;
  background-repeat: no-repeat;
  background-size: 100% 100%;
  background-origin: content-box;
}
</style>

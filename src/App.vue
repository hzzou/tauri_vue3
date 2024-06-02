<template>
  <!--  组件最顶层使用异步代码需要Suspense包裹-->
  <Suspense>
    <Music />
  </Suspense>
</template>
<script setup lang="ts">
    import Music from "./components/Music.vue";
</script>

<style scoped>

</style>

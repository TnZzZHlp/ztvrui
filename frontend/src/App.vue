<script setup lang="ts">
import { RouterView } from 'vue-router'
import Sonner from './components/ui/sonner/Sonner.vue';
import 'vue-sonner/style.css'
</script>

<template>
  <router-view v-slot="{ Component }">
    <transition name="fade" mode="out-in">
      <component :is="Component" />
    </transition>
  </router-view>

  <Sonner position="top-center" :rich-colors="true" />
</template>

<style>
.fade-enter-active,
.fade-leave-active {
  transition: opacity 0.15s;
}

.fade-enter-from,
.fade-leave-to {
  opacity: 0;
}

.fade-enter-to,
.fade-leave-from {
  opacity: 1;
}
</style>
